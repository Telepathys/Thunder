use std::{
    collections::HashMap,
    net::SocketAddr,
};
use futures_channel::mpsc::{unbounded};
use futures_util::SinkExt;
use futures_util::stream::SplitSink;
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use tokio::net::{TcpStream};
use log::{info,error};
use crate::persistence::users::user_data::{add_user_socket, delete_user_socket, get_user_socket};
use tokio_tungstenite::{tungstenite::handshake::client::Request,tungstenite::Message};
use tokio_tungstenite::tungstenite::Error as TungsteniteError;
use tokio_tungstenite::WebSocketStream;
use crate::utils::jwt::{
    verify_token,
};


pub async fn handle_connection(stream: TcpStream, socket: SocketAddr) {
    info!("Incoming TCP connection from: {}", socket);
    let mut uri = None;
    let ws_stream: tokio_tungstenite::WebSocketStream<TcpStream> = tokio_tungstenite::accept_hdr_async(stream, |req: &Request, res| {
        uri = Some(req.uri().clone());
        Ok(res)
    })
    .await
    .expect("Error during the websocket handshake occurred");
    let (outgoing, incoming) = ws_stream.split();

    let mut query_map = HashMap::new();
    if let Some(query) = uri.unwrap().query() {
        for q in query.split('&') {
            let mut q_iter = q.split('=');
            if let Some(key) = q_iter.next() {
                if let Some(value) = q_iter.next() {
                    query_map.insert(key.to_string(), value.to_string());
                }
            }
        }
    }
    
    if let Some(token) = query_map.get("token") {
        println!("토큰: {}", token);
        let result = verify_token(token);
        match result {
            Ok(value) => {
                let uuid = value.get("uuid");
                let id = value.get("id");
                let name = value.get("name");
                println!("uuid: {}", uuid.unwrap());
                println!("id: {}", id.unwrap());
                println!("name: {}", name.unwrap());
                let (_tx, rx) = unbounded();
                
                add_user_socket(socket,_tx);
            
                let broadcast_incoming = incoming.try_for_each(|msg| {
                    // tokio::spawn(async move {
                    //     router(msg,socket).await;
                    // });
            
                    // We want to broadcast the message to everyone except ourselves.
                    let sockets = get_user_socket();
                    let broadcast_recipients =
                    sockets.iter().filter(|(peer_addr, _)| peer_addr != &&socket).map(|(_, ws_sink)| ws_sink);
            
                    for recp in broadcast_recipients {
                        recp.unbounded_send(msg.clone()).unwrap();
                    }
            
                    future::ok(())
                });
            
                let receive_from_others = rx.map(Ok).forward(outgoing);
            
                pin_mut!(broadcast_incoming, receive_from_others);
                future::select(broadcast_incoming, receive_from_others).await;
            
                delete_user_socket(socket)
            }
            Err(error) => {
                error!("{}", error);
                tokio::spawn(async move {
                    // 에러 코드를 전송하는 함수를 호출합니다.
                    let mut ws_sink = outgoing.sink_map_err(|err| {
                        std::io::Error::new(std::io::ErrorKind::Other, err)
                    }).into_inner();
                    if let Err(err) = send_error_message(&mut ws_sink, 500).await {
                        error!("Failed to send error message to client: {}", err);
                    }
                });
            }
        }
    } else {
        println!("토큰이 제공되지 않았습니다.");
    }
}

async fn send_error_message(ws_sink: &mut SplitSink<WebSocketStream<TcpStream>, Message>, error_code: u32) -> Result<(), TungsteniteError> {
    let error_message = format!("Error occurred with code: {}", error_code);
    ws_sink.send(Message::Text(error_message)).await
}
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use hyper::Uri;
use tokio::net::{TcpStream};
use log::{info};
use crate::router::socket_router::router;
use crate::persistence::users::user_data::{add_user_socket, delete_user_socket, get_user_socket};
use tokio_tungstenite::{accept_hdr_async, tungstenite::handshake::client::Request};


pub async fn handle_connection(stream: TcpStream, socket: SocketAddr) {
    
    info!("Incoming TCP connection from: {}", socket);
    let mut uri = None;
    let ws_stream: tokio_tungstenite::WebSocketStream<TcpStream> = tokio_tungstenite::accept_hdr_async(stream, |req: &Request, res| {
        uri = Some(req.uri().clone());
        Ok(res)
    })
    .await
    .expect("Error during the websocket handshake occurred");

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
        let (_tx, rx) = unbounded();
    
        // total_socket.lock().unwrap().insert(socket, tx);
        add_user_socket(socket,_tx);
        let (outgoing, incoming) = ws_stream.split();
    
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
        // total_socket.lock().unwrap().remove(&socket);
    } else {
        println!("토큰이 제공되지 않았습니다.");
    }
}
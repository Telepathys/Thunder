use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use tokio::net::{TcpStream};
use log::{info};
use tokio_tungstenite::tungstenite::protocol::Message;
pub mod router;
use router::{
    router,
};

pub type Tx = UnboundedSender<Message>;
pub type TotalSocket = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

pub async fn handle_connection(total_socket: TotalSocket, raw_stream: TcpStream, socket: SocketAddr) {
    info!("Incoming TCP connection from: {}", socket);
    let ws_stream: tokio_tungstenite::WebSocketStream<TcpStream> = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");

    let (_tx, rx) = unbounded();
    
    // total_socket.lock().unwrap().insert(socket, tx);

    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg| {
        tokio::spawn(async move {
            router(msg,socket).await;
        });

        // We want to broadcast the message to everyone except ourselves.
        // let sockets = total_socket.lock().unwrap();
        // let broadcast_recipients =
        // sockets.iter().filter(|(peer_addr, _)| peer_addr != &&socket).map(|(_, ws_sink)| ws_sink);

        // for recp in broadcast_recipients {
        //     recp.unbounded_send(msg.clone()).unwrap();
        // }

        future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    info!("{} disconnected", &socket);
    total_socket.lock().unwrap().remove(&socket);
}
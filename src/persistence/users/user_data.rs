use std::sync::{Mutex, MutexGuard};
use std::{
    collections::HashMap,
    net::SocketAddr,
};
use lazy_static::lazy_static;
use futures_channel::mpsc::{UnboundedSender};
use tokio_tungstenite::tungstenite::protocol::Message;
use log::{info};

lazy_static! {
    static ref TOTAL_SOCKET: Mutex<HashMap<SocketAddr, UnboundedSender<Message>>> = Mutex::new(HashMap::new());
}

pub fn add_user_socket(socket: SocketAddr, tx: UnboundedSender<Message>) {
    info!("{} connected", &socket);
    TOTAL_SOCKET.lock().unwrap().insert(socket, tx);
}

pub fn delete_user_socket(socket: SocketAddr) {
    info!("{} disconnected", &socket);
    TOTAL_SOCKET.lock().unwrap().remove(&socket);
}

pub fn get_user_socket() -> MutexGuard<'static, HashMap<SocketAddr, UnboundedSender<Message>>> {
    TOTAL_SOCKET.lock().unwrap()
}
use std::{
    net::SocketAddr,
};
use tokio_tungstenite::tungstenite::protocol::Message;
use log::{info};
use serde_json::{Value};

pub async fn router(msg: Message, socket: SocketAddr) {
    let msg = msg.to_text().unwrap();
    let value: Value = serde_json::from_str(msg).unwrap();
    if let Some(data) = value.as_object().unwrap().iter().next() {
        let router = data.0;
        match router.as_str() {
            "caht" => {
                info!("login : {}", data.0);
            }
            _ => {
                info!("default : {}", data.0);
            }
        }
    } else {
        
    }
}
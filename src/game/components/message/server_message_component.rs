use serde::{Deserialize,Serialize};

#[derive(Debug, Deserialize)]
pub struct ServerMessage {
    pub server_message_send: ServerMessageSend,
}

#[derive(Debug, Deserialize)]
pub struct ServerMessageSend {
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerMessageSendTo {
    pub uid: String,
    pub username: String,
    pub message: String,
}
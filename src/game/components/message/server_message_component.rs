use serde::{Deserialize,Serialize};
use crate::game::enums::core_enum::{
    MessageType
};

#[derive(Debug, Deserialize)]
pub struct ServerMessage {
    pub server_message_send: ServerMessageSend,
}

#[derive(Debug, Deserialize)]
pub struct ServerMessageSend {
    pub message: String,
}

#[derive(Deserialize, Serialize)]
pub struct ServerMessageSendTo {
    pub message_type: MessageType,
    pub uid: String,
    pub username: String,
    pub message: String,
}
use serde::{Deserialize,Serialize};

use crate::game::enums::core_enum::MessageType;

#[derive(Debug, Deserialize)]
pub struct GroupMessage {
    pub group_message_send: GroupMessageSend,
}

#[derive(Debug, Deserialize)]
pub struct GroupMessageSend {
    pub message: String,
}

#[derive(Deserialize, Serialize)]
pub struct GroupMessageSendTo {
    pub message_type: MessageType,
    pub uid: String,
    pub username: String,
    pub message: String,
}
use serde::{Deserialize,Serialize};

use crate::game::enums::core_enum::MessageType;

#[derive(Debug, Deserialize)]
pub struct RandomMatchMessage {
    pub random_match_message_send: RandomMatchMessageSend,
}

#[derive(Debug, Deserialize)]
pub struct RandomMatchMessageSend {
    pub message: String,
}

#[derive(Deserialize, Serialize)]
pub struct RandomMatchMessageSendTo {
    pub message_type: MessageType,
    pub uid: String,
    pub username: String,
    pub message: String,
}
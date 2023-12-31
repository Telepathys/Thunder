use serde::{Deserialize,Serialize};

use crate::game::enums::core_enum::MessageType;

#[derive(Debug, Deserialize)]
pub struct WhisperMessage {
    pub whisper_message_send: WhisperMessageSend,
}

#[derive(Debug, Deserialize)]
pub struct WhisperMessageSend {
    pub uid: String,
    pub message: String,
}

#[derive(Deserialize, Serialize)]
pub struct WhisperMessageSendTo {
    pub message_type: MessageType,
    pub uid: String,
    pub username: String,
    pub message: String,
}
use serde::{Deserialize,Serialize};

#[derive(Debug, Deserialize)]
pub struct WhisperMessage {
    pub whisper_message_send: WhisperMessageSend,
}

#[derive(Debug, Deserialize)]
pub struct WhisperMessageSend {
    pub uid: String,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WhisperMessageSendTo {
    pub uid: String,
    pub username: String,
    pub message: String,
}
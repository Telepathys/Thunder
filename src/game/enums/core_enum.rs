use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum MessageType {
    System,
    ServerMessage,
    WhisperMessage,
    GroupJoin,
    GroupLeave,
    GroupMessage,
    RamdomMatchWaitJoin,
    RandomMatchJoin,
    RandomMatchMessage,
}
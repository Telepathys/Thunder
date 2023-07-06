use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum MessageType {
    System,
    ServerMessage,
    WhisperMessage,
    RandomMatchMessage,
    GroupJoin,
    GroupLeave,
    GroupMessage,
    RamdomMatchWaitJoin,
    RandomMatchJoin,
    RandomMatchLeave,
}
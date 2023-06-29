use serde::{Deserialize,Serialize};
use crate::game::enums::core_enum::{
    MessageType
};

#[derive(Deserialize, Serialize)]
pub struct SystemMessageSendTo {
    pub message_type: MessageType,
    pub message: String,
}
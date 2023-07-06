use serde::{Deserialize,Serialize};
use crate::game::enums::core_enum::{
    MessageType
};


#[derive(Deserialize, Serialize)]
pub struct RandomMatchLeaveSendTo {
    pub message_type: MessageType,
    pub match_id: String,
    pub uid: String,
    pub username: String,
    pub message: String,
}
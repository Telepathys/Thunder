use serde::{Deserialize,Serialize};

use crate::game::enums::core_enum::MessageType;

#[derive(Deserialize, Serialize)]
pub struct RandomMatchWaitJoinSendTo {
    pub message_type: MessageType,
    pub match_id: String,
}
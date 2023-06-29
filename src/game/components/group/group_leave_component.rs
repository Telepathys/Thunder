use serde::{Deserialize,Serialize};
use crate::game::enums::core_enum::{
    MessageType
};

#[derive(Debug, Deserialize)]
pub struct GroupLeave {
    pub group_leave: GroupLeaveData,
}

#[derive(Debug, Deserialize)]
pub struct GroupLeaveData {
    pub group_key: String,
}

#[derive(Deserialize, Serialize)]
pub struct ResponseGroupLeave {
    pub message_type: MessageType,
    pub group_key: String,
    pub uid: String,
    pub username: String,
    pub message: String,
}
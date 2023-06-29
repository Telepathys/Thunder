use serde::{Deserialize,Serialize};
use crate::game::enums::core_enum::{
    MessageType
};


#[derive(Debug, Deserialize)]
pub struct GroupJoin {
    pub group_join: GroupJoinData,
}

#[derive(Debug, Deserialize)]
pub struct GroupJoinData {
    pub group_name: String,
}

#[derive(Deserialize, Serialize)]
pub struct ResponseGroupJoin {
    pub message_type: MessageType,
    pub group_key: String,
    pub uid: String,
    pub username: String,
    pub message: String,
}
use serde::{Deserialize,Serialize};
use crate::game::enums::core_enum::{
    MessageType
};


#[derive(Debug, Deserialize)]
pub struct RandomMatchJoin {
    pub random_match_join: RandomMatchJoinData,
}

#[derive(Debug, Deserialize)]
pub struct RandomMatchJoinData {
    pub match_id: String,
    pub accept: bool,
}


#[derive(Deserialize, Serialize)]
pub struct RandomMatchJoinSendTo {
    pub message_type: MessageType,
    pub match_id: String,
    pub uid: String,
    pub username: String,
    pub message: String,
}
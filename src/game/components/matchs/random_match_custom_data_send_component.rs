use serde::{Deserialize,Serialize};

use crate::game::enums::core_enum::{CustomDataType};

#[derive(Deserialize)]
pub struct RandomMatchCustomData {
    pub random_match_custom_data_send: RandomMatchCustomDataSend,
}

#[derive(Deserialize)]
pub struct RandomMatchCustomDataSend {
    pub data_type: CustomDataType,
    pub data: String,
}

#[derive(Deserialize, Serialize)]
pub struct RandomMatchCustomDataSendTo {
    pub data_type: CustomDataType,
    pub uid: String,
    pub username: String,
    pub data: String,
}
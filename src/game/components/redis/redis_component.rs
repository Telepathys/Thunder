use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct RedisPublish {
    pub uid: String,
    pub msg: String,
}

#[derive(Serialize, Deserialize)]
pub struct RedisSubscribe {
    pub uid: String,
    pub msg: String,
}
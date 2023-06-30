use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub message_limit_second: Option<u32>,
    pub message_limit_count: Option<u32>,
    pub message_ban_second: Option<u32>,
}
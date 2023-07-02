use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub message_limit_second: Option<u32>,
    pub message_limit_count: Option<u32>,
    pub message_ban_second: Option<u32>,
    pub match_check_time: Option<u32>,
    pub match_require_user_count: Option<u32>,
    pub match_make_count_control: Option<f64>,
    pub match_join_limit_time: Option<u32>,
    
}
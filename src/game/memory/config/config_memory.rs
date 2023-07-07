use std::fs;
use std::sync::{Mutex, MutexGuard};
use lazy_static::lazy_static;

use crate::game::components::config::config_component::Config;

lazy_static! {
    static ref CONFIG: Mutex<Config> = Mutex::new(Config {
        message_limit_second: Some(5),
        message_limit_count: Some(5),
        message_ban_second: Some(30),
        match_check_time: Some(1),
        match_require_user_count: Some(2),
        match_make_count_control: Some(0.1),
        match_join_limit_time: Some(10),
    });
}

pub async fn config_init() {
    let contents = fs::read_to_string("Config.yaml").expect("Failed to read file");
    let config: Config = serde_yaml::from_str(&contents).expect("Failed to parse YAML");
    CONFIG.lock().unwrap().message_limit_second = config.message_limit_second;
    CONFIG.lock().unwrap().message_limit_count = config.message_limit_count;
    CONFIG.lock().unwrap().message_ban_second = config.message_ban_second;
    CONFIG.lock().unwrap().match_check_time = config.match_check_time;
    CONFIG.lock().unwrap().match_require_user_count = config.match_require_user_count;
    CONFIG.lock().unwrap().match_make_count_control = config.match_make_count_control;
    CONFIG.lock().unwrap().match_join_limit_time = config.match_join_limit_time;
}

pub fn get_config() -> MutexGuard<'static, Config> {
    CONFIG.lock().unwrap()
}
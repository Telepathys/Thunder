use std::fs;

use crate::game::components::config::config_component::Config;

use super::matchs::random_match_scheduler::random_match_scheduler;



pub async fn scheduler_core() {
    let contents = fs::read_to_string("Config.yaml").expect("Failed to read file");
    let config: Config = serde_yaml::from_str(&contents).expect("Failed to parse YAML");
    
    tokio::spawn(async move {
        random_match_scheduler(tokio::sync::Mutex::new(()),config).await;
    });
}
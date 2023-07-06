use tokio_tungstenite::tungstenite::protocol::Message;
use log::{info};
use serde_json::{Value};

use crate::{router::service::{
    message_router:: {
        message_router
    },
    group_router:: {
        group_router
    },
    match_router:: {
        match_router
    }
}, game::systems::message::message_limit_system::message_limit_check};

pub fn socket_router(uid: String, msg: Message) {
    let msg = msg.to_text().unwrap();
    let value: Value = serde_json::from_str(msg).unwrap();
    if let Some(data) = value.as_object().unwrap().iter().next() {
        let router = data.0;
        match router.as_str() {
            "server_message_send" | 
            "whisper_message_send" | 
            "group_message_send" |
            "random_match_message_send"
            => {
                if message_limit_check(&uid) {
                    message_router(uid,router.as_str(), msg.into());
                }
            }
            "group_join" | 
            "group_leave"
            => {
                group_router(uid,router.as_str(), msg.into());
            }
            "random_match_wait" | 
            "random_match_cancel"|
            "random_match_join" |
            "random_match_wait_success" |
            "random_match_complete"  |
            "random_match_leave"
            => {
                match_router(uid,router.as_str(), msg.into());
            }
            _ => {
                info!("default : {}", data.0);
            }
        }
    }
}
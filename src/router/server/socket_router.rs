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
            "group_message_send"
            => {
                if message_limit_check(uid.clone()) {
                    message_router(uid,router.as_str(), msg.clone().into());
                }
            }
            "group_join" | 
            "group_leave" => {
                group_router(uid,router.as_str(), msg.clone().into());
            }
            "random_match_wait" | 
            "random_match_cancel" => {
                match_router(uid,router.as_str(), msg.clone().into());
            }
            _ => {
                info!("default : {}", data.0);
            }
        }
    }
}
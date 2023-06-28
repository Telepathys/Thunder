use tokio_tungstenite::tungstenite::protocol::Message;

use crate::game::systems::message::{
    whisper_message_system::{
        whisper_message_send
    },
    server_message_system::{
        server_message_send
    }
};

pub async fn message_router(send_uid: String, service: &str, msg: Message) {
    match service {
        "server_message_send" => {
            server_message_send(send_uid, msg).await;
        }
        "whisper_message_send" => {
            whisper_message_send(send_uid, msg).await;
        }
        _ => {
        }
    }
}
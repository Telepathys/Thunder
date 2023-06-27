use log::info;
use tokio_tungstenite::tungstenite::protocol::Message;

use crate::persistence::users::user_data::get_user_socket;
use crate::database::redis::socket::socket_hash::{
    get_my_info
};
use crate::structs::message_struct::{
    WhisperMessage,
    WhisperMessageSendTo,
};

pub async fn message_router(send_uid: String, core: &str, msg: Message) {
    let msg = msg.to_text().unwrap();
    match core {
        "server_message_send" => {
            let sockets = get_user_socket();
            let user_sockets = sockets.iter().map(|(_, user_socket)| user_socket);
            let broadcast_recipients = user_sockets.map(|user_socket| user_socket.tx.clone());
            // let broadcast_recipients = user_sockets.filter(|user_socket| user_socket.socket != socket).map(|user_socket| user_socket.tx.clone());

            for recp in broadcast_recipients {
                recp.unbounded_send(msg.clone().into()).unwrap();
            }
        }
        "whisper_message_send" => {
            let data: WhisperMessage = serde_json::from_str(msg).unwrap();
            let uid = data.whisper_message_send.uid.clone();
            let message = data.whisper_message_send.message.clone();
            let sender_info = get_my_info(&send_uid).await.unwrap();
            let username = sender_info.iter().find(|(key, _)| *key == "name").map(|(_, value)| value.to_owned()).unwrap();

            let whisper_message_send_to = WhisperMessageSendTo {
                uid: send_uid.clone(),
                username: username,
                message: message.clone(),
            };

            let sockets = get_user_socket();
            let user_sockets = sockets.iter().filter(|user_sockets| user_sockets.0 == &uid).map(|(_, user_socket)| user_socket);
            let broadcast_recipients = user_sockets.map(|user_socket| user_socket.tx.clone());

            for recp in broadcast_recipients {
                recp.unbounded_send(Message::Text(serde_json::to_string(&whisper_message_send_to).unwrap())).unwrap();
            }

            
            
        }
        _ => {
        }
    }
}
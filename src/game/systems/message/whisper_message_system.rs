use std::collections::HashMap;
use futures_channel::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;
use crate::database::redis::socket::socket_hash::get_my_info;
use crate::game::components::message::whisper_message_component::{
    WhisperMessage,
    WhisperMessageSendTo,
};
use crate::game::enums::core_enum::MessageType;
use crate::game::memory::user::user_memory::get_user_socket;
use std::sync::{Arc,Mutex};

struct WhisperMessageEcsEngine {
    target_users: Arc<Mutex<HashMap<String, UnboundedSender<Message>>>>,
}

impl WhisperMessageEcsEngine {
    fn new() -> Self {
        WhisperMessageEcsEngine {
            target_users: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn add_user(&self, uid: String, recp: UnboundedSender<Message>) {
        self.target_users.lock().unwrap().insert(uid, recp);
    }

    fn clear(&self) {
        self.target_users.lock().unwrap().clear();
    }

    fn broadcast_message(&self, message: Message) {
        for recp in self.target_users.lock().unwrap().values() {
            let _ = recp.unbounded_send(message.clone()).unwrap();
        }
    }
}

pub fn whisper_message_send(
    send_uid: String,
    msg : Message,
) {
    let whisper_message_ecs_engine = Arc::new(WhisperMessageEcsEngine::new());
    let msg = msg.to_text().unwrap();
    let data: WhisperMessage = serde_json::from_str(msg).unwrap();
    let uid = data.whisper_message_send.uid.clone();
    let message = data.whisper_message_send.message.clone();
    let sender_info = get_my_info(&send_uid).unwrap();
    let username = sender_info.iter().find(|(key, _)| *key == "name").map(|(_, value)| value.to_owned()).unwrap();
    let whisper_message_send_to = WhisperMessageSendTo {
        message_type: MessageType::WhisperMessage,
        uid: send_uid.clone(),
        username: username,
        message: message.clone(),
    };
    let sockets = get_user_socket();
    let user_sockets = sockets.iter().filter(|user_sockets| user_sockets.0 == &uid).map(|(_, user_socket)| user_socket);
    let broadcast_recipients = user_sockets.map(|user_socket| user_socket.tx.clone());

    for recp in broadcast_recipients {
        whisper_message_ecs_engine.add_user(uid.clone(), recp);
    }

    whisper_message_ecs_engine.broadcast_message(Message::Text(serde_json::to_string(&whisper_message_send_to).unwrap()));

    whisper_message_ecs_engine.clear();
}
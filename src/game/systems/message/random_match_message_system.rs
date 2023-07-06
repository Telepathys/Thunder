use std::collections::HashMap;
use futures_channel::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;
use crate::database::redis::matchs::match_hash::{check_my_match_exist, get_my_match, get_match_members};
use crate::database::redis::socket::socket_hash::get_my_info;
use crate::game::components::message::random_match_message_component::{RandomMatchMessage, RandomMatchMessageSendTo};
use crate::game::enums::core_enum::MessageType;
use crate::game::memory::user::user_memory::get_user_socket;
use std::sync::{Arc,Mutex};

use super::system_message_system::system_message_send;

struct RandomMatchMessageEcsEngine {
    target_users: Arc<Mutex<HashMap<String, UnboundedSender<Message>>>>,
}

impl RandomMatchMessageEcsEngine {
    fn new() -> Self {
        RandomMatchMessageEcsEngine {
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

pub fn random_match_message_send(
    send_uid: String,
    msg : Message,
) {
    if !check_my_match_exist(&send_uid).unwrap() {
        system_message_send(&send_uid, format!("you are not in random_match."));
        return;
    }

    let random_match_message_ecs_engine = Arc::new(RandomMatchMessageEcsEngine::new());
    let msg = msg.to_text().unwrap();
    let data: RandomMatchMessage = serde_json::from_str(msg).unwrap();
    let message = data.random_match_message_send.message;
    let sender_info = get_my_info(&send_uid).unwrap();
    let username = sender_info.iter().find(|(key, _)| *key == "name").map(|(_, value)| value.to_owned()).unwrap();
    let random_match_message_send_to = RandomMatchMessageSendTo {
        message_type: MessageType::RandomMatchMessage,
        uid: send_uid.clone(),
        username: username,
        message: message,
    };

    let match_id = get_my_match(&send_uid).unwrap();
    let match_user_list = get_match_members(&match_id).unwrap();

    let sockets = get_user_socket();
    let user_sockets = sockets.iter().filter(|user_sockets| match_user_list.contains(user_sockets.0)).map(|(_, user_socket)| user_socket);

    for this_socket in user_sockets {
        random_match_message_ecs_engine.add_user(this_socket.id.clone(), this_socket.tx.clone());
    }

    random_match_message_ecs_engine.broadcast_message(Message::Text(serde_json::to_string(&random_match_message_send_to).unwrap()));

    random_match_message_ecs_engine.clear();
}
use std::collections::HashMap;
use futures_channel::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;
use crate::database::redis::group::group_hash::{
    check_my_group,
    get_group_member,
    get_my_group_key,
};
use crate::database::redis::socket::socket_hash::get_my_info;
use crate::game::components::message::group_message_component::{GroupMessage, GroupMessageSendTo};
use crate::game::enums::core_enum::MessageType;
use crate::game::memory::user::user_memory::get_user_socket;
use std::sync::{Arc,Mutex};

use super::system_message_system::system_message_send;

struct GroupMessageEcsEngine {
    target_users: Arc<Mutex<HashMap<String, UnboundedSender<Message>>>>,
}

impl GroupMessageEcsEngine {
    fn new() -> Self {
        GroupMessageEcsEngine {
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

pub fn group_message_send(
    send_uid: String,
    msg : Message,
) {
    if !check_my_group(&send_uid).unwrap() {
        system_message_send(&send_uid, format!("you are not in group."));
        return;
    }

    let group_message_ecs_engine = Arc::new(GroupMessageEcsEngine::new());

    let msg = msg.to_text().unwrap();
    let data: GroupMessage = serde_json::from_str(msg).unwrap();
    
    let message = data.group_message_send.message;
    let sender_info = get_my_info(&send_uid).unwrap();

    let username = sender_info.iter().find(|(key, _)| *key == "name").map(|(_, value)| value.to_owned()).unwrap();
    let group_message_send_to = GroupMessageSendTo {
        message_type: MessageType::GroupMessage,
        uid: send_uid.clone(),
        username: username,
        message: message,
    };

    let group_key = get_my_group_key(&send_uid).unwrap();
    let group_list = get_group_member(&group_key).unwrap();

    let sockets = get_user_socket();
    let user_sockets = sockets.iter().filter(|user_sockets| group_list.contains(user_sockets.0)).map(|(_, user_socket)| user_socket);

    for this_socket in user_sockets {
        group_message_ecs_engine.add_user(this_socket.id.clone(), this_socket.tx.clone());
    }

    group_message_ecs_engine.broadcast_message(Message::Text(serde_json::to_string(&group_message_send_to).unwrap()));

    group_message_ecs_engine.clear();
}
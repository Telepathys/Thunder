use std::collections::HashMap;
use futures_channel::mpsc::UnboundedSender;
use log::{error};
use tokio_tungstenite::tungstenite::Message;
use std::sync::{Arc,Mutex};
use crate::database::redis::socket::socket_hash::get_my_info;
use crate::game::components::group::group_join_component::{GroupJoin, ResponseGroupJoin};
use crate::game::enums::core_enum::MessageType;
use crate::game::memory::user::user_memory::get_user_socket;
use crate::game::systems::message::system_message_system::system_message_send;
use crate::utils::sha::sha1;
use crate::database::redis::group::group_hash::{
    check_group_exist,
    add_group_list,
    this_group_join,
    get_my_group_key,
    check_my_group,
};

use super::group_leave_system::group_leave;

struct GroupJoinEcsEngine {
    group_user: Arc<Mutex<HashMap<String, UnboundedSender<Message>>>>,
}

impl GroupJoinEcsEngine {
    fn new() -> Self {
        GroupJoinEcsEngine {
            group_user: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn add_user(&self, uid: String, recp: UnboundedSender<Message>) {
        self.group_user.lock().unwrap().insert(uid, recp);
    }

    fn clear(&self) {
        self.group_user.lock().unwrap().clear();
    }

    fn broadcast_message(&self, message: Message) {
        for recp in self.group_user.lock().unwrap().values() {
            let _ = recp.unbounded_send(message.clone()).unwrap();
        }
    }
}

pub fn group_join(
    send_uid: String,
    msg : Message,
) {
    let group_join_ecs_engine = Arc::new(GroupJoinEcsEngine::new());
    let msg = msg.to_text().unwrap();
    let data: GroupJoin = serde_json::from_str(msg).unwrap();
    let group_name = data.group_join.group_name;
    let group_key = sha1(&group_name);
    let sender_info = get_my_info(&send_uid).unwrap();
    let username = sender_info.iter().find(|(key, _)| *key == "name").map(|(_, value)| value.to_owned()).unwrap();

    if check_my_group(&send_uid).unwrap() && get_my_group_key(&send_uid).unwrap() == group_key {
        system_message_send(&send_uid, format!("you are aleady join this group."));
        return;
    } else if check_my_group(&send_uid).unwrap() && get_my_group_key(&send_uid).unwrap() != group_key {
        group_leave(&send_uid.clone(),None);
    }

    if !check_group_exist(&group_key).unwrap() {
        if let Err(err) = add_group_list(&group_key) {
            error!("{}", err);
        }
    }
    let group_list = this_group_join(&group_key, &send_uid.clone()).unwrap();
    let sockets = get_user_socket();
    let user_sockets = sockets.iter().filter(|user_sockets| group_list.contains(user_sockets.0)).map(|(_, user_socket)| user_socket);

    for this_socket in user_sockets {
        group_join_ecs_engine.add_user(this_socket.id.clone(), this_socket.tx.clone());
    }

    let group_join_message_send_to = ResponseGroupJoin {
        message_type: MessageType::GroupJoin,
        group_key: group_key.clone(),
        uid: send_uid.clone(),
        username: username.clone(),
        message: format!("{} is join", username),
    };

    group_join_ecs_engine.broadcast_message(Message::Text(serde_json::to_string(&group_join_message_send_to).unwrap()));

    group_join_ecs_engine.clear();
}
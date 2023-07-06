use std::collections::HashMap;
use futures_channel::mpsc::UnboundedSender;
use log::{error};
use tokio_tungstenite::tungstenite::Message;
use std::sync::{Arc,Mutex};
use crate::database::redis::socket::socket_hash::get_my_info;
use crate::game::components::group::group_leave_component::{GroupLeave, ResponseGroupLeave};
use crate::game::enums::core_enum::MessageType;
use crate::game::memory::user::user_memory::get_user_socket;
use crate::database::redis::group::group_hash::{
    this_group_leave,
    delete_group_list,
    get_my_group_key,
    check_my_group,
};
use crate::game::systems::message::system_message_system::system_message_send;

struct GroupLeaveEcsEngine {
    group_user: Arc<Mutex<HashMap<String, UnboundedSender<Message>>>>,
}

impl GroupLeaveEcsEngine {
    fn new() -> Self {
        GroupLeaveEcsEngine {
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

pub fn group_leave(
    send_uid: &String,
    msg : Option<Message>,
) {
    let group_leave_ecs_engine = Arc::new(GroupLeaveEcsEngine::new());
    let group_key: String;
    if check_my_group(&send_uid).unwrap() {
        if let Some(msg) = msg {
            let msg = msg.to_text().unwrap();
            let data: GroupLeave = serde_json::from_str(msg).unwrap();
            group_key = data.group_leave.group_key;
        } else {
            group_key = get_my_group_key(&send_uid).unwrap()
        }
        let sender_info = get_my_info(&send_uid).unwrap();
        let username = sender_info.iter().find(|(key, _)| *key == "name").map(|(_, value)| value.to_owned()).unwrap();
        let group_list = this_group_leave(&group_key, &send_uid.clone()).unwrap();
        if group_list.is_empty() {
            if let Err(err) = delete_group_list(&group_key) {
                error!("{}", err);
            }
        }
        
        let sockets = get_user_socket();
        let user_sockets = sockets.iter().filter(|user_sockets| group_list.contains(user_sockets.0) || user_sockets.0 == send_uid).map(|(_, user_socket)| user_socket);
    
        // 다른 유저들 소켓 추가
        for this_socket in user_sockets {
            group_leave_ecs_engine.add_user(this_socket.id.clone(), this_socket.tx.clone());
        }
    
        let group_leave_message_send_to = ResponseGroupLeave {
            message_type: MessageType::GroupLeave,
            group_key: group_key.clone(),
            uid: send_uid.clone(),
            username: username.clone(),
            message: format!("{} is leave", username),
        };
    
        group_leave_ecs_engine.broadcast_message(Message::Text(serde_json::to_string(&group_leave_message_send_to).unwrap()));
    
        group_leave_ecs_engine.clear();
    } else {
        system_message_send(&send_uid, format!("you are not in group."))
    }
}
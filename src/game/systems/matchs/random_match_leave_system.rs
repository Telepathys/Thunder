use std::collections::HashMap;
use futures_channel::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;
use std::sync::{Arc,Mutex};
use crate::{
    game::{
        components::matchs::{random_match_leave_component::RandomMatchLeaveSendTo}, 
        memory::user::user_memory::get_user_socket, 
        enums::core_enum::MessageType, systems::message::system_message_system::system_message_send
    },
    database::redis::{
        matchs::match_hash::{
            delete_my_match, get_match_members, get_my_match, delete_match_member, check_my_match_exist,
        }, 
        socket::socket_hash::get_my_info
    },
};

struct RandomMatchLeaveEcsEngine {
    target_users: Arc<Mutex<HashMap<String, UnboundedSender<Message>>>>,
}

impl RandomMatchLeaveEcsEngine {
    fn new() -> Self {
        RandomMatchLeaveEcsEngine {
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

pub fn random_match_leave(
    send_uid: &String,
) {
    let match_leave_ecs_engine = Arc::new(RandomMatchLeaveEcsEngine::new());
    let sender_info = get_my_info(&send_uid).unwrap();
    let username = sender_info.iter().find(|(key, _)| *key == "name").map(|(_, value)| value.to_owned()).unwrap();
    let match_id = get_my_match(send_uid).unwrap();
    if !check_my_match_exist(&send_uid).unwrap() {
        system_message_send(&send_uid, format!("you are not in random_match."));
        return;
    }
    let match_user_list = get_match_members(&match_id).unwrap();
    delete_match_member(&match_id, send_uid).unwrap();
    delete_my_match(send_uid).unwrap();
    let sockets = get_user_socket();
    let user_sockets = sockets.iter().filter(|user_sockets| match_user_list.contains(user_sockets.0)).map(|(_, user_socket)| user_socket);

    for this_socket in user_sockets {
        match_leave_ecs_engine.add_user(this_socket.id.clone(), this_socket.tx.clone());
    }

    let random_match_leave_send_to = RandomMatchLeaveSendTo {
        message_type: MessageType::RandomMatchLeave,
        match_id: match_id,
        uid: send_uid.clone(),
        username: username.clone(),
        message: format!("{} is match leave", username),
    };

    match_leave_ecs_engine.broadcast_message(Message::Text(serde_json::to_string(&random_match_leave_send_to).unwrap()));

    match_leave_ecs_engine.clear();
}
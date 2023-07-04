use std::collections::HashMap;
use futures_channel::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;
use crate::database::redis::matchs::match_hash::{add_match_join_list, add_match_wait_join_user_list, delete_random_match_wait_list};
use crate::game::components::matchs::random_match_wait_join_component::RandomMatchWaitJoinSendTo;
use crate::game::enums::core_enum::MessageType;
use crate::game::memory::user::user_memory::get_user_socket;
use std::sync::{Arc,Mutex};


struct MatchWaitJoinMessageEcsEngine {
    target_users: Arc<Mutex<HashMap<String, UnboundedSender<Message>>>>,
}

impl MatchWaitJoinMessageEcsEngine {
    fn new() -> Self {
        MatchWaitJoinMessageEcsEngine {
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

pub fn random_match_wait_join_start(
    random_match_user_list: Vec<String>,
    match_id: &String,
) {
    let match_wait_join_message_ecs_engine = Arc::new(MatchWaitJoinMessageEcsEngine::new());

    add_match_join_list(&match_id).unwrap();
    for random_match_user in &random_match_user_list {
        add_match_wait_join_user_list(match_id, random_match_user).unwrap();
        delete_random_match_wait_list(random_match_user).unwrap();
    }

    let group_message_send_to = RandomMatchWaitJoinSendTo {
        message_type: MessageType::RamdomMatchWaitJoin,
        match_id: match_id.to_string(),
    };

    let sockets = get_user_socket();
    let user_sockets = sockets.iter().filter(|user_sockets| random_match_user_list.contains(user_sockets.0)).map(|(_, user_socket)| user_socket);

    for this_socket in user_sockets {
        match_wait_join_message_ecs_engine.add_user(this_socket.id.clone(), this_socket.tx.clone());
    }

    match_wait_join_message_ecs_engine.broadcast_message(Message::Text(serde_json::to_string(&group_message_send_to).unwrap()));

    match_wait_join_message_ecs_engine.clear();
}
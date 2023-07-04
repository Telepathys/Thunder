use std::collections::HashMap;
use futures_channel::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;
use std::sync::{Arc,Mutex};
use crate::{
    game::{
        systems::message::system_message_system::system_message_send, 
        components::matchs::random_match_join_component::{
            RandomMatchJoin, RandomMatchJoinSendTo}, 
            memory::user::user_memory::get_user_socket, 
            enums::core_enum::MessageType}, 
            database::redis::{
                matchs::match_hash::{
                    check_match_exist, 
                    add_match_join_user_list, 
                    check_my_match_exist
                }, 
                socket::socket_hash::get_my_info},
};

struct MatchJoinMessageEcsEngine {
    target_users: Arc<Mutex<HashMap<String, UnboundedSender<Message>>>>,
}

impl MatchJoinMessageEcsEngine {
    fn new() -> Self {
        MatchJoinMessageEcsEngine {
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

pub fn random_match_join(
    send_uid: String,
    msg: Message,
) {
    let match_join_message_ecs_engine = Arc::new(MatchJoinMessageEcsEngine::new());
    let msg = msg.to_text().unwrap();
    let data: RandomMatchJoin = serde_json::from_str(msg).unwrap();
    let match_id = data.random_match_join.match_id;
    let sender_info = get_my_info(&send_uid).unwrap();
    let username = sender_info.iter().find(|(key, _)| *key == "name").map(|(_, value)| value.to_owned()).unwrap();
    if !check_match_exist(&match_id).unwrap() {
        system_message_send(&send_uid, format!("The match does not exist."));
        return;
    }

    if check_my_match_exist(&send_uid).unwrap() {
        system_message_send(&send_uid, format!("You are already in the match."));
        return;
    }

    let match_join_list = add_match_join_user_list(&match_id, &send_uid).unwrap();

    let sockets = get_user_socket();
    let user_sockets = sockets.iter().filter(|user_sockets| match_join_list.contains(user_sockets.0)).map(|(_, user_socket)| user_socket);

    for this_socket in user_sockets {
        match_join_message_ecs_engine.add_user(this_socket.id.clone(), this_socket.tx.clone());
    }

    let match_join_message_send_to = RandomMatchJoinSendTo {
        message_type: MessageType::RandomMatchJoin,
        match_id: match_id,
        uid: send_uid.clone(),
        username: username.clone(),
        message: format!("{} is match join", username),
    };

    match_join_message_ecs_engine.broadcast_message(Message::Text(serde_json::to_string(&match_join_message_send_to).unwrap()));

    match_join_message_ecs_engine.clear();
}
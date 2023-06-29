use std::collections::HashMap;
use futures_channel::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;
use crate::database::redis::socket::socket_hash::get_my_info;
use crate::game::components::message::server_message_component::{
    ServerMessage,
    ServerMessageSendTo,
};
use crate::game::memory::user::user_memory::get_user_socket;
use std::sync::{Arc,Mutex};

struct ServerMessageEcsEngine {
    target_users: Arc<Mutex<HashMap<String, UnboundedSender<Message>>>>,
}

impl ServerMessageEcsEngine {
    fn new() -> Self {
        ServerMessageEcsEngine {
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

pub fn server_message_send(
    send_uid: String,
    msg : Message,
) {
    let server_message_ecs_engine = Arc::new(ServerMessageEcsEngine::new());
    let msg = msg.to_text().unwrap();
    let data: ServerMessage = serde_json::from_str(msg).unwrap();
    let message = data.server_message_send.message.clone();
    let sender_info = get_my_info(&send_uid).unwrap();
    let username = sender_info.iter().find(|(key, _)| *key == "name").map(|(_, value)| value.to_owned()).unwrap();

    let server_message_send_to = ServerMessageSendTo {
        uid: send_uid.clone(),
        username: username,
        message: message.clone(),
    };

    let sockets = get_user_socket();
    let user_sockets = sockets.iter().map(|(_, user_socket)| user_socket);
    // let broadcast_recipients = user_sockets.map(|user_socket| user_socket.tx.clone());

    for this_socket in user_sockets {
        server_message_ecs_engine.add_user(this_socket.id.clone(), this_socket.tx.clone());
    }

    server_message_ecs_engine.broadcast_message(Message::Text(serde_json::to_string(&server_message_send_to).unwrap()));

    server_message_ecs_engine.clear();
}
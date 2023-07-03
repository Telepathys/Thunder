use std::collections::HashMap;
use futures_channel::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;
use crate::game::components::message::system_message_component::SystemMessageSendTo;
use crate::game::memory::user::user_memory::get_user_socket;
use std::sync::{Arc,Mutex};
use crate::game::enums::core_enum::{
    MessageType
};

struct SystemrMessageEcsEngine {
    target_users: Arc<Mutex<HashMap<String, UnboundedSender<Message>>>>,
}

impl SystemrMessageEcsEngine {
    fn new() -> Self {
        SystemrMessageEcsEngine {
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

pub fn system_message_send(
    target_uid: &String,
    msg: String,
) {
    let system_message_ecs_engine = Arc::new(SystemrMessageEcsEngine::new());

    let sockets = get_user_socket();
    let user_sockets = sockets.iter().filter(|user_sockets| user_sockets.0 == target_uid).map(|(_, user_socket)| user_socket);

    let system_message_send_to = SystemMessageSendTo {
        message_type: MessageType::System,
        message: msg,
    };

    for this_socket in user_sockets {
        system_message_ecs_engine.add_user(this_socket.id.clone(), this_socket.tx.clone());
    }

    system_message_ecs_engine.broadcast_message(Message::Text(serde_json::to_string(&system_message_send_to).unwrap()));

    system_message_ecs_engine.clear();
}
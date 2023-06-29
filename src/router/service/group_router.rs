use tokio_tungstenite::tungstenite::protocol::Message;
use crate::game::systems::group::{
    group_join_system::{
        group_join
    },
    group_leave_system::{
        group_leave
    }
};

pub fn group_router(send_uid: String, service: &str, msg: Message) {
    match service {
        "group_join" => {
            group_join(send_uid, msg);
        }
        "group_leave" => {
            group_leave(send_uid, Some(msg));
        }
        _ => {
        }
    }
}
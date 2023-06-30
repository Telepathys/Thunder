use tokio_tungstenite::tungstenite::protocol::Message;

use crate::game::systems::matchs::{random_match_wait_system::random_match_wait, random_match_cancel_system::random_match_cancel};

pub fn match_router(send_uid: String, service: &str, msg: Message) {
    match service {
        "random_match_wait" => {
            random_match_wait(send_uid);
        }
        "random_match_cancel" => {
            random_match_cancel(send_uid);
        }
        _ => {
        }
    }
}
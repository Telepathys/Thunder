use tokio_tungstenite::tungstenite::protocol::Message;

use crate::game::systems::matchs::{
    random_match_wait_system::random_match_wait, 
    random_match_cancel_system::random_match_cancel, 
    random_match_join_system::random_match_join, 
    random_match_wait_join_system::random_match_wait_join_start, 
    random_match_complete_system::random_match_complete
};

pub fn match_router(send_uid: String, service: &str, msg: Message) {
    match service {
        "random_match_wait" => {
            random_match_wait(send_uid);
        }
        "random_match_cancel" => {
            random_match_cancel(send_uid);
        }
        "random_match_join" => {
            random_match_join(send_uid, msg);
        }
        "random_match_wait_success" => {
            random_match_wait_join_start(msg);
        }
        "random_match_complete" => {
            random_match_complete(msg);
        }
        _ => {
        }
    }
}
use crate::{
    database::redis::matchs::match_hash::{add_random_match_wait_list, check_random_match_wait_list}, 
    game::systems::message::system_message_system::system_message_send,
};



pub fn random_match_wait(
    send_uid: String,
) {
    if check_random_match_wait_list(&send_uid).unwrap() {
        system_message_send(&send_uid, format!("You are already participating in the match queue."));
        return;
    }

    add_random_match_wait_list(&send_uid).unwrap();
    system_message_send(&send_uid, format!("You have entered the match queue."));
}
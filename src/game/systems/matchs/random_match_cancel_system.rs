use crate::{
    database::redis::matchs::match_hash::{check_random_match_wait_list, delete_random_match_wait_list}, 
    game::systems::message::system_message_system::system_message_send,
};



pub fn random_match_cancel(
    send_uid: String,
) {
    if !check_random_match_wait_list(&send_uid).unwrap() {
        system_message_send(send_uid.clone(), format!("You are not waiting for a match."));
        return;
    }

    delete_random_match_wait_list(&send_uid).unwrap();

    system_message_send(send_uid.clone(), format!("Waiting for the match has been canceled."));
}
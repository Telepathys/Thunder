use tokio_tungstenite::tungstenite::Message;

use crate::{
    game::{systems::message::system_message_system::system_message_send, components::matchs::random_match_complete_component::RandomMatchComplete},
};



pub fn random_match_complete(
    msg: Message,
) {

    let msg = msg.to_text().unwrap();
    let data: RandomMatchComplete= serde_json::from_str(msg).unwrap();
    let match_id = data.random_match_complete.match_id;
    let match_complete_user_list = data.random_match_complete.match_complete_user_list;
    
    for match_complete_user in match_complete_user_list {
        system_message_send(&match_complete_user, format!("All the User in the match have entered and the match({}) will begin.", match_id));
    }
}
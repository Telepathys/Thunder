use std::{fs};
use chrono::{FixedOffset, Utc, DateTime};
use crate::database::redis::message::message_hash::{add_message_history, get_message_history, exists_message_limit_list, get_message_limit_time, delete_message_limit_list, delete_message_history, add_message_limit_list, get_message_history_count};
use crate::game::systems::message::system_message_system::system_message_send;
use crate::game::components::config::config_component::Config;

pub fn message_limit_check (
    send_id: &String,
) -> bool {
    let contents = fs::read_to_string("Config.yaml").expect("Failed to read file");
    let config: Config = serde_yaml::from_str(&contents).expect("Failed to parse YAML");
    let message_limit_second = config.message_limit_second.unwrap_or(5);
    let message_limit_count = config.message_limit_count.unwrap_or(5);
    let message_ban_second = config.message_ban_second.unwrap_or(30);

    if exists_message_limit_list(&send_id).unwrap() {
        let message_limit_time = get_message_limit_time(&send_id).unwrap();
        let kst_offset = {
            let secs = 9 * 3600;
            FixedOffset::east_opt(secs).expect("FixedOffset::east out of bounds")
        };
        let now = Utc::now().with_timezone(&kst_offset);
        let parsed_message_limit_time = DateTime::parse_from_rfc3339(&message_limit_time)
            .expect("Failed to parse message_limit_time")
            .with_timezone(&Utc);
        let time_difference = now.signed_duration_since(parsed_message_limit_time);
        let seconds_difference = time_difference.num_seconds();
        let left_ban_time = message_ban_second as i64 - seconds_difference;

        if left_ban_time > 0 {
            system_message_send(send_id, format!("Your message is in a restricted state. Please try again in {} seconds.", left_ban_time));
            return false;
        }

        delete_message_limit_list(&send_id).unwrap();
    }

    let my_message_history_time = get_message_history(&send_id).unwrap();
    if !my_message_history_time.is_empty() {
        let kst_offset = {
            let secs = 9 * 3600;
            FixedOffset::east_opt(secs).expect("FixedOffset::east out of bounds")
        };
        let now = Utc::now().with_timezone(&kst_offset);
        for message_history_time in my_message_history_time {
            let this_time = DateTime::parse_from_rfc3339(&message_history_time)
            .expect("Failed to parse message_limit_time")
            .with_timezone(&Utc);
            let time_difference = now.signed_duration_since(this_time);
            let seconds_difference = time_difference.num_seconds();
            if seconds_difference > message_limit_second.into() {
                delete_message_history(&send_id, &message_history_time).unwrap();
            }
        }
    }

    let my_message_history_count = get_message_history_count(&send_id).unwrap();
    if my_message_history_count >= message_limit_count as isize {
        add_message_limit_list(&send_id).unwrap();
        system_message_send(send_id, format!("Message transmission is restricted for {} seconds due to indiscriminate messages.", message_ban_second));
        return false;
    }

    add_message_history(&send_id).unwrap();
    true
}
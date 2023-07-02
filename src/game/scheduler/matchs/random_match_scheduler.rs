use std::{time::Duration, fs};

use log::info;
use rand::seq::SliceRandom;
use tokio::time::{self, sleep};

use crate::{game::components::config::config_component::Config, database::redis::matchs::match_hash::{get_random_match_wait_list, delete_random_match_wait_list}};
use tokio::sync::Mutex as AsyncMutex;



pub async fn random_match_scheduler(shared_mutex: AsyncMutex<()>) {
    let contents = fs::read_to_string("Config.yaml").expect("Failed to read file");
    let config: Config = serde_yaml::from_str(&contents).expect("Failed to parse YAML");
    let match_check_time = config.match_check_time.unwrap_or(1);
    let match_require_user_count = config.match_require_user_count.unwrap_or(2);
    let match_make_count_control = config.match_make_count_control.unwrap_or(0.1);
    let match_join_limit_time = config.match_join_limit_time.unwrap_or(30);
    let schedule_duration = Duration::from_secs(match_check_time.into());

    loop {
        {
            let _lock = shared_mutex.lock().await;
            let random_match_wait_list = get_random_match_wait_list().unwrap();

            let match_make_count = random_match_wait_list.len() as f64 / match_require_user_count as f64 * match_make_count_control;
            info!("match_make_count : {}", match_make_count.ceil().to_string());

            for _ in 0..match_make_count.ceil() as usize {
                info!("zzzzzz");
            }


            // let mut rng = rand::thread_rng();
            // let random_pick_user_list = random_match_wait_list
            //     .choose_multiple(&mut rng, 2)
            //     .collect::<Vec<_>>();
            // for random_pick_user in random_pick_user_list {
            //     delete_random_match_wait_list(random_pick_user).unwrap();
            // }
        }

        sleep(schedule_duration).await;
    }
}
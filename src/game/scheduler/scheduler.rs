use super::matchs::random_match_scheduler::random_match_scheduler;



pub async fn scheduler_core() {
    tokio::spawn(async move {
        random_match_scheduler(tokio::sync::Mutex::new(())).await;
    });
}
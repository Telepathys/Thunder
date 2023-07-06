use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct RandomMatchWaitSuccess{
    pub random_match_wait_success: RandomMatchWaitSuccessData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RandomMatchWaitSuccessData {
    pub match_id: String,
    pub match_success_user_list: Vec<String>,
}
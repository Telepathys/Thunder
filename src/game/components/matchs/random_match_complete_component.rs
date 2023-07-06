use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct RandomMatchComplete{
    pub random_match_complete: RandomMatchCompleteData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RandomMatchCompleteData {
    pub match_id: String,
    pub match_complete_user_list: Vec<String>,
}
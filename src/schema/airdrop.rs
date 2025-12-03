use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AirdropRequest {
    pub airdrop_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AirdropResponse {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub reward_amount: f64,
    pub reward_type: String,
    pub status: String,
    pub start_time: String,
    pub end_time: String,
    pub is_participated: bool,
}

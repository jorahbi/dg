use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InviteRewardResponse {
    pub id: String,
    pub title: String,
    pub description: String,
    pub level: i32,
    pub reward_amount: f64,
    pub reward_type: String,
    pub current_progress: i32,
    pub required_progress: i32,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InviteCodeResponse {
    pub invite_code: String,
    pub invite_link: String,
    pub qrcode_url: String,
    pub generated_at: String,
}
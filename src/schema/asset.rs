use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WithdrawalRequest {
    pub amount: f64,
    pub currency: String,
    pub blockchain_code: String,
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EarningResponse {
    pub id: i64,
    pub r#type: String,
    pub amount: f64,
    pub currency: String,
    pub description: String,
    pub created_at: String,
}
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AirdropActivity {
    pub id: i64,
    pub r#type: String, // daily, vip, special
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub activity_start_time: NaiveDateTime,
    pub activity_end_time: NaiveDateTime,
    pub total_rounds: i32,
    pub round_duration: i32,    // 秒
    pub interval_duration: i32, // 秒
    pub participation_type: serde_json::Value,
    pub color: String,
    pub status: String, // active, paused, ended
    pub participant_count: i64,
    pub success_rate: rust_decimal::Decimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AirdropRecord {
    pub id: i64,
    pub user_id: i64,
    pub airdrop_id: i64,
    pub airdrop_title: String,
    pub airdrop_type: String,
    pub dg_amount: rust_decimal::Decimal,
    pub round_number: i32,
    pub status: String, // success, failed, pending
    pub failure_reason: Option<String>,
    pub claimed_at: NaiveDateTime,
    pub transaction_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAirdropRecord {
    pub user_id: i64,
    pub airdrop_id: i64,
    pub airdrop_title: String,
    pub airdrop_type: String,
    pub dg_amount: rust_decimal::Decimal,
    pub round_number: i32,
    pub status: String,
    pub transaction_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimAirdropRequest {
    pub airdrop_id: i64,
    pub current_round: i32,
}

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct InviteReward {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub level: i8,
    pub reward_amount: rust_decimal::Decimal,
    pub reward_type: String,
    pub required_progress: i32,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct InviteRecord {
    pub id: i64,
    pub inviter_id: i64,
    pub invitee_id: i64,
    pub invite_code: String,
    pub status: String, // pending, completed, rewarded
    pub reward_amount: rust_decimal::Decimal,
    pub completed_at: Option<NaiveDateTime>,
    pub rewarded_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewInviteRecord {
    pub inviter_id: i64,
    pub invitee_id: i64,
    pub invite_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInviteRecord {
    pub status: Option<String>,
    pub reward_amount: Option<rust_decimal::Decimal>,
    pub completed_at: Option<NaiveDateTime>,
    pub rewarded_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteStats {
    pub total_invites: i64,
    pub total_rewards: rust_decimal::Decimal,
    pub direct_invites: i64,
    pub indirect_invites: i64,
    pub active_invites: i64,
}

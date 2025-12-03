use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use time::PrimitiveDateTime;

#[derive(Debug, Clone, FromRow)]
pub struct UserSecurityQuestions {
    pub id: u64,
    pub user_id: u64,
    pub question_id: u32,
    pub answer_hash: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SecurityQuestion {
    pub id: i64,
    pub question: String,
    pub is_active: i8,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PasswordResetTokens {
    pub id: i64,
    pub username: String,
    pub expires_at: PrimitiveDateTime,
    pub token_hash: String,
    pub ip_address: String,
    pub is_used: i8,
}

use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
// use derive_builder::Builder;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: u64,               // bigint unsigned NOT NULL
    pub username: String,      // varchar(50) NOT NULL
    pub password_hash: String, // varchar(255) NOT NULL
    pub user_level: u8,        // tinyint unsigned NOT NULL
    pub invite_code: String,   // varchar(20) NOT NULL
    pub inviter_id: u64,       // bigint unsigned DEFAULT NULL
    pub upgrade_progress: i32,
    pub parent_inviter_id: u64,
    pub total_assets: Decimal,                 // decimal(20,8) NOT NULL
    pub dg_amount: Decimal,                    // decimal(20,8) NOT NULL
    pub is_kyc_verified: i8,                   // tinyint(1) NOT NULL
    pub has_security_questions: i8,            // tinyint(1) NOT NULL
    pub is_active: i8,                         // tinyint(1) NOT NULL
    pub is_locked: i8,                         // tinyint(1) NOT NULL
    pub login_attempts: u8,                    // tinyint unsigned NOT NULL
    pub locked_until: Option<OffsetDateTime>,  // timestamp NULL
    pub qr_code_url: Option<String>,           // varchar(500) DEFAULT NULL
    pub created_at: OffsetDateTime,            // timestamp NOT NULL
    pub updated_at: OffsetDateTime,            // timestamp NOT NULL
    pub last_login_at: Option<OffsetDateTime>, // timestamp NULL
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserSecurityAnswer {
    pub id: i64,
    pub user_id: i64,
    pub question_id: i64,
    pub answer_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub nickname: Option<String>,
    pub invite_code: Option<String>,
    pub inviter_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUser {
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub phone: Option<String>,
    pub is_email_verified: Option<bool>,
    pub is_kyc_verified: Option<bool>,
    pub kyc_level: Option<i8>,
    pub user_level: Option<i8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewSecurityAnswer {
    pub user_id: i64,
    pub question_id: i64,
    pub answer_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assets {
    #[serde(rename = "upgradeProgress")]
    pub upgrade_progress: i32,
    #[serde(rename = "amount")]
    pub dg_amount: Decimal,
    #[serde(rename = "totalYield")]
    pub total_assets: Decimal,
    #[serde(rename = "dailyYield")]
    pub daily_balance: Decimal,
    #[serde(rename = "lv")]
    pub user_level: u8,
}

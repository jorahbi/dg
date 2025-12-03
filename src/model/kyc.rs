use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct KycSubmission {
    pub id: String,
    pub user_id: i64,
    pub full_name: String,
    pub id_number: String,
    pub id_card_front_url: String,
    pub id_card_back_url: String,
    pub selfie_url: String,
    pub status: String, // submitted, processing, approved, rejected
    pub rejection_reason: Option<String>,
    pub kyc_level: String, // level_1, level_2
    pub reviewed_by: Option<String>,
    pub reviewed_at: Option<NaiveDateTime>,
    pub estimated_review_time: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewKycSubmission {
    pub id: String,
    pub user_id: i64,
    pub full_name: String,
    pub id_number: String,
    pub id_card_front_url: String,
    pub id_card_back_url: String,
    pub selfie_url: String,
    pub kyc_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateKycSubmission {
    pub status: Option<String>,
    pub rejection_reason: Option<String>,
    pub reviewed_by: Option<String>,
    pub reviewed_at: Option<NaiveDateTime>,
}
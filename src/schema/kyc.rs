use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KycStatusResponse {
    pub status: String,
    pub verification_level: i32,
    pub submitted_at: Option<String>,
    pub reviewed_at: Option<String>,
    pub rejected_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KycApplicationRequest {
    pub real_name: String,
    pub id_number: String,
    pub nationality: String,
    pub birth_date: String,
}
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CarouselItem {
    pub id: i64,
    pub image_url: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub action_url: Option<String>,
    pub sort_order: i32,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PromotionPackage {
    pub id: i64,
    pub name: String,
    pub price: rust_decimal::Decimal,
    pub original_price: rust_decimal::Decimal,
    pub currency: String,
    pub description: Option<String>,
    pub profit_percentage: rust_decimal::Decimal,
    pub duration_days: i32,
    pub features: Option<serde_json::Value>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub stock: i32,
    pub sold: i32,
    pub is_available: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NodeStats {
    pub id: i64,
    pub computing_power: String,
    pub status_message: String,
    pub total_nodes: i32,
    pub active_nodes: i32,
    pub earnings: String,
    pub is_active: bool,
    pub network_status: String,
    pub utilization_rate: rust_decimal::Decimal,
    pub average_response_time: rust_decimal::Decimal,
    pub throughput: rust_decimal::Decimal,
    pub last_update_time: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NewUserBenefit {
    pub id: i64,
    pub user_id: i64,
    pub amount: rust_decimal::Decimal,
    pub currency: String,
    pub is_claimed: bool,
    pub description: String,
    pub expire_at: NaiveDateTime,
    pub claim_conditions: Option<serde_json::Value>,
    pub bonus_multiplier: rust_decimal::Decimal,
    pub claimed_at: Option<NaiveDateTime>,
    pub bonus_id: Option<String>,
    pub transaction_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AboutUs {
    pub id: i64,
    pub email: String,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub address: Option<String>,
    pub version: String,
    pub version_tag: String,
    pub copyright: Option<String>,
    pub disclaimer: Option<String>,
    pub app_description: Option<String>,
    pub team_info: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Statistics {
    pub id: i64,
    pub daily_yield: rust_decimal::Decimal,
    pub total_yield: rust_decimal::Decimal,
    pub today_progress: String,
    pub active_users: i32,
    pub total_users: i32,
    pub total_airdrop: i32,
    pub success_rate: String,
    pub system_status: String,
    pub last_update_time: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewCarouselItem {
    pub image_url: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub action_url: Option<String>,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNodeStats {
    pub computing_power: Option<String>,
    pub status_message: Option<String>,
    pub total_nodes: Option<i32>,
    pub active_nodes: Option<i32>,
    pub earnings: Option<String>,
    pub is_active: Option<bool>,
    pub network_status: Option<String>,
    pub utilization_rate: Option<rust_decimal::Decimal>,
    pub average_response_time: Option<rust_decimal::Decimal>,
    pub throughput: Option<rust_decimal::Decimal>,
    pub last_update_time: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNewUserBenefit {
    pub is_claimed: Option<bool>,
    pub claimed_at: Option<NaiveDateTime>,
    pub bonus_id: Option<String>,
    pub transaction_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateStatistics {
    pub daily_yield: Option<rust_decimal::Decimal>,
    pub total_yield: Option<rust_decimal::Decimal>,
    pub today_progress: Option<String>,
    pub active_users: Option<i32>,
    pub total_users: Option<i32>,
    pub total_airdrop: Option<i32>,
    pub success_rate: Option<String>,
    pub system_status: Option<String>,
    pub last_update_time: Option<NaiveDateTime>,
}

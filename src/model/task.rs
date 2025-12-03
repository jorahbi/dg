use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: i64,
    pub task_type: String,
    pub required_level: i8,
    pub earnings_percent: rust_decimal::Decimal,
    pub amount: rust_decimal::Decimal,
    pub currency: String,
    pub description: Option<String>,
    pub completion_time: i32, // 秒
    pub difficulty: String,   // easy, medium, hard
    pub is_active: bool,
    pub sort_order: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserTaskRecord {
    pub id: i64,
    pub user_id: i64,
    pub task_id: i64,
    pub status: String, // available, running, completed, failed, accelerating
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub estimated_completion_time: Option<NaiveDateTime>,
    pub earnings_rate: Option<rust_decimal::Decimal>,
    pub acceleration_multiplier: rust_decimal::Decimal,
    pub points_used: rust_decimal::Decimal,
    pub completion_reward: rust_decimal::Decimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUserTaskRecord {
    pub user_id: i64,
    pub task_id: i64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserTaskRecord {
    pub status: Option<String>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub estimated_completion_time: Option<NaiveDateTime>,
    pub earnings_rate: Option<rust_decimal::Decimal>,
    pub acceleration_multiplier: Option<rust_decimal::Decimal>,
    pub points_used: Option<rust_decimal::Decimal>,
    pub completion_reward: Option<rust_decimal::Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccelerateTaskRequest {
    pub acceleration_hours: i32,
    pub points_cost: rust_decimal::Decimal,
}

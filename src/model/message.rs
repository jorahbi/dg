use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub content: String,
    pub time: NaiveDateTime,
    pub is_read: bool,
    pub r#type: String, // system, transaction, promotion, security
    pub priority: String, // low, medium, high, urgent
    pub actions: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewMessage {
    pub user_id: i64,
    pub title: String,
    pub content: String,
    pub r#type: String,
    pub priority: String,
    pub actions: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMessage {
    pub is_read: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageStats {
    pub unread_count: i64,
    pub unread_by_type: serde_json::Value,
}
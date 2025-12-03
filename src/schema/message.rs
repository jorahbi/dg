use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponse {
    pub id: i64,
    pub r#type: String,
    pub title: String,
    pub content: String,
    pub is_read: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnreadCountResponse {
    pub unread_count: i64,
    pub system_messages: i64,
    pub reward_messages: i64,
    pub activity_messages: i64,
}
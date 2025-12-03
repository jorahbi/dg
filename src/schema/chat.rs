use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessageRequest {
    pub room_id: String,
    pub content: String,
    pub message_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessageResponse {
    pub id: String,
    pub room_id: String,
    pub user_id: i64,
    pub username: String,
    pub content: String,
    pub message_type: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRoomResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub member_count: i64,
    pub is_private: bool,
}
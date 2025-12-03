use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ChatMessage {
    pub id: i64,
    pub sender_id: i64,
    pub receiver_id: i64,
    pub content: String,
    pub msg_type: i8, // 1文本 2图片 3文件
    pub status: i8,   // 1未读 2已读 3撤回
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewChatMessage {
    pub sender_id: u64,
    pub receiver_id: u64,
    pub content: String,
    pub msg_type: i8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChatMessage {
    pub status: Option<i8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageReq {
    pub receiver_id: u64,
    pub content: String,
    pub msg_type: u8, // 1文本 2图片 3文件
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageVO {
    pub id: i64,
    pub sender_id: u64,
    pub content: String,
    pub msg_type: u8,
    pub created_at: i64, // 前端使用毫秒时间戳
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatUser {
    pub id: i64,
    pub username: String,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub is_online: bool,
    pub last_seen: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub r#type: String, // message, image, typing, read, system
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageData {
    pub message_id: String,
    pub sender_id: String,
    pub content: String,
    pub timestamp: String,
    pub status: String,
}

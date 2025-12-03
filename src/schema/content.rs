use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentResponse {
    pub id: i64,
    pub title: String,
    pub subtitle: String,
    pub content_type: String,
    pub content: String,
    pub banner_url: Option<String>,
    pub author: String,
    pub view_count: i64,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromotionResponse {
    pub id: i64,
    pub title: String,
    pub subtitle: String,
    pub banner_url: String,
    pub description: String,
    pub reward_amount: f64,
    pub status: String,
    pub is_participated: bool,
}
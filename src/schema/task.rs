use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskRequest {
    pub task_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccelerateTaskRequest {
    pub points_used: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResponse {
    pub id: i64,
    pub task_type: String,
    pub required_level: i32,
    pub earnings_percent: f64,
    pub amount: f64,
    pub currency: String,
    pub description: String,
    pub is_accelerating: bool,
    pub status: String,
    pub completion_time: String,
    pub difficulty: String,
}
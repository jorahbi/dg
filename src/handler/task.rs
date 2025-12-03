use crate::{error::Result, extract::AuthUser, schema::common::ApiResponse, state::AppState};
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Json},
};
use serde_json::json;

// 获取任务列表
pub async fn get_tasks(State(_state): State<AppState>) -> Result<impl IntoResponse> {
    let tasks = json!([
        {
            "id": 1,
            "taskType": "AI智能计算",
            "requiredLevel": 1,
            "earningsPercent": 5.8,
            "amount": 150.00,
            "currency": "USDT",
            "description": "为您提供稳定高效的AI算力服务",
            "isAccelerating": false,
            "status": "available",
            "completionTime": "2小时",
            "difficulty": "简单"
        }
    ]);

    let response = ApiResponse::success(tasks);
    Ok(Json(response))
}

// 开始任务
pub async fn start_task(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
    Path(task_id): Path<i64>,
) -> Result<impl IntoResponse> {
    let task_data = json!({
        "taskId": task_id,
        "status": "running",
        "startTime": chrono::Utc::now().to_rfc3339(),
        "estimatedCompletionTime": chrono::Utc::now().to_rfc3339(),
        "earningsRate": 8.70
    });

    let response = ApiResponse::success_with_message(task_data, "Task started");
    Ok(Json(response))
}

// 加速任务
pub async fn accelerate_task(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
    Path(task_id): Path<i64>,
    Json(_payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse> {
    let accelerate_data = json!({
        "taskId": task_id,
        "accelerationMultiplier": 2.0,
        "pointsUsed": 100,
        "newCompletionTime": chrono::Utc::now().to_rfc3339(),
        "remainingAccelerationTime": "2小时"
    });

    let response = ApiResponse::success_with_message(accelerate_data, "Task accelerated successfully");
    Ok(Json(response))
}

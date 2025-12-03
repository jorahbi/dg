use crate::state::AppState;
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};
use std::sync::Arc;

/// 获取定时任务调度器状态
pub async fn get_cron_status(
    State(app_state): State<AppState>,
) -> Result<Json<Value>, StatusCode> {
    let status = app_state.cron_scheduler.get_status().await;

    Ok(Json(json!({
        "success": true,
        "data": {
            "is_running": status.is_running,
            "jobs_count": status.jobs_count
        }
    })))
}

/// 启动定时任务调度器
pub async fn start_cron_scheduler(
    State(app_state): State<AppState>,
) -> Result<Json<Value>, StatusCode> {
    let app_state_clone = Arc::new(app_state.clone());
    match app_state.cron_scheduler.start(app_state_clone).await {
        Ok(_) => Ok(Json(json!({
            "success": true,
            "message": "定时任务调度器启动成功"
        }))),
        Err(e) => {
            tracing::error!("启动定时任务调度器失败: {}", e);
            Ok(Json(json!({
                "success": false,
                "message": format!("启动定时任务调度器失败: {}", e)
            })))
        }
    }
}

/// 停止定时任务调度器
pub async fn stop_cron_scheduler(
    State(app_state): State<AppState>,
) -> Result<Json<Value>, StatusCode> {
    match app_state.cron_scheduler.stop().await {
        Ok(_) => Ok(Json(json!({
            "success": true,
            "message": "定时任务调度器停止成功"
        }))),
        Err(e) => {
            tracing::error!("停止定时任务调度器失败: {}", e);
            Ok(Json(json!({
                "success": false,
                "message": format!("停止定时任务调度器失败: {}", e)
            })))
        }
    }
}
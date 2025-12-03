use crate::{
    error::Result,
    extract::AuthUser,
    schema::common::{ApiResponse, PaginationRequest},
    state::AppState,
};
use axum::{
    extract::{Path, Query, State},
    response::{IntoResponse, Json},
};
use serde_json::json;

// 获取消息列表
pub async fn get_messages(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
    Query(pagination): Query<PaginationRequest>,
) -> Result<impl IntoResponse> {
    let messages = json!({
        "records": [
            {
                "id": 1,
                "type": "system",
                "title": "系统升级通知",
                "content": "系统将于今晚23:00-24:00进行维护升级",
                "isRead": false,
                "createdAt": "2025-11-23T16:30:00Z"
            },
            {
                "id": 2,
                "type": "reward",
                "title": "奖励到账",
                "content": "您的挖矿收益25.5 USDT已到账",
                "isRead": true,
                "createdAt": "2025-11-23T12:00:00Z"
            },
            {
                "id": 3,
                "type": "activity",
                "title": "新活动上线",
                "content": "AI算力狂欢节开启，参与即享双倍收益",
                "isRead": false,
                "createdAt": "2025-11-22T10:15:00Z"
            }
        ],
        "pagination": {
            "page": pagination.page,
            "limit": pagination.limit,
            "total": 3,
            "totalPages": 1
        },
        "unreadCount": 2
    });

    let response = ApiResponse::success(messages);
    Ok(Json(response))
}

// 标记消息为已读
pub async fn mark_message_read(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
    Path(message_id): Path<i64>,
) -> Result<impl IntoResponse> {
    let response_data = json!({
        "messageId": message_id,
        "status": "read",
        "updatedAt": chrono::Utc::now().to_rfc3339()
    });

    let response = ApiResponse::success_with_message(response_data, "Message marked as read");
    Ok(Json(response))
}

// 批量标记消息为已读
pub async fn mark_all_messages_read(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
) -> Result<impl IntoResponse> {
    let response_data = json!({
        "updatedCount": 2,
        "updatedAt": chrono::Utc::now().to_rfc3339()
    });

    let response = ApiResponse::success_with_message(response_data, "All messages marked as read");
    Ok(Json(response))
}

// 删除消息
pub async fn delete_message(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
    Path(message_id): Path<i64>,
) -> Result<impl IntoResponse> {
    let response_data = json!({
        "messageId": message_id,
        "status": "deleted",
        "deletedAt": chrono::Utc::now().to_rfc3339()
    });

    let response = ApiResponse::success_with_message(response_data, "Message deleted");
    Ok(Json(response))
}

// 获取未读消息数量
pub async fn get_unread_count(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
) -> Result<impl IntoResponse> {
    let response_data = json!({
        "unreadCount": 2,
        "systemMessages": 1,
        "rewardMessages": 0,
        "activityMessages": 1
    });

    let response = ApiResponse::success(response_data);
    Ok(Json(response))
}

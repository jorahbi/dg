use crate::{
    error::Result,
    extract::AuthUser,
    schema::common::{ApiResponse, PaginationRequest},
    state::AppState,
};
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Json},
};
use serde_json::json;

// 获取邀请奖励列表
pub async fn get_invite_rewards(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
) -> Result<impl IntoResponse> {
    let rewards = json!([
        {
            "id": "level_1",
            "title": "一级好友奖励",
            "description": "邀请1位好友注册并完成实名认证",
            "level": 1,
            "rewardAmount": 100,
            "rewardType": "points",
            "currentProgress": 0,
            "requiredProgress": 1,
            "status": "pending"
        },
        {
            "id": "level_2",
            "title": "二级好友奖励",
            "description": "累计邀请5位好友注册并完成实名认证",
            "level": 2,
            "rewardAmount": 500,
            "rewardType": "points",
            "currentProgress": 3,
            "requiredProgress": 5,
            "status": "progress"
        }
    ]);

    let response = ApiResponse::success(rewards);
    Ok(Json(response))
}

// 获取邀请码
pub async fn get_invite_code(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
) -> Result<impl IntoResponse> {
    let invite_data = json!({
        "inviteCode": "INVITE123",
        "inviteLink": "https://app.example.com/invite/INVITE123",
        "qrcodeUrl": "https://api.example.com/qrcode/INVITE123",
        "generatedAt": chrono::Utc::now().to_rfc3339()
    });

    let response = ApiResponse::success(invite_data);
    Ok(Json(response))
}

// 获取邀请记录
pub async fn get_invite_records(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
    Query(_pagination): Query<PaginationRequest>,
) -> Result<impl IntoResponse> {
    let records = json!({
        "records": [
            {
                "id": 1,
                "name": "Alice",
                "avatar": "A",
                "level": "LV.3",
                "joinDate": "2025-11-15",
                "reward": 50,
                "children": 3,
                "directReward": "50 DG",
                "indirectReward": "25 DG",
                "totalReward": "75 DG"
            }
        ],
        "stats": {
            "totalInvites": 15,
            "totalRewards": "1,250 DG",
            "directInvites": 8,
            "indirectInvites": 7,
            "activeInvites": 12
        }
    });

    let response = ApiResponse::success(records);
    Ok(Json(response))
}

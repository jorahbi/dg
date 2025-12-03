use crate::{
    error::Result,
    extract::AuthUser,
    schema::{common::ApiResponse, user::UserInfoRes},
    service::UserService,
    state::AppState,
};
use axum::{
    extract::{ State},
    response::{IntoResponse, Json},
};
use serde_json::json;

// 获取用户信息
pub async fn get_user_info(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse> {
    let user_service = UserService::new(&state);
    let user = user_service.get_user_info(auth_user.id).await?;

    // 使用 From trait 转换为响应格式
    let user_info_res = UserInfoRes::from(user);

    let response = ApiResponse::success(user_info_res);
    Ok(Json(response))
}

// 获取用户统计信息
pub async fn get_user_stats(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse> {
    let user_service = UserService::new(&state);
    let stats = user_service.get_user_stats(auth_user.id).await?;

    let response = ApiResponse::success(stats);
    Ok(Json(response))
}

// 获取邀请码
pub async fn get_invite_code(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse> {
    let user_service = UserService::new(&state);
    let invite_code = user_service.get_invite_code(auth_user.id).await?;

    let response = ApiResponse::success_with_message(
        json!({
            "inviteCode": invite_code,
            "inviteLink": format!("https://app.example.com/invite/{}", invite_code),
            "qrcodeUrl": format!("https://api.example.com/qrcode/{}", invite_code),
            "generatedAt": chrono::Utc::now().to_rfc3339()
        }),
        "获取成功",
    );

    Ok(Json(response))
}

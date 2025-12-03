use crate::{
    error::Result,
    extract::AuthUser,
    schema::common::{ApiResponse, PaginationRequest},
    service::AirdropService,
    state::AppState,
};
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Json},
};
use serde_json::json;

// 获取空投活动列表
pub async fn get_airdrops(State(state): State<AppState>) -> Result<impl IntoResponse> {
    let airdrop_service = AirdropService::new(&state);
    let airdrops = airdrop_service.get_airdrops().await?;

    let response = ApiResponse::success(airdrops);
    Ok(Json(response))
}

// 抢空投
pub async fn claim_airdrop(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(_payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse> {
    // 暂时跳过验证，简化实现
    // TODO: 实现真正的参数验证

    let airdrop_service = AirdropService::new(&state);
    // 暂时简化实现，使用固定的airdrop_id
    let result = airdrop_service.claim_airdrop(auth_user.id, 1).await?;

    let response = ApiResponse::success_with_message(result, "Airdrop claimed successfully");
    Ok(Json(response))
}

// 获取空投历史记录
pub async fn get_airdrop_history(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(_pagination): Query<PaginationRequest>,
) -> Result<impl IntoResponse> {
    let airdrop_service = AirdropService::new(&state);
    // 暂时简化实现
    let history = airdrop_service.get_airdrop_history(auth_user.id).await?;

    let response = ApiResponse::success(history);
    Ok(Json(response))
}

// 获取空投统计数据
pub async fn get_airdrop_stats(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse> {
    let airdrop_service = AirdropService::new(&state);
    let stats = airdrop_service.get_airdrop_stats(auth_user.id).await?;

    let response = ApiResponse::success(stats);
    Ok(Json(response))
}

// 获取热门空投活动
pub async fn get_popular_airdrops(
    State(state): State<AppState>,
    Query(params): Query<serde_json::Value>,
) -> Result<impl IntoResponse> {
    let _limit = params
        .get("limit")
        .and_then(|v| v.as_u64())
        .map(|v| v as u32);

    let airdrop_service = AirdropService::new(&state);
    // 暂时简化实现
    let popular_airdrops = airdrop_service.get_popular_airdrops().await?;

    let response = ApiResponse::success_with_message(popular_airdrops, "Popular airdrop activities retrieved successfully");

    Ok(Json(response))
}

// 检查今日空投状态
pub async fn check_daily_airdrop_status(
    State(state): State<AppState>,
    _auth_user: AuthUser,
) -> Result<impl IntoResponse> {
    let _airdrop_service = AirdropService::new(&state);
    // 暂时使用固定的布尔值
    let can_participate = true;

    let response = ApiResponse::success_with_message(
        json!({
            "canParticipateToday": can_participate,
            "participatedToday": !can_participate,
            "nextAvailableTime": if !can_participate {
                let now = chrono::Utc::now();
                let naive_time = now.date_naive().and_hms_opt(23, 59, 59).unwrap_or_default();
                let evening_time = naive_time.and_utc();
                Some(evening_time.to_rfc3339())
            } else {
                None
            }
        }),
        if can_participate {
            "今日可参与空投活动"
        } else {
            "今日已参与过空投活动"
        },
    );

    Ok(Json(response))
}

// 获取用户空投资格信息
pub async fn get_user_airdrop_eligibility(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse> {
    let airdrop_service = AirdropService::new(&state);
    let eligibility = airdrop_service
        .get_user_airdrop_eligibility(auth_user)
        .await?;

    let response = ApiResponse::success(eligibility);
    Ok(Json(response))
}

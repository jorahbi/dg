use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;
use crate::{
    extract::AuthUser,
    schema::common::{ApiResponse, PaginationRequest},
    state::AppState,
    error::Result,
    repository::power_repo::PowerRepo,
    AppError,
};

// 获取用户Power信息
pub async fn get_power_info(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse> {
    let power_info = PowerRepo::get_user_power_info(state.db.as_ref(), auth_user.id).await?;

    match power_info {
        Some(info) => {
            let response = ApiResponse::success(info);
            Ok(Json(response))
        }
        None => Err(AppError::NotFound("User computing power information not found".to_string()))
    }
}

// 获取用户等级列表
pub async fn get_power_levels(
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let levels = PowerRepo::find_all_levels(state.db.as_ref()).await?;
    let response = ApiResponse::success(levels);
    Ok(Json(response))
}

// 获取算力记录
pub async fn get_power_records(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(params): Query<serde_json::Value>,
) -> Result<impl IntoResponse> {
    let page = params.get("page")
        .and_then(|v| v.as_u64())
        .unwrap_or(1) as u32;

    let limit = params.get("limit")
        .and_then(|v| v.as_u64())
        .unwrap_or(20) as u32;

    let power_type = params.get("powerType")
        .and_then(|v| v.as_str());

    let status = params.get("status")
        .and_then(|v| v.as_str());

    let (records, total) = PowerRepo::find_user_power_records(
        state.db.as_ref(),
        auth_user.id,
        page,
        limit,
        power_type,
        status,
    ).await?;

    let pagination_data = crate::schema::common::PaginationData::new(page, limit, total, records);
    let response = ApiResponse::success(pagination_data);
    Ok(Json(response))
}

// 升级等级
pub async fn upgrade_level(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse> {
    let target_level = payload.get("targetLevel")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| AppError::Validation("Missing targetLevel parameter".to_string()))? as i8;

    let can_upgrade = PowerRepo::can_upgrade_to_level(
        state.db.as_ref(),
        auth_user.id,
        target_level,
    ).await?;

    if !can_upgrade {
        return Err(AppError::Business("Insufficient invitation count to upgrade to target level".to_string()));
    }

    // 更新用户等级
    PowerRepo::update_user_level(state.db.as_ref(), auth_user.id, target_level).await?;

    // 获取新的奖励倍率
    let levels = PowerRepo::find_all_levels(state.db.as_ref()).await?;
    let reward_multiplier = levels
        .into_iter()
        .find(|level| level.id == target_level)
        .map(|level| level.reward_multiplier)
        .unwrap_or_else(|| rust_decimal::Decimal::ONE);

    let response = ApiResponse::success_with_message(
        json!({
            "newLevel": target_level,
            "upgradeTime": chrono::Utc::now().to_rfc3339(),
            "rewardMultiplier": reward_multiplier
        }),
        "升级成功"
    );

    Ok(Json(response))
}

// 提交提现请求
pub async fn withdraw_power(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse> {
    let amount: String = payload.get("amount")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::Validation("Missing amount parameter".to_string()))?
        .to_string();

    let currency = payload.get("currency")
        .and_then(|v| v.as_str())
        .unwrap_or("USDT");

    let destination_address = payload.get("destinationAddress")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::Validation("Missing destinationAddress parameter".to_string()))?;

    // 验证金额
    let amount_decimal = amount.parse::<rust_decimal::Decimal>()
        .map_err(|_| AppError::Validation("Amount format error".to_string()))?;

    // 检查用户余额
    let user = crate::repository::user_repo::UserRepo::find_by_id(state.db.as_ref(), auth_user.id)
        .await?
        .ok_or_else(|| AppError::NotFound("User does not exist".to_string()))?;

    if user.dg_amount < amount_decimal {
        return Err(AppError::Business("Insufficient balance".to_string()));
    }

    // 生成提现ID
    let withdrawal_id = format!("withdraw_{}", chrono::Utc::now().timestamp_millis());

    // 创建提现记录
    let withdrawal = crate::model::asset::NewWithdrawalRecord {
        id: withdrawal_id.clone(),
        user_id: auth_user.id,
        amount: amount_decimal,
        currency: currency.to_string(),
        blockchain_code: "TRC20".to_string(),
        address: destination_address.to_string(),
        fee: rust_decimal::Decimal::ONE, // 1 USDT 手续费
    };

    crate::repository::asset_repo::AssetRepo::create_withdrawal_record(state.db.as_ref(), &withdrawal).await?;

    // 扣除用户余额
    let actual_withdraw_amount = amount_decimal - withdrawal.fee;
    crate::repository::user_repo::UserRepo::subtract_dg_amount(state.db.as_ref(), auth_user.id, actual_withdraw_amount).await?;

    let response = ApiResponse::success_with_message(
        json!({
            "withdrawalId": withdrawal_id,
            "amount": amount,
            "currency": currency,
            "status": "pending",
            "submitTime": chrono::Utc::now().to_rfc3339(),
            "estimatedProcessingTime": "24小时"
        }),
        "提现申请提交成功"
    );

    Ok(Json(response))
}

// 获取提现详情
pub async fn get_withdrawal(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(withdrawal_id): Path<String>,
) -> Result<impl IntoResponse> {
    let withdrawal = crate::repository::asset_repo::AssetRepo::find_withdrawal_by_id(
        state.db.as_ref(),
        auth_user.id,
        &withdrawal_id,
    ).await?;

    match withdrawal {
        Some(record) => {
            let response = ApiResponse::success(record);
            Ok(Json(response))
        }
        None => Err(AppError::NotFound("Withdrawal record not found".to_string()))
    }
}

// 取消提现请求
pub async fn cancel_withdrawal(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(withdrawal_id): Path<String>,
) -> Result<impl IntoResponse> {
    crate::repository::asset_repo::AssetRepo::cancel_withdrawal(state.db.as_ref(), auth_user.id, &withdrawal_id).await?;

    let response = ApiResponse::success_with_message(
        json!({
            "withdrawalId": withdrawal_id,
            "status": "cancelled",
            "cancelTime": chrono::Utc::now().to_rfc3339()
        }),
        "提现请求已取消"
    );

    Ok(Json(response))
}
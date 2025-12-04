use crate::utils::convert::FromWith;
use crate::{
    error::Result,
    extract::AuthUser,
    model::power::{convert_power_packages, convert_user_power_records},
    repository::power_repo::PowerRepo,
    schema::{
        common::{ApiResponse, PaginationRequest},
        power::{PowerPackagesResponse, PowerRecordsPagination, PowerRecordsResponse},
        UserPowerRecordStatsResp,
    },
    state::AppState,
    AppError,
};
use axum::{
    extract::{Path, Query, State},
    response::{IntoResponse, Json},
};
use serde_json::json;
use std::collections::HashMap;

// 获取所有可用算力包
pub async fn get_all_power_packages(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse> {
    // 获取所有可用的算力包
    let packages = PowerRepo::get_all_power_packages(&state.db).await?;

    // 应用多语言转换
    let package_items = convert_power_packages(packages, &auth_user.lang);
    let total_count = package_items.len() as u32;

    let response = PowerPackagesResponse {
        packages: package_items,
        total_count,
    };

    Ok(Json(ApiResponse::success(response)))
}

///开启算力加速
pub async fn start_power(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(upp): Path<u64>,
) -> Result<impl IntoResponse> {
    let m_power = PowerRepo::start_user_power_record(&state.db, auth_user.id, upp).await;
    let resp: HashMap<String, String> = match m_power {
        Ok(_) => HashMap::new(),
        Err(err) => {
            tracing::error!("Enable computing power acceleration: {}", err);
            return Err(AppError::NotFound(format!(
                "Computing power package not exists:{}",
                err
            )));
        }
    };

    Ok(Json(ApiResponse::success(resp)))
}

// 获取Power记录
pub async fn get_power_records(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(pagination): Query<PaginationRequest>,
) -> Result<impl IntoResponse> {
    // 使用分页请求的默认值
    let page = pagination.page.unwrap_or(1);
    let limit = if pagination.limit.is_some() && pagination.limit.unwrap() > 100 {
        100 // 限制最大每页数量
    } else {
        pagination.limit.unwrap_or(20)
    };

    // 获取用户的算力记录
    let (records, total) = PowerRepo::get_user_power(&state.db, auth_user.id, page, limit).await?;

    // 转换为响应格式，使用 auth_user 的语言偏好进行多语言转换
    let power_records = convert_user_power_records(records, &auth_user.lang);

    let total_pages = ((total as f64) / (limit as f64)).ceil() as u32;

    let pagination_info = PowerRecordsPagination {
        page,
        limit,
        total,
        total_pages,
    };

    let response = PowerRecordsResponse {
        records: power_records,
        pagination: pagination_info,
    };

    Ok(Json(ApiResponse::success(response)))
}

// 获取算力统计信息
pub async fn get_power_stats(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(date): Path<time::Date>,
) -> Result<impl IntoResponse> {
    let result = PowerRepo::get_daily_power_record_by_date(&state.db, &date, auth_user.id).await;
    match result {
        Ok(result) => {
            let res: Vec<UserPowerRecordStatsResp> = result
                .into_iter()
                .map(|item| UserPowerRecordStatsResp::from_with(item, ""))
                .collect();
            Ok(Json(res))
        }
        Err(err) => {
            tracing::error!(
                "get_power_stats id: {}, date: {}, err: {}",
                auth_user.id,
                date,
                err
            );
            Err(AppError::NotFound("".to_string()))
        }
    }
}

// 升级等级
pub async fn upgrade_level(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
    Path(_level_id): Path<i64>,
) -> Result<impl IntoResponse> {
    let upgrade_result = json!({
        "currentLevel": 4,
        "currentPower": 20.0,
        "bonusPower": 4.2,
        "newEarningRate": 0.15,
        "upgradedAt": chrono::Utc::now().to_rfc3339()
    });

    let response = ApiResponse::success_with_message(upgrade_result, "Level upgrade successful");
    Ok(Json(response))
}

// 提现Power
pub async fn withdraw_power(
    _state: State<AppState>,
    _auth_user: AuthUser,
    Json(_payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse> {
    // 暂时简化实现，避免复杂逻辑的编译错误
    // TODO: 实现真正的提现逻辑
    let response = ApiResponse::success_with_message(
        json!({
            "withdrawalId": format!("withdraw_{}", chrono::Utc::now().timestamp_millis()),
            "status": "pending",
            "createdAt": chrono::Utc::now().to_rfc3339()
        }),
        "Withdrawal application submitted",
    );

    Ok(Json(response))
}

// 获取提现记录
pub async fn get_withdrawal(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
    Query(pagination): Query<PaginationRequest>,
) -> Result<impl IntoResponse> {
    let records = json!({
        "records": [
            {
                "id": "WDR123456",
                "amount": 100.0,
                "status": "pending",
                "createdAt": "2025-11-23T10:30:00Z",
                "processedAt": null
            }
        ],
        "pagination": {
            "page": pagination.page,
            "limit": pagination.limit,
            "total": 1,
            "totalPages": 1
        }
    });

    let response = ApiResponse::success(records);
    Ok(Json(response))
}

// 取消提现
pub async fn cancel_withdrawal(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
    Path(_withdrawal_id): Path<String>,
) -> Result<impl IntoResponse> {
    let cancel_result = json!({
        "withdrawalId": _withdrawal_id,
        "status": "cancelled",
        "cancelledAt": chrono::Utc::now().to_rfc3339()
    });

    let response = ApiResponse::success_with_message(cancel_result, "Withdrawal cancelled");
    Ok(Json(response))
}

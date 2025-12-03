use axum::{
    extract::{Query, State, Path},
    response::{IntoResponse, Json},
};
use serde_json::json;
use crate::{
    extract::AuthUser,
    schema::common::{ApiResponse, PaginationRequest},
    state::AppState,
    error::Result,
};

// 获取充值记录
pub async fn get_recharge_records(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
    Query(pagination): Query<PaginationRequest>,
) -> Result<impl IntoResponse> {
    let records = json!({
        "records": [
            {
                "id": 1,
                "type": "USDT",
                "amount": 1000.00,
                "transactionHash": "0x1234567890abcdef",
                "address": "TRX742d35Cc6634C0532925a3b844Bc454e4438f33e",
                "confirmations": 12,
                "fee": 1.5,
                "status": "completed",
                "createdAt": "2025-11-20T10:30:00Z"
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

// 获取提现记录
pub async fn get_withdrawal_records(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
    Query(pagination): Query<PaginationRequest>,
) -> Result<impl IntoResponse> {
    let records = json!({
        "records": [
            {
                "id": "WDR123456",
                "amount": 500.00,
                "currency": "USDT",
                "blockchainCode": "TRC20",
                "address": "TRX742d35Cc6634C0532925a3b844Bc454e4438f33e",
                "fee": 2.5,
                "status": "pending",
                "estimatedProcessingTime": "24小时",
                "createdAt": "2025-11-23T15:45:00Z"
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

// 获取兑换记录
pub async fn get_conversion_records(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
    Query(pagination): Query<PaginationRequest>,
) -> Result<impl IntoResponse> {
    let records = json!({
        "records": [
            {
                "id": 1,
                "fromCurrency": "USDT",
                "toCurrency": "BTC",
                "fromAmount": 1000.00,
                "toAmount": 0.025,
                "exchangeRate": 0.000025,
                "fee": 0.5,
                "transactionId": "TXN789012",
                "status": "completed",
                "createdAt": "2025-11-22T09:15:00Z"
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

// 获取收益记录
pub async fn get_asset_earnings(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
    Query(pagination): Query<PaginationRequest>,
) -> Result<impl IntoResponse> {
    let earnings = json!({
        "records": [
            {
                "id": 1,
                "type": "mining",
                "amount": 25.50,
                "currency": "USDT",
                "description": "AI算力挖矿收益",
                "createdAt": "2025-11-23T12:00:00Z"
            },
            {
                "id": 2,
                "type": "referral",
                "amount": 10.00,
                "currency": "USDT",
                "description": "邀请好友奖励",
                "createdAt": "2025-11-23T08:30:00Z"
            }
        ],
        "pagination": {
            "page": pagination.page,
            "limit": pagination.limit,
            "total": 2,
            "totalPages": 1
        },
        "stats": {
            "totalEarnings": 1250.75,
            "todayEarnings": 35.50,
            "monthlyEarnings": 750.25
        }
    });

    let response = ApiResponse::success(earnings);
    Ok(Json(response))
}

// 获取支持的区块链
pub async fn get_supported_blockchains(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse> {
    let blockchains = json!([
        {
            "code": "TRC20",
            "name": "TRC20 (USDT)",
            "fee": 1.0,
            "minAmount": 10.0,
            "maxAmount": 100000.0,
            "icon": "https://example.com/icons/trc20.png",
            "confirmationTime": "5分钟",
            "isAvailable": true
        },
        {
            "code": "ERC20",
            "name": "ERC20 (USDT)",
            "fee": 2.5,
            "minAmount": 10.0,
            "maxAmount": 100000.0,
            "icon": "https://example.com/icons/erc20.png",
            "confirmationTime": "15分钟",
            "isAvailable": true
        }
    ]);

    let response = ApiResponse::success(blockchains);
    Ok(Json(response))
}

// 创建提现请求
pub async fn create_withdrawal(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse> {
    let withdrawal_data = json!({
        "id": "WDR789012",
        "amount": payload.get("amount"),
        "currency": payload.get("currency"),
        "blockchainCode": payload.get("blockchainCode"),
        "address": payload.get("address"),
        "fee": 2.5,
        "status": "pending",
        "estimatedProcessingTime": "24小时",
        "createdAt": chrono::Utc::now().to_rfc3339()
    });

    let response = ApiResponse::success_with_message(withdrawal_data, "Withdrawal request submitted");
    Ok(Json(response))
}

// 取消提现请求
pub async fn cancel_asset_withdrawal(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
    Path(withdrawal_id): Path<String>,
) -> Result<impl IntoResponse> {
    let cancel_data = json!({
        "withdrawalId": withdrawal_id,
        "status": "cancelled",
        "cancelTime": chrono::Utc::now().to_rfc3339(),
        "refundAmount": 500.00
    });

    let response = ApiResponse::success_with_message(cancel_data, "Withdrawal request cancelled");
    Ok(Json(response))
}

// 币种兑换
pub async fn exchange_currency(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse> {
    let exchange_result = json!({
        "userId": _auth_user.id,
        "fromCurrency": payload.get("fromCurrency"),
        "toCurrency": payload.get("toCurrency"),
        "fromAmount": payload.get("fromAmount"),
        "toAmount": payload.get("toAmount"),
        "exchangeRate": 1.0,
        "fee": 0.5,
        "transactionId": format!("EXCH{}", chrono::Utc::now().timestamp()),
        "status": "completed",
        "completedAt": chrono::Utc::now().to_rfc3339(),
        "message": "币种兑换成功"
    });

    let response = ApiResponse::success_with_message(exchange_result, "Currency exchange successful");
    Ok(Json(response))
}

// 资产提现
pub async fn withdraw_asset(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse> {
    let withdrawal_result = json!({
        "userId": _auth_user.id,
        "withdrawalId": format!("WITHDRAW{}", chrono::Utc::now().timestamp()),
        "currency": payload.get("currency"),
        "amount": payload.get("amount"),
        "address": payload.get("address"),
        "blockchain": payload.get("blockchain"),
        "fee": 2.0,
        "actualAmount": payload.get("amount").map(|v| v.as_f64().unwrap_or(0.0) - 2.0),
        "status": "pending",
        "createdAt": chrono::Utc::now().to_rfc3339(),
        "message": "提现申请已提交，等待审核"
    });

    let response = ApiResponse::success_with_message(withdrawal_result, "Withdrawal request submitted");
    Ok(Json(response))
}

// 获取提现详情
pub async fn get_withdrawal_detail(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
    Path(withdrawal_id): Path<String>,
) -> Result<impl IntoResponse> {
    let withdrawal_detail = json!({
        "withdrawalId": withdrawal_id,
        "currency": "USDT",
        "amount": 1000.0,
        "address": "TRX742d35Cc6634C0532925a3b844Bc454e4438f33e",
        "blockchain": "TRC-20",
        "status": "completed",
        "fee": 2.0,
        "actualAmount": 998.0,
        "transactionHash": "0x1234567890abcdef1234567890abcdef12345678",
        "createdAt": "2025-11-20T10:30:00Z",
        "completedAt": "2025-11-20T12:15:00Z"
    });

    let response = ApiResponse::success(withdrawal_detail);
    Ok(Json(response))
}

// 取消资产提现（别名函数）
pub async fn cancel_withdrawal_asset(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
    Path(withdrawal_id): Path<String>,
) -> Result<impl IntoResponse> {
    let cancel_data = json!({
        "withdrawalId": withdrawal_id,
        "status": "cancelled",
        "cancelTime": chrono::Utc::now().to_rfc3339(),
        "refundAmount": 1000.0
    });

    let response = ApiResponse::success_with_message(cancel_data, "Withdrawal request cancelled");
    Ok(Json(response))
}
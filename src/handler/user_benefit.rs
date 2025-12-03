use crate::{error::Result, extract::AuthUser, schema::common::ApiResponse, state::AppState};
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Json},
};
use serde_json::json;

// 获取用户福利中心
pub async fn get_benefit_center(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
) -> Result<impl IntoResponse> {
    let benefit_center = json!({
        "userId": _auth_user.id,
        "userLevel": 3,
        "userLevelName": "黄金会员",
        "nextLevel": {
            "level": 4,
            "name": "铂金会员",
            "requiredExp": 5000,
            "currentExp": 3250,
            "progress": 65.0,
            "remainingExp": 1750
        },
        "benefits": [
            {
                "id": 1,
                "title": "生日礼金",
                "description": "生日当月可获得100 USDT礼金",
                "type": "birthday",
                "status": "available",
                "rewardAmount": 100.0,
                "conditions": "完成KYC认证",
                "isClaimed": false,
                "nextAvailable": "2026-06-15"
            },
            {
                "id": 2,
                "title": "签到奖励",
                "description": "每日签到可获得积分奖励",
                "type": "daily_signin",
                "status": "available",
                "rewardAmount": 10.0,
                "conditions": "每日登录",
                "isClaimed": false,
                "consecutiveDays": 7
            },
            {
                "id": 3,
                "title": "升级礼包",
                "description": "升级至更高等级可获得丰厚礼包",
                "type": "level_up",
                "status": "pending",
                "nextLevelReward": 500.0,
                "conditions": "累计经验值达到要求"
            }
        ],
        "statistics": {
            "totalBenefitsClaimed": 1250.8,
            "currentMonthBenefits": 125.5,
            "availableBenefits": 6,
            "expiredBenefits": 2
        }
    });

    let response = ApiResponse::success(benefit_center);
    Ok(Json(response))
}


// 领取福利
pub async fn claim_benefit(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
    Path(benefit_id): Path<i64>,
) -> Result<impl IntoResponse> {
    let claim_result = json!({
        "userId": _auth_user.id,
        "benefitId": benefit_id,
        "status": "claimed",
        "claimedAt": chrono::Utc::now().to_rfc3339(),
        "rewardAmount": 100.0,
        "rewardType": "USDT",
        "transactionId": format!("BENEFIT{}", chrono::Utc::now().timestamp()),
        "message": "福利领取成功"
    });

    let response = ApiResponse::success_with_message(claim_result, "Benefit claimed successfully");
    Ok(Json(response))
}

// 获取新用户福利
pub async fn get_new_user_benefit(
    State(_state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse> {
    let benefit = json!({
        "userId": auth_user.id,
        "isEligible": true,
        "benefitAmount": 100.0,
        "currency": "USDT",
        "status": "available",
        "conditions": [
            "完成KYC认证",
            "首次购买算力套餐"
        ],
        "expiryDate": chrono::Utc::now() + chrono::Duration::days(30),
        "description": "新用户专享体验金，可用于购买算力套餐"
    });

    let response = ApiResponse::success(benefit);
    Ok(Json(response))
}

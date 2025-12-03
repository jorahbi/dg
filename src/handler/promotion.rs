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

// 获取推广活动列表
pub async fn get_promotions(
    State(_state): State<AppState>,
    Query(pagination): Query<PaginationRequest>,
) -> Result<impl IntoResponse> {
    let promotions = json!({
        "records": [
            {
                "id": 1,
                "title": "新用户专享福利",
                "subtitle": "注册即送100 USDT体验金",
                "type": "new_user",
                "bannerUrl": "https://example.com/banners/new-user.jpg",
                "description": "新用户注册完成KYC认证后，即可获得100 USDT体验金，可用于购买算力套餐",
                "rewardAmount": 100.0,
                "rewardType": "USDT",
                "conditions": [
                    "完成手机号验证",
                    "完成KYC身份认证",
                    "首次购买任意算力套餐"
                ],
                "startTime": "2025-11-01T00:00:00Z",
                "endTime": "2025-12-31T23:59:59Z",
                "participantCount": 15850,
                "maxParticipants": 50000,
                "status": "active",
                "isParticipated": false
            },
            {
                "id": 2,
                "title": "AI算力狂欢节",
                "subtitle": "双倍收益限时开启",
                "type": "double_rewards",
                "bannerUrl": "https://example.com/banners/double-rewards.jpg",
                "description": "活动期间，所有算力套餐享受双倍收益，邀请好友获得更多奖励",
                "rewardMultiplier": 2.0,
                "conditions": [
                    "持有任意有效算力套餐",
                    "每日登录签到"
                ],
                "startTime": "2025-11-20T00:00:00Z",
                "endTime": "2025-11-30T23:59:59Z",
                "participantCount": 25680,
                "maxParticipants": 100000,
                "status": "active",
                "isParticipated": true
            }
        ],
        "pagination": {
            "page": pagination.page,
            "limit": pagination.limit,
            "total": 2,
            "totalPages": 1
        }
    });

    let response = ApiResponse::success(promotions);
    Ok(Json(response))
}

// 获取活动详情
pub async fn get_promotion_detail(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
    Path(promotion_id): Path<i64>,
) -> Result<impl IntoResponse> {
    let promotion_detail = json!({
        "id": promotion_id,
        "title": "新用户专享福利",
        "subtitle": "注册即送100 USDT体验金",
        "type": "new_user",
        "bannerUrl": "https://example.com/banners/new-user.jpg",
        "description": "新用户注册完成KYC认证后，即可获得100 USDT体验金，可用于购买算力套餐。体验金有效期为30天，请在有效期内使用。",
        "rules": [
            "每人仅限参与一次",
            "体验金不能提现，只能用于购买算力套餐",
            "体验金有效期为30天",
            "作弊行为将取消参与资格"
        ],
        "rewardAmount": 100.0,
        "rewardType": "USDT",
        "conditions": [
            {
                "title": "完成手机号验证",
                "description": "绑定并验证手机号码",
                "isCompleted": true,
                "reward": 30.0
            },
            {
                "title": "完成KYC身份认证",
                "description": "上传身份证件并完成实名认证",
                "isCompleted": true,
                "reward": 40.0
            },
            {
                "title": "首次购买任意算力套餐",
                "description": "购买任意金额的算力套餐",
                "isCompleted": false,
                "reward": 30.0
            }
        ],
        "progress": {
            "completedSteps": 2,
            "totalSteps": 3,
            "percentage": 66.7,
            "claimedRewards": 70.0,
            "pendingRewards": 30.0
        },
        "startTime": "2025-11-01T00:00:00Z",
        "endTime": "2025-12-31T23:59:59Z",
        "participantCount": 15850,
        "maxParticipants": 50000,
        "status": "active",
        "isParticipated": true
    });

    let response = ApiResponse::success(promotion_detail);
    Ok(Json(response))
}

// 参与活动
pub async fn join_promotion(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
    Path(promotion_id): Path<i64>,
) -> Result<impl IntoResponse> {
    let join_data = json!({
        "promotionId": promotion_id,
        "userId": _auth_user.id,
        "status": "participated",
        "joinedAt": chrono::Utc::now().to_rfc3339(),
        "message": "成功参与活动"
    });

    let response = ApiResponse::success_with_message(join_data, "Activity joined successfully");
    Ok(Json(response))
}

// 领取奖励
pub async fn claim_reward(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
    Path(promotion_id): Path<i64>,
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse> {
    let claim_data = json!({
        "promotionId": promotion_id,
        "stepId": payload.get("stepId"),
        "rewardAmount": 30.0,
        "rewardType": "USDT",
        "status": "claimed",
        "claimedAt": chrono::Utc::now().to_rfc3339(),
        "transactionId": format!("TXN{}", chrono::Utc::now().timestamp())
    });

    let response = ApiResponse::success_with_message(claim_data, "Reward claimed successfully");
    Ok(Json(response))
}

// 获取我的活动
pub async fn get_my_promotions(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
    Query(pagination): Query<PaginationRequest>,
) -> Result<impl IntoResponse> {
    let my_promotions = json!({
        "records": [
            {
                "id": 2,
                "title": "AI算力狂欢节",
                "subtitle": "双倍收益限时开启",
                "type": "double_rewards",
                "status": "participated",
                "progress": {
                    "completedSteps": 1,
                    "totalSteps": 2,
                    "percentage": 50.0
                },
                "earnedRewards": 85.6,
                "pendingRewards": 85.6,
                "joinedAt": "2025-11-20T10:30:00Z",
                "lastActiveAt": "2025-11-23T12:00:00Z"
            },
            {
                "id": 1,
                "title": "新用户专享福利",
                "subtitle": "注册即送100 USDT体验金",
                "type": "new_user",
                "status": "completed",
                "progress": {
                    "completedSteps": 3,
                    "totalSteps": 3,
                    "percentage": 100.0
                },
                "earnedRewards": 100.0,
                "pendingRewards": 0.0,
                "joinedAt": "2025-11-15T14:20:00Z",
                "completedAt": "2025-11-18T16:45:00Z"
            }
        ],
        "pagination": {
            "page": pagination.page,
            "limit": pagination.limit,
            "total": 2,
            "totalPages": 1
        },
        "stats": {
            "totalParticipated": 5,
            "totalEarned": 1250.8,
            "activePromotions": 2,
            "completedPromotions": 3
        }
    });

    let response = ApiResponse::success(my_promotions);
    Ok(Json(response))
}

// 获取限时优惠套餐
pub async fn get_promotion_packages(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
) -> Result<impl IntoResponse> {
    let packages = json!({
        "packages": [
            {
                "id": 1,
                "name": "AI算力体验套餐",
                "originalPrice": 200.0,
                "promotionPrice": 100.0,
                "discount": 50,
                "description": "限时5折优惠，体验AI算力挖矿",
                "features": [
                    "100 USDT算力套餐",
                    "30天有效期",
                    "日均收益3-5 USDT",
                    "支持多种加密货币"
                ],
                "endTime": chrono::Utc::now() + chrono::Duration::days(7),
                "stock": 100,
                "sold": 75,
                "tags": ["热销", "限时", "新手推荐"]
            },
            {
                "id": 2,
                "name": "专业矿工套餐",
                "originalPrice": 1000.0,
                "promotionPrice": 750.0,
                "discount": 25,
                "description": "专业矿工专享，高性能算力套餐",
                "features": [
                    "500 USDT算力套餐",
                    "60天有效期",
                    "日均收益25-30 USDT",
                    "专属客服支持"
                ],
                "endTime": chrono::Utc::now() + chrono::Duration::days(14),
                "stock": 50,
                "sold": 30,
                "tags": ["专业", "高收益", "VIP"]
            }
        ]
    });

    let response = ApiResponse::success(packages);
    Ok(Json(response))
}

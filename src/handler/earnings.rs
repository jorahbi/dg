use crate::{
    error::Result,
    extract::AuthUser,
    schema::common::{ApiResponse, DateRangeQuery},
    state::AppState,
};
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Json},
};
use serde_json::json;

// 获取收益数据
pub async fn get_earnings(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
    Query(_query): Query<DateRangeQuery>,
) -> Result<impl IntoResponse> {
    // 在实际实现中，这里应该查询数据库获取收益数据
    let earnings_data = json!({
        "earnings": [
            {
                "id": 1,
                "date": "2025-11-23",
                "amount": 156.5,
                "source": {
                    "id": "mining",
                    "name": "挖矿收益",
                    "color": "#4ECDC4"
                },
                "status": "confirmed",
                "description": "算力挖矿收益 156.50 DG - 已确认"
            }
        ],
        "summary": {
            "totalAmount": 1587.5,
            "todayAmount": 181.5,
            "weekAmount": 234.8,
            "monthAmount": 892.3,
            "totalCount": 25,
            "statusCounts": {
                "confirmed": 20,
                "pending": 3,
                "failed": 1,
                "cancelled": 1
            },
            "sourceAmounts": {
                "mining": 892.5,
                "referral": 234.0,
                "task": 156.3,
                "staking": 189.7,
                "airdrop": 115.0
            }
        }
    });

    let response = ApiResponse::success(earnings_data);
    Ok(Json(response))
}

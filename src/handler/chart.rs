use crate::{error::Result, extract::AuthUser, schema::common::ApiResponse, state::AppState};
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Json},
};
use serde_json::json;

// 获取算力套餐收益统计
pub async fn get_power_chart_data(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
    Query(params): Query<serde_json::Value>,
) -> Result<impl IntoResponse> {
    let time_range = params
        .get("timeRange")
        .and_then(|v| v.as_str())
        .unwrap_or("7d");

    let chart_data = match time_range {
        "24h" => json!({
            "labels": ["00:00", "04:00", "08:00", "12:00", "16:00", "20:00"],
            "earnings": [2.5, 3.2, 4.1, 5.8, 4.5, 3.9],
            "hashrate": [850, 920, 1050, 1200, 1100, 980],
            "activePower": 12.5
        }),
        "7d" => json!({
            "labels": ["周一", "周二", "周三", "周四", "周五", "周六", "周日"],
            "earnings": [45.2, 52.8, 48.6, 61.3, 58.9, 72.4, 68.1],
            "hashrate": [850, 920, 1050, 1200, 1100, 1350, 1280],
            "activePower": 15.8
        }),
        "30d" => json!({
            "labels": ["第1周", "第2周", "第3周", "第4周"],
            "earnings": [320.5, 385.2, 412.8, 456.3],
            "hashrate": [950, 1100, 1250, 1400],
            "activePower": 18.2
        }),
        _ => json!({
            "labels": ["1月", "2月", "3月", "4月", "5月", "6月"],
            "earnings": [1200, 1450, 1680, 1920, 2150, 2380],
            "hashrate": [800, 950, 1100, 1250, 1400, 1550],
            "activePower": 22.5
        }),
    };

    let response = ApiResponse::success(chart_data);
    Ok(Json(response))
}

// 获取资产统计图表
pub async fn get_asset_chart_data(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
    Query(params): Query<serde_json::Value>,
) -> Result<impl IntoResponse> {
    let chart_type = params
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("overview");

    let chart_data = match chart_type {
        "overview" => json!({
            "totalAssets": 12580.50,
            "currency": "USDT",
            "chartData": {
                "labels": ["1月", "2月", "3月", "4月", "5月", "6月"],
                "values": [8500, 9200, 10500, 11200, 12000, 12580.50]
            },
            "distribution": {
                "mining": { "amount": 8500.30, "percentage": 67.6 },
                "rewards": { "amount": 2580.20, "percentage": 20.5 },
                "referral": { "amount": 1500.00, "percentage": 11.9 }
            }
        }),
        "income" => json!({
            "labels": ["周一", "周二", "周三", "周四", "周五", "周六", "周日"],
            "datasets": [
                {
                    "label": "挖矿收益",
                    "data": [45.2, 52.8, 48.6, 61.3, 58.9, 72.4, 68.1],
                    "backgroundColor": "#10B981"
                },
                {
                    "label": "邀请奖励",
                    "data": [12.5, 15.3, 8.9, 18.2, 22.1, 25.8, 20.3],
                    "backgroundColor": "#3B82F6"
                }
            ],
            "totalIncome": 580.5,
            "growth": "+15.8%"
        }),
        _ => json!({
            "labels": ["充值", "提现", "兑换"],
            "datasets": [
                {
                    "label": "本月",
                    "data": [5000, 2000, 1500],
                    "backgroundColor": "#8B5CF6"
                },
                {
                    "label": "上月",
                    "data": [4200, 1800, 1200],
                    "backgroundColor": "#F59E0B"
                }
            ]
        }),
    };

    let response = ApiResponse::success(chart_data);
    Ok(Json(response))
}

// 获取实时数据
pub async fn get_realtime_data(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
) -> Result<impl IntoResponse> {
    let realtime_data = json!({
        "currentHashrate": 1250.8,
        "dailyEarnings": 85.6,
        "activePower": 18.5,
        "networkDifficulty": 1250000000000_i64,
        "blockReward": 6.25,
        "usdtPrice": 1.00,
        "btcPrice": 47580.50,
        "ethPrice": 2850.30,
        "lastUpdate": chrono::Utc::now().to_rfc3339()
    });

    let response = ApiResponse::success(realtime_data);
    Ok(Json(response))
}

// 获取排行榜数据
pub async fn get_leaderboard(
    State(_state): State<AppState>,
    Query(params): Query<serde_json::Value>,
) -> Result<impl IntoResponse> {
    let leaderboard_type = params
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("earnings");

    let leaderboard = match leaderboard_type {
        "hashrate" => json!({
            "rankings": [
                { "rank": 1, "username": "miner001", "hashrate": 5680.5, "power": 85.2 },
                { "rank": 2, "username": "crypto_king", "hashrate": 4250.8, "power": 68.9 },
                { "rank": 3, "username": "ai_miner_pro", "hashrate": 3850.2, "power": 62.1 },
                { "rank": 4, "username": "bit_digger", "hashrate": 3250.7, "power": 55.8 },
                { "rank": 5, "username": "hash_master", "hashrate": 2850.4, "power": 48.3 }
            ],
            "currentUser": { "rank": 15, "username": "current_user", "hashrate": 1250.8, "power": 18.5 }
        }),
        "referrals" => json!({
            "rankings": [
                { "rank": 1, "username": "inviter001", "referrals": 156, "rewards": 15600.0 },
                { "rank": 2, "username": "network_builder", "referrals": 128, "rewards": 12800.0 },
                { "rank": 3, "username": "community_leader", "referrals": 95, "rewards": 9500.0 },
                { "rank": 4, "username": "growth_hacker", "referrals": 78, "rewards": 7800.0 },
                { "rank": 5, "username": "social_influencer", "referrals": 65, "rewards": 6500.0 }
            ],
            "currentUser": { "rank": 8, "username": "current_user", "referrals": 15, "rewards": 1500.0 }
        }),
        _ => json!({
            "rankings": [
                { "rank": 1, "username": "top_earner", "earnings": 158650.50, "joinDate": "2024-01-15" },
                { "rank": 2, "username": "profit_master", "earnings": 125890.30, "joinDate": "2024-02-20" },
                { "rank": 3, "username": "crypto_whale", "earnings": 98520.80, "joinDate": "2024-03-10" },
                { "rank": 4, "username": "bitcoin_pro", "earnings": 85680.40, "joinDate": "2024-04-05" },
                { "rank": 5, "username": "ai_miner", "earnings": 72450.60, "joinDate": "2024-05-12" }
            ],
            "currentUser": { "rank": 25, "username": "current_user", "earnings": 5680.50, "joinDate": "2024-11-01" }
        }),
    };

    let response = ApiResponse::success(leaderboard);
    Ok(Json(response))
}

// 获取K线数据
pub async fn get_kline_data(
    State(_state): State<AppState>,
    Query(params): Query<serde_json::Value>,
) -> Result<impl IntoResponse> {
    let symbol = params
        .get("symbol")
        .and_then(|v| v.as_str())
        .unwrap_or("BTCUSDT");
    let interval = params
        .get("interval")
        .and_then(|v| v.as_str())
        .unwrap_or("1h");

    let kline_data = json!({
        "symbol": symbol,
        "interval": interval,
        "data": [
            {
                "timestamp": 1700000000000_i64,
                "open": 42000.5,
                "high": 42500.0,
                "low": 41800.0,
                "close": 42350.5,
                "volume": 1250.8
            },
            {
                "timestamp": 1700003600000_i64,
                "open": 42350.5,
                "high": 42800.0,
                "low": 42100.0,
                "close": 42680.0,
                "volume": 980.3
            },
            {
                "timestamp": 1700007200000_i64,
                "open": 42680.0,
                "high": 43200.0,
                "low": 42450.0,
                "close": 42950.0,
                "volume": 1150.6
            }
        ]
    });

    let response = ApiResponse::success(kline_data);
    Ok(Json(response))
}

// 获取价格信息
pub async fn get_price_info(
    State(_state): State<AppState>,
    Query(params): Query<serde_json::Value>,
) -> Result<impl IntoResponse> {
    let symbol = params
        .get("symbol")
        .and_then(|v| v.as_str())
        .unwrap_or("BTCUSDT");

    let price_info = json!({
        "symbol": symbol,
        "price": 42950.0,
        "priceChange": 650.5,
        "priceChangePercent": 1.54,
        "high24h": 43200.0,
        "low24h": 41800.0,
        "volume24h": 3381.7,
        "timestamp": chrono::Utc::now().timestamp_millis()
    });

    let response = ApiResponse::success(price_info);
    Ok(Json(response))
}

// 获取交易对信息
pub async fn get_trading_pairs(State(_state): State<AppState>) -> Result<impl IntoResponse> {
    let trading_pairs = json!({
        "symbols": [
            {
                "symbol": "BTCUSDT",
                "baseAsset": "BTC",
                "quoteAsset": "USDT",
                "status": "TRADING",
                "price": 42950.0,
                "priceChangePercent": 1.54
            },
            {
                "symbol": "ETHUSDT",
                "baseAsset": "ETH",
                "quoteAsset": "USDT",
                "status": "TRADING",
                "price": 2280.5,
                "priceChangePercent": 2.18
            },
            {
                "symbol": "BNBUSDT",
                "baseAsset": "BNB",
                "quoteAsset": "USDT",
                "status": "TRADING",
                "price": 315.8,
                "priceChangePercent": -0.85
            }
        ]
    });

    let response = ApiResponse::success(trading_pairs);
    Ok(Json(response))
}

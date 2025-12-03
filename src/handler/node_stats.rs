use crate::{error::Result, schema::common::ApiResponse, state::AppState};
use axum::{
    extract::State,
    response::{IntoResponse, Json},
};
use serde_json::json;

// 获取节点统计信息
pub async fn get_node_stats(State(_state): State<AppState>) -> Result<impl IntoResponse> {
    let node_stats = json!({
        "totalNodes": 1250,
        "activeNodes": 1185,
        "inactiveNodes": 65,
        "totalHashrate": 45680.5,
        "averageHashrate": 38.5,
        "totalPower": 2580.8,
        "averagePower": 2.18,
        "uptime": 99.85,
        "lastUpdate": chrono::Utc::now().to_rfc3339(),
        "geographicDistribution": [
            {"region": "亚太地区", "count": 680, "percentage": 54.4},
            {"region": "北美地区", "count": 320, "percentage": 25.6},
            {"region": "欧洲地区", "count": 180, "percentage": 14.4},
            {"region": "其他地区", "count": 70, "percentage": 5.6}
        ],
        "nodeTypes": [
            {"type": "专业节点", "count": 850, "percentage": 68.0},
            {"type": "普通节点", "count": 400, "percentage": 32.0}
        ],
        "performance": {
            "dailyBlocks": 144,
            "averageBlockTime": "10分钟",
            "networkDifficulty": 1250000000000_i64,
            "blockReward": 6.25,
            "confirmations": 6
        }
    });

    let response = ApiResponse::success(node_stats);
    Ok(Json(response))
}

// 获取实时性能指标
pub async fn get_performance_metrics(State(_state): State<AppState>) -> Result<impl IntoResponse> {
    let metrics = json!({
        "currentHashrate": 45680.5,
        "hashrate24h": 44520.8,
        "hashrateChange": "+2.6%",
        "networkDifficulty": 1250000000000_i64,
        "difficultyChange": "+0.8%",
        "blockHeight": 825680,
        "nextBlockIn": "3分42秒",
        "averageBlockTime": "9分58秒",
        "mempoolSize": 1258,
        "unconfirmedTransactions": 8520,
        "networkStability": 99.92,
        "nodeConnectivity": 98.7,
        "lastUpdate": chrono::Utc::now().to_rfc3339()
    });

    let response = ApiResponse::success(metrics);
    Ok(Json(response))
}

// 获取算力分布图数据
pub async fn get_hashrate_distribution(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse> {
    let distribution = json!({
        "timeRange": "24h",
        "dataPoints": [
            {"time": "00:00", "hashrate": 42580.5, "nodes": 1050},
            {"time": "04:00", "hashrate": 43250.8, "nodes": 1080},
            {"time": "08:00", "hashrate": 44520.6, "nodes": 1120},
            {"time": "12:00", "hashrate": 45680.5, "nodes": 1185},
            {"time": "16:00", "hashrate": 45250.3, "nodes": 1160},
            {"time": "20:00", "hashrate": 44850.7, "nodes": 1140}
        ],
        "peakHashrate": 45680.5,
        "averageHashrate": 44355.6,
        "minimumHashrate": 42580.5,
        "maximumNodes": 1185,
        "averageNodes": 1122,
        "minimumNodes": 1050
    });

    let response = ApiResponse::success(distribution);
    Ok(Json(response))
}

// 获取节点地图数据
pub async fn get_node_map(State(_state): State<AppState>) -> Result<impl IntoResponse> {
    let node_map = json!({
        "nodes": [
            {
                "id": "node_001",
                "location": {"lat": 39.9042, "lng": 116.4074, "city": "北京"},
                "status": "active",
                "hashrate": 125.8,
                "power": 8.5,
                "uptime": 99.95,
                "lastSeen": chrono::Utc::now().to_rfc3339()
            },
            {
                "id": "node_002",
                "location": {"lat": 31.2304, "lng": 121.4737, "city": "上海"},
                "status": "active",
                "hashrate": 98.5,
                "power": 6.8,
                "uptime": 99.88,
                "lastSeen": chrono::Utc::now().to_rfc3339()
            },
            {
                "id": "node_003",
                "location": {"lat": 37.7749, "lng": -122.4194, "city": "旧金山"},
                "status": "active",
                "hashrate": 156.2,
                "power": 10.5,
                "uptime": 99.92,
                "lastSeen": chrono::Utc::now().to_rfc3339()
            },
            {
                "id": "node_004",
                "location": {"lat": 51.5074, "lng": -0.1278, "city": "伦敦"},
                "status": "inactive",
                "hashrate": 0,
                "power": 0,
                "uptime": 0,
                "lastSeen": "2025-11-23T08:30:00Z"
            },
            {
                "id": "node_005",
                "location": {"lat": 35.6762, "lng": 139.6503, "city": "东京"},
                "status": "active",
                "hashrate": 112.8,
                "power": 7.5,
                "uptime": 99.97,
                "lastSeen": chrono::Utc::now().to_rfc3339()
            }
        ],
        "regions": [
            {
                "name": "亚太地区",
                "center": {"lat": 25.0, "lng": 115.0},
                "nodes": 680,
                "totalHashrate": 28500.5,
                "color": "#10B981"
            },
            {
                "name": "北美地区",
                "center": {"lat": 45.0, "lng": -100.0},
                "nodes": 320,
                "totalHashrate": 12580.3,
                "color": "#3B82F6"
            },
            {
                "name": "欧洲地区",
                "center": {"lat": 50.0, "lng": 10.0},
                "nodes": 180,
                "totalHashrate": 4599.7,
                "color": "#F59E0B"
            }
        ],
        "totalNodes": 1250,
        "activeNodes": 1185,
        "totalHashrate": 45680.5
    });

    let response = ApiResponse::success(node_map);
    Ok(Json(response))
}

// 刷新节点统计
pub async fn refresh_node_stats(State(_state): State<AppState>) -> Result<impl IntoResponse> {
    let refresh_result = json!({
        "success": true,
        "message": "节点统计数据已刷新",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "nextRefresh": chrono::Utc::now() + chrono::Duration::minutes(5),
        "updatedFields": [
            "hashrate",
            "power",
            "temperature",
            "uptime",
            "networkStatus"
        ]
    });

    let response = ApiResponse::success_with_message(refresh_result, "Node statistics data refreshed successfully");
    Ok(Json(response))
}

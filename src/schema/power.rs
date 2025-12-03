use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct PowerPackageRequest {
    pub power_id: i64,
    pub quantity: i32,
    pub blockchain_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PowerPackageResponse {
    pub id: i64,
    pub task_type: String,
    pub required_level: i32,
    pub earnings_percent: f64,
    pub amount: f64,
    pub currency: String,
    pub description: String,
    pub duration: String,
    pub features: Vec<String>,
    pub daily_earnings: f64,
    pub total_earnings: f64,
}

/// 算力包响应数据
#[derive(Debug, Serialize, Deserialize)]
pub struct PowerPackageItem {
    pub id: u64,
    pub title: String,               // varchar(50) NOT NULL
    pub lv: u16,                     // varchar(255) NOT NULL
    pub daily_yield_percentage: f64, // tinyint unsigned NOT NULL
    pub amount: f64,
    pub description: String,
    pub is_upgrade: bool,
    pub status: i8,
    pub sort_order: u32,
}

/// 算力包列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct PowerPackagesResponse {
    pub packages: Vec<PowerPackageItem>,
    pub total_count: u32,
}

/// 算力记录响应数据
#[derive(Debug, Serialize, Deserialize)]
pub struct PowerRecord {
    pub id: u64,
    pub power_package_id: u64,
    pub order_id: String,
    pub types: i16, // 1 赠送or 0 购买
    pub amount: f64,
    pub start_time: Option<OffsetDateTime>,
    pub status: i16,   // 0 no-pay 1 active, 2 cancelled
    pub earnings: f64, //计收益金额
    pub title: String,
    pub lv: u16,
    pub daily_yield_percentage: f64,
    pub description: String,
}

/// 算力记录列表响应
#[derive(Debug, Serialize, Deserialize)]
pub struct PowerRecordsResponse {
    pub records: Vec<PowerRecord>,
    pub pagination: PowerRecordsPagination,
}

/// 算力记录分页信息
#[derive(Debug, Serialize, Deserialize)]
pub struct PowerRecordsPagination {
    pub page: u32,
    pub limit: u32,
    pub total: u64,
    pub total_pages: u32,
}

fn default_sort_field() -> String {
    "created_at".to_string()
}

fn default_sort_direction() -> String {
    "desc".to_string()
}

/// 算力统计响应
#[derive(Debug, Serialize, Deserialize)]
pub struct PowerStatsResponse {
    pub total_records: u64,
    pub active_records: u64,
    pub total_invested: rust_decimal::Decimal,
    pub total_earnings: rust_decimal::Decimal,
    pub total_hashrate: rust_decimal::Decimal,
    pub today_earnings: Option<rust_decimal::Decimal>,
}

pub struct OrderRequest {
    pub power_id: i64,
    pub num: u64,
}

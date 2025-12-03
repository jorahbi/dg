use bigdecimal::ToPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::types::{Json, JsonValue};
use sqlx::FromRow;
use time::OffsetDateTime;
use crate::schema::power::PowerPackageItem;

#[derive(Debug, Clone, FromRow)]
pub struct PowerPackage {
    pub id: u64,
    #[sqlx(json)]
    pub title: Json<JsonValue>, // varchar(50) NOT NULL
    pub lv: u16,                         // varchar(255) NOT NULL
    pub daily_yield_percentage: Decimal, // tinyint unsigned NOT NULL
    pub amount: Decimal,
    #[sqlx(json)]
    pub description: Json<JsonValue>,
    pub status: i8,
    pub is_upgrade: i8,
    pub sort_order: u32,
    pub created_at: OffsetDateTime, // timestamp NOT NULL
    pub updated_at: OffsetDateTime, // timestamp NOT NULL
}

pub const POWER_PACKAGE_STATUS_NO_UPGRADE: i8 = 0;
pub const POWER_PACKAGE_STATUS_UPGRADE: i8 = 1;

///0 no-pay 1 active, 2 cancelled, 3 upgrade
pub const USER_POWER_RECORD_STATUS_NO_PAY: i16 = 0;
pub const USER_POWER_RECORD_STATUS_ACTIVE: i16 = 1;
pub const USER_POWER_RECORD_STATUS_CANCELED: i16 = 2;
pub const USER_POWER_RECORD_STATUS_UPGRADE: i8 = 3;

#[derive(Debug, Clone, FromRow)]
pub struct UserPowerDetail {
    pub id: u64,
    pub power_package_id: u64,
    pub order_id: String,
    pub types: i16, // 1 赠送or 0 购买
    pub amount: Option<Decimal>,
    pub start_time: Option<OffsetDateTime>,
    pub status: i16,       // 0 no-pay 1 active, 2 cancelled
    pub earnings: Decimal, // 累计收益金额
    pub title: Option<JsonValue>,
    pub lv: Option<u16>,
    pub daily_yield_percentage: Option<Decimal>,
    pub description: Option<JsonValue>,
}

#[derive(Debug, Clone, FromRow)]
pub struct UserPower {
    pub id: u64,
    pub user_id: u64,
    pub power_package_id: u64,
    pub daily_yield_percentage: Decimal,
    pub order_id: String,
    pub types: i32,
    pub lv: i16,
    pub amount: Decimal,
    pub start_time: Option<OffsetDateTime>,
    pub status: i16,       // 0 no-pay 1 active, 2 cancelled
    pub earnings: Decimal, // 累计收益金额
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, FromRow)]
pub struct UserPowerRecord {
    pub id: u64,
    pub user_id: u64,
    pub power_package_id: u64,
    pub user_power_id: u64,
    pub lv: i16,
    pub daily_yield_percentage: Decimal,
    pub close_price: Decimal,
    pub package_amount: Decimal,
    pub amount: Decimal,
    pub create_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserPower {
    pub status: Option<String>,
    pub earnings: Option<Decimal>,
    pub current_hashrate: Option<Decimal>,
    pub total_hashrate: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawalRequest {
    pub user_id: i64,
    pub amount: Decimal,
    pub currency: String,
    pub destination_address: String,
}

// 引入 schema 中的 PowerRecord
use crate::schema::power::PowerRecord;

/// 从 UserPower 转换为 PowerRecord，处理多语言 JSON 字段
pub fn convert_user_power_record_to_power_record(
    record: UserPowerDetail,
    lang: &str,
) -> PowerRecord {
    let title_str = match record.title {
        Some(json_title) => extract_localized_string(&json_title, lang),
        None => "Unknown Package".to_string(),
    };

    let description_str = match record.description {
        Some(json_desc) => extract_localized_string(&json_desc, lang),
        None => "No description available".to_string(),
    };

    PowerRecord {
        id: record.id,
        power_package_id: record.power_package_id,
        order_id: record.order_id,
        types: record.types,
        amount: record.amount.and_then(|bd| bd.to_f64()).unwrap_or(0.0),
        start_time: record.start_time,
        status: record.status,
        earnings: record.earnings.to_f64().unwrap_or(0.0),
        title: title_str,
        lv: record.lv.unwrap_or(0),
        daily_yield_percentage: record
            .daily_yield_percentage
            .and_then(|bd| bd.to_f64())
            .unwrap_or(0.0),
        description: description_str,
    }
}

/// 从 JSON 值中提取指定语言的文本
fn extract_localized_string(json_value: &JsonValue, lang: &str) -> String {
    match json_value {
        Value::String(s) => s.clone(),
        Value::Object(map) => {
            // 优先使用指定语言，回退到英文，再回退到第一个值
            if let Some(value) = map.get(lang) {
                extract_string_value(value)
            } else if let Some(value) = map.get("en") {
                extract_string_value(value)
            } else if let Some(value) = map.values().next() {
                extract_string_value(value)
            } else {
                String::new()
            }
        }
        _ => String::new(),
    }
}

/// 从 JSON 值中提取字符串
fn extract_string_value(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        _ => value.to_string(),
    }
}

/// 将 Vec<UserPower> 转换为 Vec<PowerRecord>
pub fn convert_user_power_records(
    records: Vec<UserPowerDetail>,
    lang: &str,
) -> Vec<PowerRecord> {
    records
        .into_iter()
        .map(|record| convert_user_power_record_to_power_record(record, lang))
        .collect()
}

/// 从 PowerPackage 转换为 PowerPackageItem，处理多语言 JSON 字段
pub fn convert_power_package_to_package_item(
    package: PowerPackage,
    lang: &str,
) -> PowerPackageItem {
    let title_str = extract_localized_string(&package.title, lang);
    let description_str = extract_localized_string(&package.description, lang);

    PowerPackageItem {
        id: package.id,
        title: title_str,
        lv: package.lv,
        daily_yield_percentage: package.daily_yield_percentage.to_f64().unwrap_or(0.0),
        amount: package.amount.to_f64().unwrap_or(0.0),
        description: description_str,
        status: package.status,
        is_upgrade: if package.is_upgrade > 0 { true } else { false },
        sort_order: package.sort_order,
    }
}

/// 将 Vec<PowerPackage> 转换为 Vec<PowerPackageItem>
pub fn convert_power_packages(packages: Vec<PowerPackage>, lang: &str) -> Vec<PowerPackageItem> {
    packages
        .into_iter()
        .map(|package| convert_power_package_to_package_item(package, lang))
        .collect()
}

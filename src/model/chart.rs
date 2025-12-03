use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct KlineData {
    pub id: i64,
    pub symbol: String,
    pub interval_type: String, // 1m, 5m, 15m, 30m, 1h, 4h, 1d, 1w
    pub timestamp: i64,
    pub open_price: rust_decimal::Decimal,
    pub high_price: rust_decimal::Decimal,
    pub low_price: rust_decimal::Decimal,
    pub close_price: rust_decimal::Decimal,
    pub volume: rust_decimal::Decimal,
    pub amount: rust_decimal::Decimal,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TradingPair {
    pub symbol: String,
    pub base_currency: String,
    pub quote_currency: String,
    pub precision_amount: i32,
    pub precision_price: i32,
    pub supported_time_ranges: serde_json::Value,
    pub is_active: bool,
    pub min_trade_amount: rust_decimal::Decimal,
    pub max_trade_amount: rust_decimal::Decimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PriceInfo {
    pub symbol: String,
    pub current_price: rust_decimal::Decimal,
    pub price_change: rust_decimal::Decimal,
    pub price_change_percent: rust_decimal::Decimal,
    pub high_24h: rust_decimal::Decimal,
    pub low_24h: rust_decimal::Decimal,
    pub volume_24h: rust_decimal::Decimal,
    pub last_update: NaiveDateTime,
    pub market_cap: Option<rust_decimal::Decimal>,
    pub circulating_supply: Option<rust_decimal::Decimal>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewKlineData {
    pub symbol: String,
    pub interval_type: String,
    pub timestamp: i64,
    pub open_price: rust_decimal::Decimal,
    pub high_price: rust_decimal::Decimal,
    pub low_price: rust_decimal::Decimal,
    pub close_price: rust_decimal::Decimal,
    pub volume: rust_decimal::Decimal,
    pub amount: rust_decimal::Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePriceInfo {
    pub current_price: rust_decimal::Decimal,
    pub price_change: rust_decimal::Decimal,
    pub price_change_percent: rust_decimal::Decimal,
    pub high_24h: rust_decimal::Decimal,
    pub low_24h: rust_decimal::Decimal,
    pub volume_24h: rust_decimal::Decimal,
    pub market_cap: Option<rust_decimal::Decimal>,
    pub circulating_supply: Option<rust_decimal::Decimal>,
}

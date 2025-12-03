use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Earning {
    pub id: i64,
    pub user_id: i64,
    pub date: chrono::NaiveDate,
    pub amount: rust_decimal::Decimal,
    pub source_id: String, // mining, referral, task, staking, airdrop
    pub source_name: String,
    pub color: String,
    pub status: String, // confirmed, pending, failed, cancelled
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RechargeRecord {
    pub id: i64,
    pub user_id: i64,
    pub r#type: String,
    pub amount: rust_decimal::Decimal,
    pub transaction_hash: Option<String>,
    pub address: String,
    pub confirmations: i32,
    pub fee: rust_decimal::Decimal,
    pub status: String, // pending, completed, failed
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ConversionRecord {
    pub id: i64,
    pub user_id: i64,
    pub from_currency: String,
    pub to_currency: String,
    pub from_amount: rust_decimal::Decimal,
    pub to_amount: rust_decimal::Decimal,
    pub exchange_rate: rust_decimal::Decimal,
    pub fee: rust_decimal::Decimal,
    pub transaction_id: String,
    pub status: String, // completed, failed, pending
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WithdrawalRecord {
    pub id: String,
    pub user_id: i64,
    pub amount: rust_decimal::Decimal,
    pub currency: String,
    pub blockchain_code: String,
    pub address: String,
    pub fee: rust_decimal::Decimal,
    pub status: String, // pending, processing, completed, cancelled, failed
    pub transaction_hash: Option<String>,
    pub processed_time: Option<NaiveDateTime>,
    pub estimated_processing_time: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SupportedBlockchain {
    pub code: String,
    pub name: String,
    pub fee: rust_decimal::Decimal,
    pub min_amount: rust_decimal::Decimal,
    pub max_amount: rust_decimal::Decimal,
    pub icon: String,
    pub confirmation_time: String,
    pub is_available: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewEarning {
    pub user_id: i64,
    pub date: chrono::NaiveDate,
    pub amount: rust_decimal::Decimal,
    pub source_id: String,
    pub source_name: String,
    pub status: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewRechargeRecord {
    pub user_id: i64,
    pub r#type: String,
    pub amount: rust_decimal::Decimal,
    pub transaction_hash: Option<String>,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewConversionRecord {
    pub user_id: i64,
    pub from_currency: String,
    pub to_currency: String,
    pub from_amount: rust_decimal::Decimal,
    pub to_amount: rust_decimal::Decimal,
    pub exchange_rate: rust_decimal::Decimal,
    pub fee: rust_decimal::Decimal,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewWithdrawalRecord {
    pub id: String,
    pub user_id: i64,
    pub amount: rust_decimal::Decimal,
    pub currency: String,
    pub blockchain_code: String,
    pub address: String,
    pub fee: rust_decimal::Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRequest {
    pub from_currency: String,
    pub to_currency: String,
    pub amount: rust_decimal::Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawRequest {
    pub amount: String,
    pub blockchain_code: String,
    pub address: String,
}

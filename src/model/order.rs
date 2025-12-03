use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use rust_decimal::Decimal;

///订单状态：0 pending待支付/1 paid已支付/2 cancelled已取消/3 已升级
pub const ORDER_STATUS_PENDING: i8 = 0;
pub const ORDER_STATUS_PAID: i8 = 1;
pub const ORDER_STATUS_CANCELLED: i8 = 2;
pub const ORDER_STATUS_UPGRADE: i8 = 3;
#[derive(Debug, Clone, FromRow)]
pub struct Order {
    pub id: u64,
    pub order_id: String,
    pub user_id: u64,
    pub power_package_id: u64,
    pub quantity: u32,
    pub amount: Decimal, // 使用 sqlx 的 BigDecimal 类型
    pub asset_pay: Decimal, // 使用 sqlx 的 BigDecimal 类型
    pub coin_pay: Decimal, // 使用 sqlx 的 BigDecimal 类型
    pub blockchain_type: String,
    pub blockchain_address: String,
    pub transaction_hash: Option<String>,
    pub status: i8,  // pending, paid, completed, cancelled, expired
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewOrder {
    pub id: String,
    pub order_number: String,
    pub user_id: i64,
    pub package_id: i64,
    pub quantity: i32,
    pub amount: rust_decimal::Decimal,
    pub currency: String,
    pub blockchain_type: String,
    pub blockchain_address: String,
    pub expired_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrder {
    pub transaction_hash: Option<String>,
    pub status: Option<String>,
    pub paid_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseOrderRequest {
    pub power_id: i64,
    pub quantity: i32,
}

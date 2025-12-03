use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateOrderRequest {
    #[serde(rename = "powerId")]
    #[validate(range(min = 1, message = "Computing power package ID incorrect"))]
    pub power_id: u64,
    #[serde(alias = "blockchainType")]
    #[validate(length(min = 2, message = "Chain type incorrect"))]
    pub blockchain_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderResponse {
    #[serde(alias = "orderNumber")]
    pub order_number: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateOrderStatusRequest {
    #[serde(rename = "orderId")]
    #[validate(length(min = 1, message = "Order ID cannot be empty"))]
    pub order_id: String,

    /// 新的订单状态
    #[serde(rename = "status")]
    #[validate(range(min = 0, message = "Order status cannot be empty"))]
    pub status: i8,

    #[serde(rename = "userId")]
    #[validate(range(min = 0, message = "user_id incorrect"))]
    pub user_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateOrderStatusResponse {
    pub order_id: String,
    pub new_status: i8,
    pub updated_at: i64, // 毫秒时间戳
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpgradeOrderRequest {
    #[serde(rename = "oldUserPowerId")]
    #[validate(range(min = 1, message = "Original computing power package cannot be empty"))]
    pub old_user_power_id: u64,

    /// 升级的算力包id
    #[serde(rename = "powerId")]
    #[validate(range(min = 1, message = "Upgrade computing power package cannot be empty"))]
    pub power_id: u64,

    #[serde(alias = "blockchainType")]
    #[validate(length(min = 2, message = "Chain type incorrect"))]
    pub blockchain_type: String,
}

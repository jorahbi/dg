use crate::utils::time_zone::TimeZone;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use strum::{Display, EnumString};
use time::OffsetDateTime;
use derive_builder::Builder;
use crate::utils::gen::generate_no;



#[derive(Debug, Clone, FromRow)]
#[derive(Builder)]
pub struct Transactions {
    /// 交易记录ID，主键
    #[builder(default = 0)]
    pub id: u64,
    /// 用户ID，外键关联users表
    #[builder(default = 0)]
    pub user_id: u64,
    /// 交易唯一标识，系统生成的交易ID，唯一索引
    #[builder(default = generate_no("T"))]
    pub transaction_id: String,
    /// 交易类型：充值/提现/兑换/购买/空投/邀请/任务收益/挖矿收益
    #[builder(default = String::new())]
    pub types: String,
    /// 源货币代码，兑换交易的转出货币，如BTC、USDT等
    #[builder(default = String::new())]
    pub from_currency: String,
    /// 目标货币代码，兑换交易的转入货币，如ETH、DG等
    #[builder(default = String::new())]
    pub to_currency: String,
    /// 交易金额，8位小数精度，交易的主要数值
    #[builder(default = Decimal::ZERO)]
    pub amount: Decimal,
    /// 交易手续费，8位小数精度，平台收取的服务费用
    #[builder(default = Decimal::ZERO)]
    pub fee: Decimal,
    /// 兑换汇率，用于币种兑换交易，表示两个货币之间的兑换比例
    #[builder(default = Decimal::ZERO)]
    pub exchange_rate: Decimal,
    /// 交易状态：pending待处理/processing处理中/completed已完成/failed失败/cancelled已取消
    #[builder(default = String::new())]
    pub status: String,
    /// 区块链类型，如TRC20、ERC20等，用于链上交易
    #[builder(default = String::new())]
    pub blockchain_type: String,
    /// 转出地址，充值或提现的区块链钱包地址
    #[builder(default = String::new())]
    pub from_address: String,
    /// 转入地址，充值或提现的区块链钱包地址
    #[builder(default = String::new())]
    pub to_address: String,
    /// 交易描述，用户友好的交易说明
    #[builder(default = String::new())]
    pub description: String,
    /// 交易完成时间，null表示未完成
    #[builder(default = None)]
    pub completed_at: Option<OffsetDateTime>,
    /// 交易创建时间，自动创建时间戳
    #[builder(default = TimeZone::Beijing.get_time())]
    pub created_at: OffsetDateTime,
    /// 交易元数据JSON，存储额外的交易相关信息
    #[builder(default = None)]
    pub metadata: Option<Vec<u8>>,
    /// 交易最后更新时间，自动更新时间戳
    #[builder(default = TimeZone::Beijing.get_time())]
    pub updated_at: OffsetDateTime,
}

/// 交易类型：提现/兑换/购买/撤消购买/空投/邀请/挖矿收益/福利
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum OrderType {
    ///提现
    Withdraw,
    ///兑换
    Exchange,
    ///购买
    Purchase,
    ///撤消购买
    CancelPurchase,
    ///空投
    Airdrop,
    ///邀请
    Referral,
    ///挖矿收益
    MiningEarning,
    ///福利
    Welcome,
}

/// 交易状态：pending待处理/processing处理中/completed已完成/failed失败/cancelled已取消
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum OrderStatus {
    Cancelled,
    Completed,
    Failed,
    Pending,
    Processing,
}

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use strum::{Display, EnumString};
use time::OffsetDateTime;

#[derive(Debug, Clone, FromRow)]
pub struct SystemConfig {
    pub id: u64,
    pub config_key: String,
    pub config_value: String,
    pub description: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Serialize, Deserialize)]
#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct ConfigLevel {
    pub recharge: i32,
    pub lv: u8,
}

#[derive(Display, EnumString, Debug, PartialEq)]
#[strum(serialize_all = "snake_case")]
pub enum SystemConfigType {
    Blockchain,
    WelcomeBonus,
    UpgradeProgress,
}

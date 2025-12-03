use crate::model::system_config::SystemConfig;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfigRequest {
    pub config_key: String,
    pub config_value: String,
    pub config_type: String,
    pub description: Option<String>,
    pub is_encrypted: Option<i8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfigResponse {
    pub id: u64,
    pub config_key: String,
    pub config_value: String,
    pub config_type: String,
    pub description: Option<String>,
    pub is_encrypted: i8,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SystemConfigCreateRequest {
    #[validate(length(min = 1, message = "Configuration key cannot be empty"))]
    pub config_key: String,
    #[validate(length(min = 1, message = "Configuration value cannot be empty"))]
    pub config_value: String,
    pub description: Option<String>,
}

impl From<SystemConfig> for SystemConfigResponse {
    fn from(config: SystemConfig) -> Self {
        Self {
            id: config.id,
            config_key: config.config_key,
            config_value: config.config_value,
            config_type: "string".to_string(), // 默认类型
            description: config.description,
            is_encrypted: 0, // 默认不加密
        }
    }
}

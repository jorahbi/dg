use std::collections::HashMap;

use crate::{
    error::Result,
    model::system_config::{SystemConfig, SystemConfigType},
    repository::system_config_repo::SystemConfigRepo,
    state::AppState,
    AppError,
};

/// 系统配置服务
pub struct SystemConfigService {
    db: sqlx::MySqlPool,
}

impl SystemConfigService {
    pub fn new(state: &AppState) -> Self {
        Self {
            db: (*state.db).clone(),
        }
    }

    /// 创建系统配置
    pub async fn create_config(
        &self,
        config_key: &str,
        config_value: &str,
        description: Option<&str>,
    ) -> Result<u64> {
        // 验证配置键是否已存在
        if SystemConfigRepo::exists_config_key(
            &self.db, config_key, None, // 排除当前编辑的配置
        )
        .await?
        {
            return Err(crate::error::AppError::Conflict(format!(
                "Configuration key '{}' already exists",
                config_key
            )));
        }

        let config_id =
            SystemConfigRepo::create_config(&self.db, config_key, config_value, description)
                .await?;

        Ok(config_id)
    }

    /// 根据配置键获取系统配置
    pub async fn get_config_by_key(&self, config_key: &str) -> Result<SystemConfig> {
        let config = SystemConfigRepo::get_config_by_key(&self.db, config_key)
            .await?;

        Ok(config)
    }

    /// 更新系统配置
    pub async fn update_config(
        &self,
        key: &str,
        config_value: &str,
        description: Option<&str>,
    ) -> Result<()> {
        SystemConfigRepo::update_config(&self.db, key, config_value, description).await?;

        // (|| crate::error::AppError::Internal("Failed to update configuration".to_string()))?;

        Ok(())
    }

    /// 删除系统配置
    pub async fn delete_config(&self, key: &str) -> Result<String> {
        SystemConfigRepo::delete_config(&self.db, key).await?;

        Ok("Configuration deleted successfully".to_string())
    }

    pub async fn get_chain_addr(&self, types: &str) -> Result<String> {
        let config = self
            .get_config_by_key(SystemConfigType::Blockchain.to_string().as_str())
            .await?;
        let mut chain: HashMap<String, String> = serde_json::from_str(&config.config_value)?;
        let Some(addr) = chain.remove(types) else {
            return Err(AppError::Validation(format!("Unsupported payment type: {}", types)));
        };

        Ok(addr)
    }
}

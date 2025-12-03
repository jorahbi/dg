use crate::utils::time_zone::TimeZone;
use crate::{error::Result, model::system_config::SystemConfig, AppError};
use sqlx::{MySql, Pool};
use time::OffsetDateTime;

/// 系统配置仓库
pub struct SystemConfigRepo;

impl SystemConfigRepo {
    /// 创建系统配置
    pub async fn create_config(
        pool: &Pool<MySql>,
        config_key: &str,
        config_value: &str,
        description: Option<&str>,
    ) -> Result<u64> {
        // let config_value_json = serde_json::to_value(config_value);
        let curr_time = TimeZone::Beijing.get_time();
        let result = sqlx::query!(
            r#"
            INSERT INTO system_configs (
                config_key, config_value, description, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?)
            "#,
            config_key,
            config_value,
            description,
            curr_time,
            curr_time
        )
        .execute(pool)
        .await?;

        Ok(result.last_insert_id())
    }

    /// 根据配置键获取系统配置
    pub async fn get_config_by_key(pool: &Pool<MySql>, config_key: &str) -> Result<SystemConfig> {
        let config = sqlx::query_as!(
            SystemConfig,
            r#"
            SELECT id, config_key as "config_key: String", config_value as "config_value: String", description as "description: String",
            created_at, updated_at
            FROM system_configs
            WHERE config_key = ?
            "#,
            config_key
        )
        .fetch_optional(pool)
        .await?;

        match config {
            Some(config) => Ok(config),
            None => Err(AppError::NotFound(format!(
                "System config key {} not found",
                config_key
            ))),
        }
    }

    /// 获取所有系统配置
    pub async fn get_all_configs(
        pool: &Pool<MySql>,
        page: u32,
        limit: u32,
        offset: u32,
    ) -> Result<(Vec<SystemConfig>, u64)> {
        let offset = (page - 1) * limit;

        // 获取总记录数
        let total_result = sqlx::query_scalar!("SELECT COUNT(*) as count FROM system_configs")
            .fetch_one(pool)
            .await?;

        let total = total_result as u64;

        // 获取分页数据
        let configs = sqlx::query_as!(
            SystemConfig,
            r#"
            SELECT id, config_key as "config_key: String", config_value  as "config_value: String", description  as "description: String", created_at, updated_at
            FROM system_configs
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        Ok((configs, total))
    }

    /// 更新系统配置
    pub async fn update_config(
        pool: &Pool<MySql>,
        key: &str,
        config_value: &str,
        description: Option<&str>,
    ) -> Result<()> {
        let now = TimeZone::Beijing.get_time();

        sqlx::query!(
            r#"
            UPDATE system_configs
            SET config_value = ?, description = ?, updated_at = ?
            WHERE config_key = ?
            "#,
            config_value,
            description,
            now,
            key
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// 删除系统配置
    pub async fn delete_config(pool: &Pool<MySql>, key: &str) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM system_configs WHERE config_key = ?
            "#,
            key
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// 检查配置键是否存在
    pub async fn exists_config_key(
        pool: &Pool<MySql>,
        config_key: &str,
        exclude_id: Option<u64>,
    ) -> Result<bool> {
        let query = if let Some(exclude_id) = exclude_id {
            sqlx::query_scalar!(
                r#"
                SELECT COUNT(*) FROM system_configs
                WHERE config_key = ? AND id != ?
                "#,
                config_key,
                exclude_id
            )
        } else {
            sqlx::query_scalar!(
                r#"
                SELECT COUNT(*) FROM system_configs
                WHERE config_key = ?
                "#,
                config_key
            )
        };

        let count = query.fetch_one(pool).await?;
        Ok(count > 0)
    }
}

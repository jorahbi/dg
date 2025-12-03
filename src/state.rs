use sqlx::MySqlPool;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::config::Config;
use crate::websocket::hub::WsHub;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: Arc<MySqlPool>,
    pub ws_hub: Arc<RwLock<WsHub>>,
    pub cron_scheduler: Arc<crate::cron::scheduler::CronSchedulerManager>,
}

impl AppState {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        // 创建数据库连接池
        let db = Arc::new(
            sqlx::mysql::MySqlPoolOptions::new()
                .max_connections(config.database.max_connections)
                .min_connections(config.database.min_connections)
                .acquire_timeout(std::time::Duration::from_secs(
                    config.database.connection_timeout,
                ))
                .idle_timeout(std::time::Duration::from_secs(config.database.idle_timeout))
                .max_lifetime(std::time::Duration::from_secs(config.database.max_lifetime))
                .connect(&config.database.url)
                .await?,
        );

        // 创建WebSocket Hub
        let ws_hub = Arc::new(RwLock::new(WsHub::new()));

        // 创建定时任务调度器
        let cron_scheduler = Arc::new(crate::cron::scheduler::CronSchedulerManager::new());

        Ok(Self {
            config: Arc::new(config),
            db,
            ws_hub,
            cron_scheduler,
        })
    }

    pub async fn health_check(&self) -> bool {
        // 检查数据库连接
        if let Err(e) = sqlx::query("SELECT 1")
            .fetch_one(self.db.as_ref())
            .await
        {
            tracing::error!("Database health check failed: {:?}", e);
            return false;
        }

        true
    }
}
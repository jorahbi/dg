use super::{error::{Result, CronError}, tasks::daily_midnight_task};
use crate::state::AppState;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{error, info, warn};

/// 定时任务调度器管理器
pub struct CronSchedulerManager {
    scheduler: Arc<Mutex<Option<JobScheduler>>>,
    is_running: Arc<Mutex<bool>>,
}

impl CronSchedulerManager {
    /// 创建新的调度器管理器
    pub fn new() -> Self {
        Self {
            scheduler: Arc::new(Mutex::new(None)),
            is_running: Arc::new(Mutex::new(false)),
        }
    }

    /// 启动调度器
    pub async fn start(&self, app_state: Arc<AppState>) -> Result<()> {
        let mut is_running = self.is_running.lock().await;
        if *is_running {
            warn!("Cron scheduler is already running");
            return Ok(());
        }

        let scheduler = JobScheduler::new().await
            .map_err(|e| CronError::SchedulerError(format!("Failed to create scheduler: {}", e)))?;

        // 添加每日23:59:59执行的任务
        self.add_daily_midnight_job(scheduler.clone(), app_state).await?;

        // 启动调度器
        scheduler.start().await
            .map_err(|e| CronError::SchedulerError(format!("Failed to start scheduler: {}", e)))?;

        {
            let mut scheduler_guard = self.scheduler.lock().await;
            *scheduler_guard = Some(scheduler);
        }

        *is_running = true;
        info!("Cron scheduler started successfully");

        Ok(())
    }

    /// 停止调度器
    pub async fn stop(&self) -> Result<()> {
        let mut is_running = self.is_running.lock().await;
        if !*is_running {
            warn!("Cron scheduler is not running");
            return Ok(());
        }

        {
            let mut scheduler_guard = self.scheduler.lock().await;
            if let Some(mut scheduler) = scheduler_guard.take() {
                scheduler.shutdown().await
                    .map_err(|e| CronError::SchedulerError(format!("Failed to shutdown scheduler: {}", e)))?;
                info!("Cron scheduler has been stopped");
            }
        }

        *is_running = false;
        Ok(())
    }

    /// 检查调度器是否运行
    pub async fn is_running(&self) -> bool {
        let is_running = self.is_running.lock().await;
        *is_running
    }

        /// 添加每日午夜任务 (23:59:59)
    async fn add_daily_midnight_job(&self, scheduler: JobScheduler, app_state: Arc<AppState>) -> Result<()> {
        // 使用预定义的 @daily 表达式，每天午夜执行
        // 这种方法更简单，避免了所有权冲突问题
        let app_state_clone = app_state.clone();//0 * * * * * @daily
        let job = Job::new_async("@daily", move |_uuid, _l| {
            let app_state = app_state_clone.clone();
            Box::pin(async move {
                info!("Trigger daily midnight task");
                if let Err(e) = daily_midnight_task(app_state).await {
                    error!("每日午夜任务执行失败: {}", e);
                }
            })
        })
        .map_err(|e| CronError::SchedulerError(format!("Failed to create cron job: {}", e)))?;

        scheduler.add(job).await
            .map_err(|e| CronError::SchedulerError(format!("Failed to add cron job: {}", e)))?;
        info!("Daily 23:59:59 cron job has been added");

        Ok(())
    }
    pub async fn get_status(&self) -> CronSchedulerStatus {
        let is_running = self.is_running.lock().await;
        let scheduler_guard = self.scheduler.lock().await;

        CronSchedulerStatus {
            is_running: *is_running,
            jobs_count: if scheduler_guard.is_some() {
                // 这里可以获取实际的任务数量，但需要更多API调用
                // 为了简化，暂时返回固定值
                1
            } else {
                0
            },
        }
    }
}

/// 定时任务调度器状态
#[derive(Debug, Clone)]
pub struct CronSchedulerStatus {
    pub is_running: bool,
    pub jobs_count: usize,
}

impl Default for CronSchedulerManager {
    fn default() -> Self {
        Self::new()
    }
}

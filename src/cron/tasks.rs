use crate::model::transactions::TransactionsBuilder;
use crate::model::USER_POWER_RECORD_STATUS_ACTIVE;
use crate::repository::power_repo::PowerRepo;
use crate::utils::time_zone::TimeZone;
use crate::{error::AppError, state::AppState};
use dashmap::DashMap;
use rust_decimal::Decimal;
use sqlx::QueryBuilder;
use std::sync::Arc;
use time::Duration;
use tracing::{error, info};

/// 每日23:59:59执行的定时任务
pub async fn daily_midnight_task(
    state: Arc<AppState>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let now = TimeZone::Beijing.get_time();
    info!("Starting daily midnight task - time: {}", now);

    // 执行具体的业务逻辑
    if let Err(e) = execute_daily_tasks(state.clone()).await {
        error!("Daily task execution failed: {}", e);
        return Err(Box::new(e));
    }

    let end_time = TimeZone::Beijing.get_time();
    let duration = end_time - now;
    info!("Daily midnight task execution completed - duration: {:?}", duration);

    Ok(())
}

/// 执行每日具体的任务
async fn execute_daily_tasks(state: Arc<AppState>) -> Result<(), AppError> {
    // 2. 更新用户每日收益统计
    update_daily_earnings(&state).await?;
    Ok(())
}

/// 更新用户每日收益统计
async fn update_daily_earnings(state: &AppState) -> Result<(), AppError> {
    info!("Starting to update user daily earnings statistics");
    // 减少2小时减少误差
    let curr = TimeZone::Beijing.get_time() - Duration::hours(2);
    let user_power =
        PowerRepo::get_power_record_by_time(&state.db, USER_POWER_RECORD_STATUS_ACTIVE, &curr)
            .await;
    let user_power = match user_power {
        Ok(user_power) => user_power,
        Err(err) => {
            error!("Failed to get all active power records for cron task: {}", err);
            return Err(err);
        }
    };
    if user_power.len() == 0 {
        return Ok(());
    }
    let mut qb = QueryBuilder::<sqlx::MySql>::new(
        r#"
            INSERT INTO user_power_record (
                user_id, user_power_id, power_package_id, lv, daily_yield_percentage, close_price,
                package_amount, amount, created_at)
            "#,
    );
    let curr_time = TimeZone::Beijing.get_time();
    let closing_price = Decimal::from(2);
    let amount_hub: Arc<DashMap<u64, Decimal>> = Arc::new(DashMap::new());
    qb.push_values(&user_power, |mut b, power| {
        let percent = Decimal::from(100);
        let amount = (power.amount * power.daily_yield_percentage) / percent;
        let real = amount / (closing_price / percent);
        b.push_bind(power.user_id)
            .push_bind(power.id)
            .push_bind(power.power_package_id)
            .push_bind(power.lv)
            .push_bind(power.daily_yield_percentage)
            .push_bind(&closing_price)
            .push_bind(power.amount)
            .push_bind(real)
            .push_bind(&curr_time);
        let am = amount_hub.clone();
        am.entry(power.user_id)
            .and_modify(|v| *v += real)
            .or_insert(real);
    });
    qb.push("ON DUPLICATE KEY UPDATE created_at = VALUES(created_at)");
    let query = qb.build();
    query.execute(state.db.as_ref()).await?;

    Ok(())
}

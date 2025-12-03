use crate::model::{User, USER_POWER_RECORD_STATUS_ACTIVE, USER_POWER_RECORD_STATUS_UPGRADE};
use crate::utils::time_zone::TimeZone;
use crate::{
    error::Result,
    model::{PowerPackage, UserPower, UserPowerDetail},
    AppError,
};
use rust_decimal::Decimal;
use sqlx::{MySql, MySqlConnection, Pool};
use time::OffsetDateTime;

/// 算力记录仓库
pub struct PowerRepo;

impl PowerRepo {
    /// 获取用户的算力记录列表（分页）
    pub async fn get_user_power_records(
        pool: &Pool<MySql>,
        user_id: u64,
        page: u32,
        limit: u32,
    ) -> Result<(Vec<UserPowerDetail>, u64)> {
        let offset = (page - 1) * limit;

        // 获取总记录数
        let total_result = sqlx::query_scalar!(
            "SELECT COUNT(*) as count FROM user_power WHERE user_id = ?",
            user_id
        )
        .fetch_one(pool)
        .await?;

        let total = total_result as u64;
        // 获取分页数据
        let records = sqlx::query_as!(
            UserPowerDetail,
            r#"
            SELECT
                ur.id, ur.power_package_id, order_id as "order_id: String", ur.types, ur.amount, ur.start_time,
                ur.status, ur.earnings, pp.title, pp.lv, pp.daily_yield_percentage, pp.description
            FROM user_power as ur
            LEFT JOIN power_packages as pp ON ur.power_package_id = pp.id
            WHERE ur.user_id = ?
            ORDER BY ur.created_at DESC
            LIMIT ? OFFSET ?
            "#,
            user_id,
            limit,
            offset
        )
            .fetch_all(pool)
            .await?;

        Ok((records, total))
    }

    /// 根据ID获取算力记录详情
    pub async fn get_power_record_by_id(
        pool: &Pool<MySql>,
        record_id: u64,
    ) -> Result<Option<PowerPackage>> {
        let record = sqlx::query_as!(
            PowerPackage,
            r#"
            SELECT id, title, lv, daily_yield_percentage, amount, description,
                status, updated_at, sort_order, created_at, is_upgrade
            FROM power_packages
            WHERE id = ?
            "#,
            record_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(record)
    }

    /// 根据userID获取算力记录
    pub async fn get_power_record_by_user(
        pool: &Pool<MySql>,
        user_id: u64,
        status: i8,
    ) -> Result<Vec<UserPower>> {
        let record = sqlx::query_as!(
            UserPower,
            r#"
            SELECT id, user_id, power_package_id, order_id as "order_id: String", types, amount,
            start_time, status, earnings, created_at, updated_at, lv, daily_yield_percentage
            FROM user_power
            WHERE user_id = ? AND status = ?
            "#,
            user_id,
            status
        )
        .fetch_all(pool)
        .await?;

        Ok(record)
    }

    /// 根据userID获取算力记录
    pub async fn get_power_record_by_time(
        pool: &Pool<MySql>,
        status: i16,
        time: &OffsetDateTime,
    ) -> Result<Vec<UserPower>> {
        println!("-----{}", time.date().to_string());
        let record = sqlx::query_as!(
            UserPower,
            r#"
            SELECT id, user_id, power_package_id, order_id as "order_id: String", types, amount,
            start_time, status, earnings, created_at, updated_at, lv, daily_yield_percentage
            FROM user_power
            WHERE status = ? AND start_time > ? ORDER BY user_id
            "#,
            status,
            time.date()
        )
        .fetch_all(pool)
        .await?;

        Ok(record)
    }

    /// 根据ID和用户id获取算力记录详情
    pub async fn get_power_record_by_id_and_user(
        pool: &Pool<MySql>,
        record_id: u64,
        user_id: u64,
    ) -> Result<UserPower> {
        let record = sqlx::query_as!(
            UserPower,
            r#"
            SELECT id, user_id, power_package_id, order_id as "order_id: String", types, amount,
            start_time, status, earnings, created_at, updated_at, lv, daily_yield_percentage
            FROM user_power
            WHERE id = ? AND user_id = ?
            "#,
            record_id,
            user_id
        )
        .fetch_optional(pool)
        .await?;
        match record {
            Some(record) => Ok(record),
            None => {
                return Err(AppError::NotFound(
                    "Computing power package not found".to_string(),
                ))
            }
        }
    }

    /// 获取所有可用的算力包
    pub async fn get_all_power_packages(pool: &Pool<MySql>) -> Result<Vec<PowerPackage>> {
        let packages = sqlx::query_as!(
            PowerPackage,
            r#"
            SELECT
                id, title, lv, daily_yield_percentage, amount, description,
                status, updated_at, sort_order, created_at, is_upgrade
            FROM power_packages
            WHERE status = 1
            ORDER BY sort_order ASC, lv ASC
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(packages)
    }

    /// 创建用户算力记录（在事务中执行）
    pub async fn create_user_power_record(
        tx: &mut MySqlConnection,
        user_id: u64,
        power_package_id: u64,
        order_no: &str,
        power_type: i16, // 1 赠送 or 0 购买
        amount: &Decimal,
        state: i16,
        lv: u16,
        daily_yield_percentage: &Decimal,
    ) -> Result<u64> {
        let result = sqlx::query!(
            r#"
            INSERT INTO user_power(
                user_id, power_package_id, order_id, types, amount, status, earnings, lv, daily_yield_percentage
            ) VALUES (?, ?, ?, ?, ?, ?, 0, ?, ?)
            "#,
            user_id,
            power_package_id,
            order_no,
            power_type,
            amount,
            state,
            lv,
            daily_yield_percentage
        )
            .execute(&mut *tx)
            .await?;

        Ok(result.last_insert_id())
    }

    /// 升级用户算力（在事务中执行）
    pub async fn upgrade_user_power_record(
        tx: &mut MySqlConnection,
        user_id: u64,
        user_power_id: u64,
    ) -> Result<u64> {
        let result = sqlx::query!(
            r#"
            UPDATE user_power
            SET status = ?, updated_at = ? WHERE user_id = ? AND id = ?
            "#,
            USER_POWER_RECORD_STATUS_UPGRADE,
            TimeZone::Beijing.get_time(),
            user_id,
            user_power_id
        )
        .execute(&mut *tx)
        .await?;

        Ok(result.last_insert_id())
    }

    /// 算力加速
    pub async fn start_user_power_record(
        pool: &Pool<MySql>,
        user_id: u64,
        user_power_id: u64,
    ) -> Result<u64> {
        let result = sqlx::query!(
            r#"
            UPDATE user_power SET start_time = ? WHERE user_id = ? AND id = ? AND status = ?
            "#,
            TimeZone::Beijing.get_time(),
            user_id,
            user_power_id,
            USER_POWER_RECORD_STATUS_ACTIVE
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
    }

    /// 获取当日开启开速的算力
    pub async fn get_start_all_power_record(
        pool: &Pool<MySql>,
        current_time: &OffsetDateTime,
    ) -> Result<Vec<UserPower>> {
        let record = sqlx::query_as!(
            UserPower,
            r#"
            SELECT id, user_id, power_package_id, order_id as "order_id: String", types, amount,
            start_time, status, earnings, created_at, updated_at, lv, daily_yield_percentage
            FROM user_power
            WHERE start_time > ?
            "#,
            current_time
        )
        .fetch_all(pool)
        .await?;
        Ok(record)
    }

    /// 获取当日收益
    pub async fn get_daily_power_record(
        pool: &Pool<MySql>,
        current_time: &OffsetDateTime,
        user_id: u64,
    ) -> Result<Decimal> {
        let record = sqlx::query!(
            r#"
            SELECT sum(amount) as total FROM user_power_record WHERE created_at = ? AND user_id = ?
            "#,
            current_time.date(),
            user_id
        )
        .fetch_one(pool)
        .await?;
        let Some(total) = record.total else {
            return Ok(Decimal::ZERO);
        };
        Ok(total)
    }
}

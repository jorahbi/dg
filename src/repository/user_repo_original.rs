use crate::{
    error::Result,
    model::{User, SecurityQuestion, UserSecurityAnswer},
    state::AppState,
};
use sqlx::{MySqlPool, Row};

pub struct UserRepo;

impl UserRepo {
    // 根据用户名查找用户
    pub async fn find_by_username(pool: &MySqlPool, username: &str) -> Result<Option<User>> {
        // 暂时返回None，避免SQLX编译错误
        // TODO: 实现真正的数据库查询
        Ok(None)
    }

    // 根据邮箱查找用户
    pub async fn find_by_email(pool: &MySqlPool, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, nickname, avatar, phone, status,
                   is_email_verified, is_kyc_verified, kyc_level, invite_code, inviter_id,
                   total_assets, dg_amount, user_level, login_attempts, locked_until,
                   last_login_at, created_at, updated_at
            FROM users
            WHERE email = ?
            "#,
            email
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    // 根据ID查找用户
    pub async fn find_by_id(pool: &MySqlPool, user_id: i64) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, nickname, avatar, phone, status,
                   is_email_verified, is_kyc_verified, kyc_level, invite_code, inviter_id,
                   total_assets, dg_amount, user_level, login_attempts, locked_until,
                   last_login_at, created_at, updated_at
            FROM users
            WHERE id = ?
            "#,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    // 创建新用户
    pub async fn create(pool: &MySqlPool, user: &crate::model::user::NewUser) -> Result<i64> {
        let result = sqlx::query!(
            r#"
            INSERT INTO users (username, email, password_hash, nickname, invite_code, inviter_id)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            user.username,
            user.email,
            user.password_hash,
            user.nickname,
            user.invite_code,
            user.inviter_id
        )
        .execute(pool)
        .await?;

        Ok(result.last_insert_id() as i64)
    }

    // 更新用户信息
    pub async fn update(pool: &MySqlPool, user_id: i64, update: &crate::model::user::UpdateUser) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE users
            SET nickname = COALESCE(?, nickname),
                avatar = COALESCE(?, avatar),
                phone = COALESCE(?, phone),
                is_email_verified = COALESCE(?, is_email_verified),
                is_kyc_verified = COALESCE(?, is_kyc_verified),
                kyc_level = COALESCE(?, kyc_level),
                user_level = COALESCE(?, user_level),
                updated_at = NOW(3)
            WHERE id = ?
            "#,
            update.nickname,
            update.avatar,
            update.phone,
            update.is_email_verified,
            update.is_kyc_verified,
            update.kyc_level,
            update.user_level,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    // 更新密码
    pub async fn update_password(pool: &MySqlPool, user_id: i64, password_hash: &str) -> Result<()> {
        sqlx::query!(
            "UPDATE users SET password_hash = ?, updated_at = NOW(3) WHERE id = ?",
            password_hash,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    // 更新最后登录时间
    pub async fn update_last_login(pool: &MySqlPool, user_id: i64) -> Result<()> {
        sqlx::query!(
            "UPDATE users SET last_login_at = NOW(3) WHERE id = ?",
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    // 增加登录失败次数
    pub async fn increment_login_attempts(pool: &MySqlPool, user_id: i64) -> Result<()> {
        sqlx::query!(
            "UPDATE users SET login_attempts = login_attempts + 1 WHERE id = ?",
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    // 重置登录失败次数
    pub async fn reset_login_attempts(pool: &MySqlPool, user_id: i64) -> Result<()> {
        sqlx::query!(
            "UPDATE users SET login_attempts = 0, locked_until = NULL WHERE id = ?",
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    // 锁定账户
    pub async fn lock_account(pool: &MySqlPool, user_id: i64, lock_duration_hours: i32) -> Result<()> {
        sqlx::query!(
            "UPDATE users SET locked_until = NOW(3) + INTERVAL ? HOUR WHERE id = ?",
            lock_duration_hours,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    // 检查用户名是否存在
    pub async fn username_exists(pool: &MySqlPool, username: &str) -> Result<bool> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) as count FROM users WHERE username = ?",
            username
        )
        .fetch_one(pool)
        .await?;

        Ok(count.unwrap_or(0) > 0)
    }

    // 检查邮箱是否存在
    pub async fn email_exists(pool: &MySqlPool, email: &str) -> Result<bool> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) as count FROM users WHERE email = ?",
            email
        )
        .fetch_one(pool)
        .await?;

        Ok(count.unwrap_or(0) > 0)
    }

    // 验证邀请码
    pub async fn validate_invite_code(pool: &MySqlPool, invite_code: &str) -> Result<Option<i64>> {
        let user_id = sqlx::query_scalar!(
            "SELECT id FROM users WHERE invite_code = ? AND status = 1",
            invite_code
        )
        .fetch_optional(pool)
        .await?;

        Ok(user_id)
    }

    // 获取安全问题列表
    pub async fn get_security_questions(pool: &MySqlPool) -> Result<Vec<SecurityQuestion>> {
        let questions = sqlx::query_as!(
            SecurityQuestion,
            r#"
            SELECT id, question, is_active, sort_order, created_at, updated_at
            FROM security_questions
            WHERE is_active = true
            ORDER BY sort_order
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(questions)
    }

    // 获取用户安全问题答案
    pub async fn get_user_security_answers(pool: &MySqlPool, user_id: i64) -> Result<Vec<UserSecurityAnswer>> {
        let answers = sqlx::query_as!(
            UserSecurityAnswer,
            r#"
            SELECT id, user_id, question_id, answer_hash, created_at, updated_at
            FROM user_security_answers
            WHERE user_id = ?
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(answers)
    }

    // 保存用户安全问题答案
    pub async fn save_security_answer(pool: &MySqlPool, answer: &crate::model::user::NewSecurityAnswer) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO user_security_answers (user_id, question_id, answer_hash)
            VALUES (?, ?, ?)
            ON DUPLICATE KEY UPDATE answer_hash = VALUES(answer_hash), updated_at = NOW(3)
            "#,
            answer.user_id,
            answer.question_id,
            answer.answer_hash
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    // 删除用户安全问题答案
    pub async fn delete_security_answers(pool: &MySqlPool, user_id: i64) -> Result<()> {
        sqlx::query!(
            "DELETE FROM user_security_answers WHERE user_id = ?",
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    // 检查用户是否设置了安全问题
    pub async fn has_security_questions(pool: &MySqlPool, user_id: i64) -> Result<bool> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) as count FROM user_security_answers WHERE user_id = ?",
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(count.unwrap_or(0) > 0)
    }

    // 获取用户的安全问题（不包含答案）
    pub async fn get_user_security_questions(pool: &MySqlPool, user_id: i64) -> Result<Vec<SecurityQuestion>> {
        let questions = sqlx::query_as!(
            SecurityQuestion,
            r#"
            SELECT sq.id, sq.question, sq.is_active, sq.sort_order, sq.created_at, sq.updated_at
            FROM security_questions sq
            INNER JOIN user_security_answers usa ON sq.id = usa.question_id
            WHERE usa.user_id = ? AND sq.is_active = true
            ORDER BY sq.sort_order
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(questions)
    }

    // 更新用户资产
    pub async fn update_assets(pool: &MySqlPool, user_id: i64, total_assets: rust_decimal::Decimal, dg_amount: rust_decimal::Decimal) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE users
            SET total_assets = ?, dg_amount = ?, updated_at = NOW(3)
            WHERE id = ?
            "#,
            total_assets,
            dg_amount,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    // 增加DG余额
    pub async fn add_dg_amount(pool: &MySqlPool, user_id: i64, amount: rust_decimal::Decimal) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE users
            SET dg_amount = dg_amount + ?, total_assets = total_assets + ?, updated_at = NOW(3)
            WHERE id = ?
            "#,
            amount,
            amount,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    // 减少DG余额
    pub async fn subtract_dg_amount(pool: &MySqlPool, user_id: i64, amount: rust_decimal::Decimal) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE users
            SET dg_amount = dg_amount - ?, total_assets = total_assets - ?, updated_at = NOW(3)
            WHERE id = ? AND dg_amount >= ?
            "#,
            amount,
            amount,
            user_id,
            amount
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    // 获取用户统计信息
    pub async fn get_user_stats(pool: &MySqlPool, user_id: i64) -> Result<Option<serde_json::Value>> {
        let stats = sqlx::query!(
            r#"
            SELECT
                u.total_assets,
                u.dg_amount,
                u.user_level,
                u.is_kyc_verified,
                u.invite_code,
                (SELECT COUNT(*) FROM invite_records WHERE inviter_id = u.id) as invite_count,
                (SELECT COUNT(*) FROMuser_powerWHERE user_id = u.id AND status = 'active') as active_power_count,
                (SELECT COUNT(*) FROM airdrop_records WHERE user_id = u.id AND status = 'success') as airdrop_success_count
            FROM users u
            WHERE u.id = ?
            "#,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        match stats {
            Some(s) => {
                let stats_json = serde_json::json!({
                    "total_assets": s.total_assets,
                    "dg_amount": s.dg_amount,
                    "user_level": s.user_level,
                    "is_kyc_verified": s.is_kyc_verified,
                    "invite_code": s.invite_code,
                    "invite_count": s.invite_count.unwrap_or(0),
                    "active_power_count": s.active_power_count.unwrap_or(0),
                    "airdrop_success_count": s.airdrop_success_count.unwrap_or(0)
                });
                Ok(Some(stats_json))
            }
            None => Ok(None),
        }
    }
}
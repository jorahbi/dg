use crate::{error::Result, model::User, AppError};
use chrono::{Duration, Utc};
use sqlx::{FromRow, MySqlConnection, MySqlPool};

use crate::model::user_security_questions::{PasswordResetTokens, SecurityQuestion};
use crate::model::Assets;
use rust_decimal::Decimal;

// 数据库查询结果结构体
#[derive(Debug, FromRow)]
pub struct UserLoginInfo {
    pub id: u64,
    pub username: String,
    pub password_hash: String,
    pub user_level: u8,        // tinyint unsigned
    pub is_kyc_verified: i8,   // tinyint(1)
    pub login_attempts: u8,    // tinyint unsigned
    pub total_assets: Decimal, // decimal(20,8)
    pub dg_amount: Decimal,
    pub locked_until: Option<time::OffsetDateTime>,
    pub last_login_at: Option<time::OffsetDateTime>,
    pub created_at: time::OffsetDateTime, // NOT NULL timestamp
}

#[derive(Debug, FromRow)]
pub struct InviterInfo {
    pub id: u64,
    pub inviter_id: u64,
    pub parent_inviter_id: u64,
}

#[derive(Debug, FromRow)]
pub struct LoginAttemptsInfo {
    pub login_attempts: i8,
}

#[derive(Debug, FromRow)]
pub struct UserPermissionInfo {
    pub user_level: u8,
    pub is_kyc_verified: i8,
}

pub struct UserRepo;

impl UserRepo {
    // 根据用户名查找用户
    pub async fn find_by_username(pool: &MySqlPool, username: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT
                id, username as "username: String", password_hash as "password_hash: String", user_level,
                invite_code as "invite_code: String", parent_inviter_id, inviter_id, total_assets, dg_amount,
                is_kyc_verified, has_security_questions, upgrade_progress,
                is_active, is_locked, login_attempts, locked_until,
                qr_code_url as "qr_code_url: String", created_at, updated_at, last_login_at
            FROM users
            WHERE username = ?
            "#,
            username
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    // 根据ID查找用户
    pub async fn find_by_id(pool: &MySqlPool, user_id: u64) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT
                id, username as "username: String", password_hash as "password_hash: String", user_level,
                invite_code as "invite_code: String", parent_inviter_id, inviter_id, total_assets, dg_amount,
                is_kyc_verified, has_security_questions, upgrade_progress,
                is_active, is_locked, login_attempts, locked_until,
                qr_code_url as "qr_code_url: String", created_at, updated_at, last_login_at
            FROM users
            WHERE id = ?
            "#,
            user_id
        )
        .fetch_optional(pool)
        .await?;
        let Some(user) = user else {
            return Err(AppError::NotFound(format!(
                "User with id {} not found",
                user_id
            )));
        };

        Ok(user)
    }

    // 验证邀请码
    pub async fn validate_invite_code(pool: &MySqlPool, invite_code: &str) -> Result<InviterInfo> {
        let inviter = sqlx::query_as!(
            InviterInfo,
            r#"
            SELECT id, inviter_id, parent_inviter_id
            FROM users
            WHERE invite_code = ?
            "#,
            invite_code
        )
        .fetch_optional(pool)
        .await?;

        let Some(inviter) = inviter else {
            return Err(AppError::NotFound(format!(
                "Inviter {} not found",
                invite_code
            )));
        };

        Ok(inviter)
    }

    // 生成唯一邀请码
    pub async fn generate_invite_code(pool: &MySqlPool) -> Result<String> {
        let mut attempts = 0;
        let max_attempts = 10;

        while attempts < max_attempts {
            // 生成8位随机邀请码（数字+字母）
            let code = format!("{:08X}", uuid::Uuid::new_v4().as_u128() % 100000000);
            let invite_code = code[0..8].to_string();

            // 检查邀请码是否已存在
            let existing = sqlx::query!("SELECT id FROM users WHERE invite_code = ?", invite_code)
                .fetch_optional(pool)
                .await?;

            if existing.is_none() {
                return Ok(invite_code);
            }

            attempts += 1;
        }

        Err(AppError::Internal(
            "Failed to generate invite code".to_string(),
        ))
    }

    // 创建用户
    pub async fn create_user(
        pool: &mut MySqlConnection,
        username: &str,
        password_hash: &str,
        user_invite_code: &str,
        inviter_id: u64,
        parent_inviter_id: u64,
        url: &str,
    ) -> Result<u64> {
        let user_id = sqlx::query!(
            r#"
            INSERT INTO users (
                username, password_hash, user_level,
                invite_code, inviter_id, parent_inviter_id, total_assets, dg_amount,
                is_kyc_verified, has_security_questions,
                is_active, is_locked, login_attempts, qr_code_url,
                created_at, updated_at
            ) VALUES (
                ?, ?, ?,
                ?, ?, ?, ?, ?,
                ?, ?,
                ?, ?, ?, ?,
                NOW(), NOW()
            )
            "#,
            username,
            password_hash,
            0i8,
            user_invite_code,
            inviter_id,
            parent_inviter_id,
            Decimal::ZERO,
            Decimal::ZERO,
            false,
            false,
            true,
            false,
            0i8,
            url,
        )
        .execute(pool)
        .await?
        .last_insert_id();

        Ok(user_id)
    }

    // 更新帐户资产
    pub async fn tx_update_assets(
        pool: &mut MySqlConnection,
        user_id: u64,
        assets: &Decimal,
        de_assets: &Decimal,
    ) -> Result<()> {
        sqlx::query!(
            "UPDATE users SET dg_amount = dg_amount+?, total_assets = total_assets+?, updated_at = NOW() WHERE id = ?",
            assets, user_id, de_assets
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    // 锁定用户账户
    pub async fn lock_user(
        pool: &MySqlPool,
        user_id: u64,
        locked_until: chrono::NaiveDateTime,
    ) -> Result<()> {
        sqlx::query(
            "UPDATE users SET is_locked = true, locked_until = ?, updated_at = NOW() WHERE id = ?",
        )
        .bind(locked_until)
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    // 解锁用户账户
    pub async fn unlock_user(pool: &MySqlPool, user_id: u64) -> Result<()> {
        sqlx::query("UPDATE users SET is_locked = false, locked_until = NULL, login_attempts = 0, updated_at = NOW() WHERE id = ?")
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    // 更新密码
    pub async fn update_password(
        pool: &MySqlPool,
        user_id: u64,
        password_hash: &str,
    ) -> Result<()> {
        sqlx::query("UPDATE users SET password_hash = ?, updated_at = NOW() WHERE id = ?")
            .bind(password_hash)
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    // 获取安全问题列表
    pub async fn get_security_questions(pool: &MySqlPool) -> Result<Vec<SecurityQuestion>> {
        let questions = sqlx::query_as!(
            SecurityQuestion,
            r#"
            SELECT id, question as "question: String", is_active, sort_order
            FROM security_questions
            ORDER BY sort_order ASC
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(questions)
    }

    // 保存用户安全问题答案
    pub async fn save_security_answers(
        pool: &MySqlPool,
        user_id: u64,
        answers: &[(i64, String)], // (question_id, answer_hash)
    ) -> Result<()> {
        for (question_id, answer_hash) in answers {
            sqlx::query!(
                r#"
                INSERT INTO user_security_questions (user_id, question_id, answer_hash, created_at, updated_at)
                VALUES (?, ?, ?, NOW(3), NOW(3))
                ON DUPLICATE KEY UPDATE
                    answer_hash = VALUES(answer_hash),
                    updated_at = NOW(3)
                "#,
                user_id,
                *question_id as u32,
                answer_hash,
            )
            .execute(pool)
            .await?;
        }

        tracing::info!("成功保存用户{}的{}个安全问题答案", user_id, answers.len());
        Ok(())
    }

    // 在事务中保存用户安全问题答案
    pub async fn save_security_answers_in_tx(
        tx: &mut MySqlConnection,
        user_id: u64,
        answers: &[(i64, String)], // (question_id, answer_hash)
    ) -> Result<()> {
        for (question_id, answer_hash) in answers {
            sqlx::query!(
                r#"
                INSERT INTO user_security_questions (user_id, question_id, answer_hash, created_at, updated_at)
                VALUES (?, ?, ?, NOW(3), NOW(3))
                ON DUPLICATE KEY UPDATE
                    answer_hash = VALUES(answer_hash),
                    updated_at = NOW(3)
                "#,
                user_id,
                *question_id as u32,
                answer_hash,
            )
            .execute(&mut *tx)
            .await?;
        }

        tracing::info!(
            "在事务中成功保存用户{}的{}个安全问题答案",
            user_id,
            answers.len()
        );
        Ok(())
    }

    // 验证安全问题答案
    pub async fn verify_security_answer(
        pool: &MySqlPool,
        user_id: u64,
        question_id: u64,
        answer: &str,
    ) -> Result<bool> {
        // 查询该用户对应问题的答案哈希
        let result = sqlx::query!(
            r#"
            SELECT answer_hash as "answer_hash: String"
            FROM user_security_questions
            WHERE user_id = ? AND question_id = ?
            "#,
            user_id,
            question_id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(record) = result {
            // 使用bcrypt验证答案
            let is_valid = bcrypt::verify(answer, &record.answer_hash).map_err(|e| {
                tracing::error!(
                    "bcrypt verification failed: user_id={}, question_id={}, error={}",
                    user_id,
                    question_id,
                    e
                );
                crate::error::AppError::Internal("Answer verification error".to_string())
            })?;

            tracing::info!(
                "安全问题答案验证结果: user_id={}, question_id={}, is_valid={}",
                user_id,
                question_id,
                is_valid
            );

            Ok(is_valid)
        } else {
            tracing::warn!("未找到用户{}的问题{}的答案记录", user_id, question_id);
            Ok(false)
        }
    }

    // 检查用户是否设置了安全问题
    pub async fn check_user_has_security_questions(pool: &MySqlPool, user_id: u64) -> Result<bool> {
        // 查询用户的has_security_questions字段
        let user = sqlx::query!(
            "SELECT has_security_questions FROM users WHERE id = ?",
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(
            if user.map(|u| u.has_security_questions).unwrap_or(0i8) > 0 {
                true
            } else {
                false
            },
        )
    }

    // 更新用户的安全问题设置标志
    pub async fn update_user_security_questions_flag(pool: &MySqlPool, user_id: u64) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE users
            SET has_security_questions = 1, updated_at = NOW(3)
            WHERE id = ?
            "#,
            user_id
        )
        .execute(pool)
        .await?;

        tracing::info!("已更新用户{}的安全问题设置标志", user_id);
        Ok(())
    }

    // 在事务中更新用户的安全问题设置标志
    pub async fn update_user_security_questions_flag_in_tx(
        tx: &mut sqlx::MySqlConnection,
        user_id: u64,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE users
            SET has_security_questions = 1, updated_at = NOW(3)
            WHERE id = ?
            "#,
            user_id
        )
        .execute(&mut *tx)
        .await?;

        tracing::info!("在事务中已更新用户{}的安全问题设置标志", user_id);
        Ok(())
    }

    // 获取所有安全问题
    pub async fn find_all_security_questions(_pool: &MySqlPool) -> Result<Vec<SecurityQuestion>> {
        // 暂时返回空列表，等待安全问题表创建
        // TODO: 实现真正的数据库查询
        Ok(vec![])
    }

    // 获取用户的安全问题（不包含答案）
    pub async fn get_user_security_questions(
        pool: &MySqlPool,
        user_name: &str,
    ) -> Result<Vec<crate::schema::SecurityQuestionRes>> {
        // 查询用户设置的安全问题
        let rows = sqlx::query!(
            r#"
            SELECT sq.id, sq.question as "question: String"
            FROM security_questions sq
            LEFT JOIN user_security_questions usq ON sq.id = usq.question_id
            LEFT JOIN users on users.id = usq.user_id
            WHERE users.username = ? AND sq.is_active = 1
            ORDER BY sq.sort_order ASC
            "#,
            user_name
        )
        .fetch_all(pool)
        .await?;

        // 手动映射结果
        let questions: Vec<crate::schema::SecurityQuestionRes> = rows
            .into_iter()
            .map(|row| crate::schema::SecurityQuestionRes {
                id: row.id,
                question: row.question,
            })
            .collect();

        tracing::info!("获取用户{}的{}个安全问题", user_name, questions.len());
        Ok(questions)
    }

    // 存储密码重置令牌
    pub async fn store_password_reset_token(
        pool: &MySqlPool,
        username: &str,
        token: &str,
    ) -> Result<()> {
        // 使用bcrypt对令牌进行哈希
        let token_hash = bcrypt::hash(token, bcrypt::DEFAULT_COST)
            .map_err(|e| crate::error::AppError::PasswordHash(e))?;

        // 设置过期时间为15分钟后
        let expires_at = Utc::now() + Duration::hours(8) + Duration::minutes(15);

        // 删除该用户之前的所有未使用令牌
        sqlx::query!(
            r#"
            DELETE FROM password_reset_tokens
            WHERE username = ? AND is_used = 0 AND expires_at > NOW()
            "#,
            username
        )
        .execute(pool)
        .await?;

        // 插入新的重置令牌
        sqlx::query!(
            r#"
            INSERT INTO password_reset_tokens (username, token_hash, expires_at, ip_address)
            VALUES (?, ?, ?, ?)
            "#,
            username,
            token_hash,
            expires_at,
            "127.0.0.1"
        ) // TODO: 从请求中获取真实IP地址
        .execute(pool)
        .await?;

        tracing::info!("已为用户{}存储密码重置令牌", username);
        Ok(())
    }

    // 验证密码重置令牌
    pub async fn verify_password_reset_token(
        pool: &MySqlPool,
        username: &str,
        token: &str,
    ) -> Result<bool> {
        // 查询该用户的有效令牌
        let result = sqlx::query_as!(
            PasswordResetTokens,
            r#"
            SELECT id, username as "username: String", token_hash as "token_hash: String", expires_at, is_used,
            ip_address as "ip_address: String"
            FROM password_reset_tokens
            WHERE username = ? AND is_used = 0
            LIMIT 1
            "#,
            username
        )
        .fetch_optional(pool)
        .await?;

        if let Some(token_info) = result {
            // 检查令牌是否过期

            if token_info.expires_at.assume_utc() < time::OffsetDateTime::now_utc() {
                return Ok(false);
            }

            // 验证令牌哈希
            let is_valid = bcrypt::verify(token, &token_info.token_hash).map_err(|_| {
                crate::error::AppError::Internal("Token verification error".to_string())
            })?;

            Ok(is_valid)
        } else {
            Ok(false)
        }
    }

    // 标记密码重置令牌为已使用
    pub async fn mark_password_reset_token_used(
        pool: &MySqlPool,
        username: &str,
        token: &str,
    ) -> Result<()> {
        // 先验证令牌
        let is_valid = Self::verify_password_reset_token(pool, username, token).await?;
        if !is_valid {
            return Err(AppError::Auth(
                "Reset token is invalid or has expired".to_string(),
            ));
        }

        // 标记令牌为已使用
        sqlx::query!(
            r#"
            UPDATE password_reset_tokens
            SET is_used = 1
            WHERE username = ? AND is_used = 0
            LIMIT 1
            "#,
            username
        )
        .execute(pool)
        .await?;

        tracing::info!("已标记用户{}的密码重置令牌为已使用", username);
        Ok(())
    }

    // 在事务中更新密码
    pub async fn update_password_in_tx(
        tx: &mut MySqlConnection,
        user_id: u64,
        password_hash: &str,
    ) -> Result<()> {
        sqlx::query("UPDATE users SET password_hash = ?, updated_at = NOW() WHERE id = ?")
            .bind(password_hash)
            .bind(user_id)
            .execute(&mut *tx)
            .await?;

        Ok(())
    }

    // 在事务中解锁用户
    pub async fn unlock_user_in_tx(tx: &mut sqlx::MySqlConnection, user_id: u64) -> Result<()> {
        sqlx::query("UPDATE users SET is_locked = false, locked_until = NULL, login_attempts = 0, updated_at = NOW() WHERE id = ?")
            .bind(user_id)
            .execute(&mut *tx)
            .await?;

        Ok(())
    }

    // 在事务中更新用户资产
    pub async fn update_user_assets_in_tx(
        tx: &mut MySqlConnection,
        user_id: u64,
        total_assets_change: Decimal,
        old_assets: Decimal,
    ) -> Result<()> {
        sqlx::query!(
            r#"
        UPDATE users SET total_assets = ?, updated_at = NOW()
        WHERE id = ? AND total_assets = ?"#,
            total_assets_change,
            user_id,
            old_assets
        )
        .execute(tx)
        .await?;

        Ok(())
    }

    // 更新帐户资产
    pub async fn tx_update_lv(
        pool: &mut MySqlConnection,
        user_id: u64,
        lv: u8,
        upgrade_progress: &Decimal,
    ) -> Result<()> {
        sqlx::query!(
            "UPDATE users SET upgrade_progress = upgrade_progress+?, user_level = ?, updated_at = NOW() WHERE id = ?",
            upgrade_progress,lv, user_id
        )
            .execute(pool)
            .await?;

        Ok(())
    }

    // 更新用户的安全问题设置标志
    pub async fn get_assets(pool: &MySqlPool, user_id: u64) -> Result<Assets> {
        let asset = sqlx::query_as!(
            Assets,
            r#" SELECT upgrade_progress, user_level, dg_amount, dg_amount as daily_balance, total_assets FROM users WHERE id = ? "#,
            user_id
        )
        .fetch_optional(pool)
        .await?;
        let Some(asset) = asset else {
            return Err(AppError::NotFound("user assets not found".to_string()));
        };

        Ok(asset)
    }
}

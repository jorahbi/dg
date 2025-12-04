use crate::model::user_security_questions::SecurityQuestion;
use crate::schema::SecurityQuestionAnswerReq;
use crate::utils::{generate_qr_image, FileUploadService};
use crate::{
    error::Result, repository::user_repo::UserRepo, schema::user::LoginRes, state::AppState, Config,
};
use bcrypt::{hash, DEFAULT_COST};
use std::sync::Arc;

pub struct AuthService {
    db: sqlx::MySqlPool,
    cfg: Arc<Config>,
}

impl AuthService {
    // 获取用户的安全问题（根据用户名，用于忘记密码流程）
    pub async fn get_user_security_questions_by_username(
        &self,
        username: &str,
    ) -> Result<Vec<crate::schema::SecurityQuestionRes>> {
        // 1. 查找用户
        let user = UserRepo::find_by_username(&self.db, username)
            .await?
            .ok_or_else(|| {
                tracing::warn!("Failed to get security questions - User does not exist: {}", username);
                crate::error::AppError::Auth("User does not exist".to_string())
            })?;

        // 2. 检查用户是否设置了安全问题
        if user.has_security_questions == 0 {
            return Err(crate::error::AppError::Auth(
                "User has not set up security questions".to_string(),
            ));
        }
        let username = user.username;
        // 3. 获取用户的安全问题（不包含答案）
        UserRepo::get_user_security_questions(&self.db, &username).await
    }

    // 获取用户的安全问题（根据用户ID，用于用户查看自己的问题）
    pub(crate) async fn get_user_security_questions(
        &self,
        username: &str,
    ) -> Result<Vec<crate::schema::SecurityQuestionRes>> {
        UserRepo::get_user_security_questions(&self.db, username).await
    }
}

impl AuthService {
    pub fn new(state: &AppState) -> Self {
        Self {
            db: (*state.db).clone(),
            cfg: state.config.clone(),
        }
    }
    // 用户注册
    pub async fn register(&self, username: &str, password: &str, invite_code: &str) -> Result<i64> {
        // 1. 验证邀请码是否存在且有效
        let inviter = UserRepo::validate_invite_code(&self.db, invite_code).await?;

        // 2. 检查用户名是否已存在
        let existing_user = UserRepo::find_by_username(&self.db, username).await?;
        if existing_user.is_some() {
            return Err(crate::error::AppError::Conflict("Username already exists".to_string()));
        }

        // 3. 生成密码哈希和用户唯一邀请码
        let password_hash =
            hash(password, DEFAULT_COST).map_err(|e| crate::error::AppError::PasswordHash(e))?;
        let user_invite_code = UserRepo::generate_invite_code(&self.db).await?;
        //todo apk name
        let data = format!(
            "{}/assets/pkg/{}/xxx.apk",
            self.cfg.server.site_url, user_invite_code
        );
        let base_image = generate_qr_image(&data, Some(self.cfg.upload.qrcord_size)).await?;
        let fileinfo = FileUploadService::new(&self.cfg.upload)
            .process_file_upload(
                &format!("{}.png", user_invite_code),
                "qrcode",
                base_image.as_slice(),
            )
            .await?;

        // 4. 开始事务，因为需要执行多个写操作
        let mut tx = self.db.begin().await?;

        // 5. 创建用户记录
        let user_id = UserRepo::create_user(
            &mut *tx,
            username,
            &password_hash,
            &user_invite_code,
            inviter.id,
            inviter.inviter_id,
            &fileinfo.url,
        )
        .await?;
        // 9. 提交事务
        tx.commit().await?;

        tracing::info!("User registration successful: username={}, user_id={}", username, user_id);
        Ok(user_id as i64)
    }

    // 用户登录
    pub async fn login(&self, state: AppState, username: &str, password: &str) -> Result<LoginRes> {
        // 1. 查询用户信息
        let user = UserRepo::find_by_username(&self.db, username)
            .await?
            .ok_or_else(|| {
                tracing::warn!("Login failed - User does not exist: {}", username);
                crate::error::AppError::Auth("Username or password incorrect".to_string())
            })?;

        // 2. 检查账户是否被锁定
        if let Some(_locked_time) = user.locked_until {
            return Err(crate::error::AppError::Auth("Account has been locked".to_string()));
        }

        // 3. 验证密码
        let is_password_valid = bcrypt::verify(password, &user.password_hash).map_err(|e| {
            tracing::error!(
                "Password verification failed - bcrypt error: user_id={}, error={}",
                user.id,
                e
            );
            crate::error::AppError::Internal("Password verification error".to_string())
        })?;

        if !is_password_valid {
            return Err(crate::error::AppError::Auth("Username or password incorrect".to_string()));
        }
        let username = user.username;
        // 5. 生成JWT令牌
        let jwt_service = crate::utils::jwt::JwtService::new(state.config.jwt.clone());
        let token = jwt_service.generate_token(user.id, &username, user.user_level as i32)?;

        let has_security_questions = user.has_security_questions > 0;
        // 7. 返回登录响应
        Ok(LoginRes {
            username,
            token,
            has_security_questions,
            level: user.user_level,
        })
    }

    // 获取安全问题
    pub async fn get_security_questions(&self) -> Result<Vec<SecurityQuestion>> {
        UserRepo::get_security_questions(&self.db).await
    }

    // 验证安全问题答案
    pub async fn verify_security_answers(
        &self,
        id: u64,
        answers: Vec<(i64, String)>,
    ) -> Result<bool> {
        // 首先根据用户名找到用户ID
        let user = UserRepo::find_by_id(&self.db, id).await?;

        // 验证每个答案
        for (question_id, answer) in answers {
            if !UserRepo::verify_security_answer(
                &self.db,
                user.id as u64,
                question_id as u64,
                &answer,
            )
            .await?
            {
                return Ok(false);
            }
        }

        Ok(true)
    }

    // 重置密码
    pub async fn reset_password(
        &self,
        username: &str,
        new_password: &str,
        reset_token: &str,
    ) -> Result<()> {
        // 找到用户
        let user = UserRepo::find_by_username(&self.db, username)
            .await?
            .ok_or_else(|| crate::error::AppError::Auth("User does not exist".to_string()))?;
        let username = user.username;
        // 验证重置令牌
        let is_token_valid =
            UserRepo::verify_password_reset_token(&self.db, &username, reset_token).await?;
        if !is_token_valid {
            return Err(crate::error::AppError::Auth(
                "Reset token is invalid or has expired".to_string(),
            ));
        }

        // 生成新密码哈希
        let password_hash = bcrypt::hash(new_password, DEFAULT_COST)
            .map_err(|e| crate::error::AppError::PasswordHash(e))?;

        // 在事务中执行密码重置
        let mut tx = self.db.begin().await?;

        // 更新密码
        UserRepo::update_password_in_tx(&mut *tx, user.id, &password_hash).await?;

        // 标记重置令牌为已使用
        UserRepo::mark_password_reset_token_used(&self.db, &username, reset_token).await?;

        UserRepo::unlock_user_in_tx(&mut *tx, user.id).await?;

        // 提交事务
        tx.commit().await?;

        tracing::info!("User {} password reset successful", username);
        Ok(())
    }

    // 保存安全问题
    pub async fn save_security_questions(
        &self,
        id: u64,
        questions: Vec<SecurityQuestionAnswerReq>,
    ) -> Result<()> {
        // 对答案进行加密
        let mut answers_encrypted = Vec::new();
        for question in questions {
            // 使用 bcrypt 对答案进行哈希加密
            let answer_hash = bcrypt::hash(&question.answer, DEFAULT_COST)
                .map_err(|e| crate::error::AppError::PasswordHash(e))?;
            answers_encrypted.push((question.question_id, answer_hash));
        }

        // 开始事务，确保数据一致性
        let mut tx = self.db.begin().await?;
        // 转换id为u64
        let user_id = id;
        // 在事务中保存安全问题答案
        UserRepo::save_security_answers_in_tx(&mut *tx, user_id, &answers_encrypted).await?;

        // 在事务中更新用户表，标记已设置安全问题
        UserRepo::update_user_security_questions_flag_in_tx(&mut *tx, user_id).await?;

        // 提交事务
        tx.commit().await?;

        tracing::info!(
            "User {} successfully saved {} security questions",
            id,
            answers_encrypted.len()
        );
        Ok(())
    }

    // 检查安全问题
    pub async fn check_security_questions(&self, id: u64) -> Result<bool> {
        let user = UserRepo::find_by_id(&self.db, id).await?;

        UserRepo::check_user_has_security_questions(&self.db, user.id).await
    }

    // 忘记密码问题
    pub async fn forgot_password_questions(&self, id: u64) -> Result<Vec<SecurityQuestion>> {
        // 验证用户存在
        let _user = UserRepo::find_by_id(&self.db, id).await?;

        // 获取所有安全问题
        UserRepo::find_all_security_questions(&self.db).await
    }

    // 忘记密码验证
    pub async fn forgot_password_verify(
        &self,
        username: &str,
        answers: Vec<SecurityQuestionAnswerReq>,
    ) -> Result<String> {
        // 1. 根据用户名查找用户
        let user = UserRepo::find_by_username(&self.db, username)
            .await?
            .ok_or_else(|| {
                tracing::warn!("Forgot password verification failed - User does not exist: {}", username);
                crate::error::AppError::Auth("Username or security answer incorrect".to_string())
            })?;

        // 2. 检查用户是否设置了安全问题
        if user.has_security_questions == 0 {
            return Err(crate::error::AppError::Auth(
                "User has not set up security questions".to_string(),
            ));
        }

        // 3. 验证每个安全问题答案
        for question_answer in answers {
            let question_id = question_answer.question_id as u64;
            let answer = &question_answer.answer;

            // 使用bcrypt验证答案（答案在存储时已经过哈希）
            let is_valid =
                UserRepo::verify_security_answer(&self.db, user.id, question_id, answer).await?;

            if !is_valid {
                tracing::warn!(
                    "Forgot password verification failed - Answer incorrect: user_id={}, question_id={}",
                    user.id,
                    question_id
                );
                return Err(crate::error::AppError::Auth(
                    "Username or security answer incorrect".to_string(),
                ));
            }
        }

        // 4. 生成重置令牌
        let reset_token = crate::utils::password::PasswordService::generate_reset_token();

        // 5. 将重置令牌存储到数据库，并设置过期时间（15分钟）
        UserRepo::store_password_reset_token(&self.db, username, &reset_token).await?;

        tracing::info!(
            "Forgot password verification successful: user_id={}, username={}, reset_token={}",
            user.id,
            username,
            &reset_token[..8] // Only log first 8 characters to avoid exposing full token in logs
        );

        Ok(reset_token)
    }

    // 修改密码
    pub async fn change_password(
        &self,
        id: u64,
        old_password: &str,
        new_password: &str,
    ) -> Result<()> {
        // 找到用户并获取登录信息
        let user = UserRepo::find_by_id(&self.db, id).await?;

        // 验证旧密码
        let is_old_password_valid = bcrypt::verify(old_password, &user.password_hash)
            .map_err(|_| crate::error::AppError::Internal("Password verification error".to_string()))?;

        if !is_old_password_valid {
            return Err(crate::error::AppError::Auth("Current password incorrect".to_string()));
        }

        // 生成新密码哈希
        let new_password_hash = bcrypt::hash(new_password, DEFAULT_COST)
            .map_err(|e| crate::error::AppError::PasswordHash(e))?;

        // 更新密码
        UserRepo::update_password(&self.db, user.id, &new_password_hash).await?;

        Ok(())
    }

    // 用户登出
    pub async fn logout(&self, _user_id: u64) -> Result<()> {
        // 当前实现中登出主要是客户端删除token，服务端暂时不需要特殊处理
        // TODO: 如果需要实现token黑名单，可以在这里添加
        Ok(())
    }
}

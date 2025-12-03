use crate::config::JwtConfig;
use crate::error::{AppError, Result};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: u64, // 用户ID
    pub username: String,
    pub user_level: i32,
    pub exp: i64, // 过期时间戳
    pub iat: i64, // 签发时间戳
    pub iss: Option<HashSet<String>>,
    pub aud: Vec<String>,
}

pub struct JwtService {
    config: JwtConfig,
}

impl JwtService {
    pub fn new(config: JwtConfig) -> Self {
        Self { config }
    }

    pub fn generate_token(&self, user_id: u64, username: &str, user_level: i32) -> Result<String> {
        let now = Utc::now();
        let exp = now + Duration::seconds(self.config.expiration);

        let claims = Claims {
            sub: user_id,
            username: username.to_string(),
            user_level,
            exp: exp.timestamp(),
            iat: now.timestamp(),
            iss: self.config.issuer.clone(),
            aud: self.config.audience.clone(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.secret.as_ref()),
        )?;

        Ok(token)
    }

    pub fn generate_refresh_token(&self, user_id: u64) -> Result<String> {
        let now = Utc::now();
        let exp = now + Duration::seconds(self.config.refresh_expiration);

        let claims = Claims {
            sub: user_id,
            username: String::new(), // 刷新令牌不需要用户名
            user_level: 0,
            exp: exp.timestamp(),
            iat: now.timestamp(),
            iss: self.config.issuer.clone(),
            aud: self.config.audience.clone(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.secret.as_ref()),
        )?;

        Ok(token)
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        let mut validation = Validation::new(Algorithm::HS256);
        // 关键两行：明确告诉库我们期望的 aud 是什么
        validation.set_audience(&self.config.audience);
        // 如果你同时要校验 iss，可以加
        validation.iss = self.config.issuer.clone();
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.config.secret.as_ref()),
            &validation,
        )?;

        Ok(token_data.claims)
    }

    pub fn extract_user_id_from_token(&self, token: &str) -> Result<u64> {
        let claims = self.verify_token(token)?;
        Ok(claims.sub)
    }

    pub fn is_token_expired(&self, token: &str) -> bool {
        match self.verify_token(token) {
            Ok(claims) => {
                let now = Utc::now().timestamp();
                let is_expired = claims.exp < now;
                tracing::debug!(
                    "Token expiration check: exp={}, now={}, expired={}",
                    claims.exp,
                    now,
                    is_expired
                );
                is_expired
            }
            Err(e) => {
                tracing::debug!("Token verification failed: {}", e);
                true
            }
        }
    }

    pub fn refresh_access_token(
        &self,
        refresh_token: &str,
        username: &str,
        user_level: i32,
    ) -> Result<String> {
        let claims = self.verify_token(refresh_token)?;

        // 验证是否为刷新令牌（用户名和邮箱为空）
        if !claims.username.is_empty() {
            return Err(AppError::Auth("Invalid refresh token".to_string()));
        }

        // 检查令牌是否过期
        let now = Utc::now().timestamp();
        if claims.exp < now {
            return Err(AppError::Auth("Refresh token has expired".to_string()));
        }

        // 生成新的访问令牌
        let user_id = claims.sub;

        self.generate_token(user_id, username, user_level)
    }
}

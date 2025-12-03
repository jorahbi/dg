use axum::{async_trait, extract::FromRequestParts, http::request::Parts};

use crate::{
    error::{AppError, Result},
    utils::jwt::Claims,
};

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: u64,
    pub username: String,
    pub user_level: i32,
    pub lang: String,
}

impl AuthUser {
    pub fn from_claims(claims: Claims) -> Self {
        let user_id = claims.sub;

        Self {
            id: user_id,
            username: claims.username,
            user_level: claims.user_level,
            lang: "en".to_string(), // 默认语言为英文，后续可以从数据库或配置中获取
        }
    }

    pub fn from_claims_with_lang(claims: Claims, lang: String) -> Self {
        let user_id = claims.sub;

        Self {
            id: user_id,
            username: claims.username,
            user_level: claims.user_level,
            lang,
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        // 从Extension中获取已经验证过的AuthUser
        // 这个验证应该在JWT中间件中完成
        match parts.extensions.get::<AuthUser>() {
            Some(user) => Ok(user.clone()),
            None => Err(AppError::Auth("User not authenticated or authentication expired".to_string())),
        }
    }
}

// JWT Claims extractor - 直接从token解析，不验证用户状态
#[derive(Debug, Clone)]
pub struct JwtClaims {
    pub user_id: u64,
    pub username: String,
    pub user_level: i32,
}

impl JwtClaims {
    pub fn from_claims(claims: Claims) -> Self {
        let user_id = claims.sub;

        Self {
            user_id,
            username: claims.username,
            user_level: claims.user_level,
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for JwtClaims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        // 从Extension中获取已经验证过的JWT Claims
        // 这个验证应该在JWT中间件中完成
        match parts.extensions.get::<JwtClaims>() {
            Some(claims) => Ok(claims.clone()),
            None => Err(AppError::Auth("JWT token invalid or not authenticated".to_string())),
        }
    }
}

// 可选的认证用户（可能不存在）
#[derive(Debug, Clone)]
pub struct OptionalAuthUser(pub Option<AuthUser>);

#[async_trait]
impl<S> FromRequestParts<S> for OptionalAuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        // 从Extension中获取已经验证过的AuthUser
        match parts.extensions.get::<AuthUser>() {
            Some(user) => Ok(OptionalAuthUser(Some(user.clone()))),
            None => {
                // 检查是否有Authorization header，如果有但验证失败，返回None而不是错误
                let has_auth = parts.headers.get("authorization").is_some();

                if has_auth {
                    tracing::debug!("发现Authorization header但验证失败，返回None");
                }

                Ok(OptionalAuthUser(None))
            }
        }
    }
}

// 检查用户等级权限的辅助函数
pub fn check_user_level(user_level: i32, required_level: i32) -> Result<()> {
    if user_level < required_level {
        return Err(AppError::Authorization(format!(
            "Required level {}, current level {}",
            required_level, user_level
        )));
    }
    Ok(())
}

// 检查用户是否为管理员
pub fn is_admin_user(user_level: i32) -> bool {
    user_level >= 5 // 假设等级5及以上为管理员
}

// 检查用户是否有特定权限
pub fn has_permission(user_level: i32, permission: &str) -> bool {
    match permission {
        "basic" => user_level >= 0,
        "vip" => user_level >= 3,
        "admin" => user_level >= 5,
        "super_admin" => user_level >= 9,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_user_level() {
        assert!(check_user_level(5, 3).is_ok()); // 等级5大于要求等级3
        assert!(check_user_level(2, 3).is_err()); // 等级2小于要求等级3
    }

    #[test]
    fn test_is_admin_user() {
        assert!(is_admin_user(5));
        assert!(is_admin_user(9));
        assert!(!is_admin_user(3));
        assert!(!is_admin_user(0));
    }

    #[test]
    fn test_has_permission() {
        assert!(has_permission(1, "basic"));
        assert!(has_permission(4, "vip"));
        assert!(has_permission(6, "admin"));
        assert!(has_permission(9, "super_admin"));

        assert!(!has_permission(2, "vip"));
        assert!(!has_permission(3, "admin"));
        assert!(!has_permission(5, "super_admin"));
    }
}

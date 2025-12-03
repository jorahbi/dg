use crate::{
    error::{AppError, Result},
    extract::{AuthUser, JwtClaims},
    state::AppState,
    utils::jwt::JwtService,
};
use axum::{
    extract::{Extension, Request, State},
    http::header,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

/// JWT认证中间件 - 验证JWT token并将用户信息添加到请求中
pub async fn jwt_auth_middleware(
    State(app_state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Result<Response> {
    // 从Authorization header中提取token
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|auth_str| {
            if auth_str.starts_with("Bearer ") {
                Some(&auth_str[7..])
            } else {
                None
            }
        });

    let token = auth_header.ok_or_else(|| {
        AppError::Auth("Missing Authorization header, format should be: Bearer <token>".to_string())
    })?;

    // 验证JWT token
    let jwt_service = JwtService::new(app_state.config.jwt.clone());
    let claims = jwt_service.verify_token(token)?;
    // 创建AuthUser对象
    let auth_user = AuthUser::from_claims(claims.clone());

    // 创建JwtClaims对象
    let jwt_claims = JwtClaims::from_claims(claims);

    // 将用户信息添加到请求扩展中
    request.extensions_mut().insert(auth_user);
    request.extensions_mut().insert(jwt_claims);

    // 继续处理请求
    Ok(next.run(request).await)
}

// 管理员权限中间件
pub async fn admin_middleware(
    auth_user: AuthUser,
    request: Request,
    next: Next,
) -> Result<Response> {
    // 检查用户是否为管理员（等级5及以上）
    if auth_user.user_level < 5 {
        return Err(AppError::Authorization("Administrator permission required".to_string()));
    }

    Ok(next.run(request).await)
}

// VIP权限中间件
pub async fn vip_middleware(auth_user: AuthUser, request: Request, next: Next) -> Result<Response> {
    // 检查用户是否为VIP（等级3及以上）
    if auth_user.user_level < 3 {
        return Err(AppError::Authorization("VIP permission required".to_string()));
    }

    Ok(next.run(request).await)
}

// KYC认证中间件
pub async fn kyc_middleware(
    Extension(_app_state): Extension<Arc<AppState>>,
    _auth_user: AuthUser,
    request: Request,
    next: Next,
) -> Result<Response> {
    // 检查用户是否通过KYC认证 - 暂时简化实现
    // TODO: 实现真正的KYC检查
    let is_kyc_verified = false; // 暂时返回false，避免数据库查询编译错误

    if is_kyc_verified {
        Ok(next.run(request).await)
    } else {
        Err(AppError::Authorization("KYC verification required".to_string()))
    }
}

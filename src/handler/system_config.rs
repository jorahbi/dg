use crate::schema::SystemConfigCreateRequest;
use crate::{
    error::AppError::*,
    error::Result,
    extract::AuthUser,
    schema::common::ApiResponse,
    schema::system_config::{SystemConfigRequest, SystemConfigResponse},
    service::system_config::SystemConfigService,
    state::AppState,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use std::collections::HashMap;

/// 创建系统配置
pub async fn create_config(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(request): Json<SystemConfigCreateRequest>,
) -> Result<impl IntoResponse> {
    // 检查用户权限（需要管理员权限）
    if auth_user.user_level < 2 {
        return Err(Authorization("Administrator permission required".to_string()));
    }

    let config_service = SystemConfigService::new(&state);
    let id = config_service
        .create_config(
            &request.config_key,
            &request.config_value,
            request.description.as_deref(),
        )
        .await?;
    let mut response = HashMap::new();
    response.insert("id", id);
    Ok(Json(ApiResponse::success(response)))
}

/// 根据配置键获取系统配置
pub async fn get_config_by_key(
    State(state): State<AppState>,
    Path(config_key): Path<String>,
) -> Result<impl IntoResponse> {
    let config_service = SystemConfigService::new(&state);

    let config: SystemConfigResponse = config_service.get_config_by_key(&config_key).await?.into();

    Ok(Json(ApiResponse::success(config)))
}

/// 更新系统配置
pub async fn update_config(
    State(state): State<AppState>,
    Path(key): Path<String>,
    Json(request): Json<SystemConfigRequest>,
) -> Result<impl IntoResponse> {
    let config_service = SystemConfigService::new(&state);
    let config = config_service
        .update_config(&key, &request.config_value, request.description.as_deref())
        .await?;

    Ok(Json(ApiResponse::success_with_message(
        config,
        "配置更新成功",
    )))
}

/// 删除系统配置
pub async fn delete_config(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(key): Path<String>,
) -> Result<impl IntoResponse> {
    // 检查用户权限（需要管理员权限）
    if auth_user.user_level < 2 {
        return Err(Authorization("Administrator permission required".to_string()));
    }

    let config_service = SystemConfigService::new(&state);
    let message = config_service.delete_config(&key).await?;

    Ok(Json(ApiResponse::success_with_message(
        message,
        "配置删除成功",
    )))
}

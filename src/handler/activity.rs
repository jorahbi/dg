use crate::model::system_config::SystemConfigType;
use crate::service::activity::{ActivityService, SystemActivityWelcomeBonus};
use crate::service::system_config::SystemConfigService;
use crate::{
    error::Result, extract::AuthUser, schema::common::ApiResponse, state::AppState, AppError,
};
use axum::{
    extract::State,
    response::{IntoResponse, Json},
};

///新用户领取奖励
pub async fn welcome_bonus(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse> {
    let key = SystemConfigType::WelcomeBonus.to_string();
    let scs = SystemConfigService::new(&state);
    let value = scs.get_config_by_key(&key).await;
    let Ok(value) = value else {
        return Err(AppError::NotFound("Welcome bonus not enabled".to_string()));
    };
    let bonus: SystemActivityWelcomeBonus = serde_json::from_str(&value.config_value)?;
    if bonus.amount <= 0 {
        return Err(AppError::NotFound(
            "Welcome bonus amount setting error".to_string(),
        ));
    }
    let activity_service = ActivityService::new(&state);
    let res = activity_service.welcome_bonus(auth_user.id, &bonus).await;

    if let Err(err) = res {
        tracing::error!("welcome bonus error: {}", err);
        return Err(AppError::NotFound(
            "Failed to claim welcome bonus".to_string(),
        ));
    };

    Ok(Json(ApiResponse::empty_object()))
}

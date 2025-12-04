use crate::extract::AuthUser;
use crate::repository::power_repo::PowerRepo;
use crate::repository::UserRepo;
use crate::utils::time_zone::TimeZone;
use crate::{error::Result, schema::common::ApiResponse, state::AppState};
use axum::{
    extract::State,
    response::{IntoResponse, Json},
};
use rust_decimal::Decimal;

// 获取统计数据
pub async fn get_statistics(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse> {
    let mut asset = UserRepo::get_assets(&state.db, auth_user.id).await?;
    asset.daily_balance = Decimal::ZERO;
    let curr = TimeZone::Beijing.get_time();
    asset.daily_balance = PowerRepo::get_daily_power_total(&state.db, &curr, auth_user.id).await?;
    let response = ApiResponse::success(asset);
    Ok(Json(response))
}

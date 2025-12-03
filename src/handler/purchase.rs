use crate::model::power::convert_power_package_to_package_item;
use crate::model::{POWER_PACKAGE_STATUS_NO_UPGRADE, USER_POWER_RECORD_STATUS_ACTIVE};
use crate::repository::power_repo::PowerRepo;
use crate::schema::UpgradeOrderRequest;
use crate::service::system_config::SystemConfigService;
use crate::service::{order::OrderService, power::PowerService, UserService};
use crate::{
    error::AppError::*,
    error::Result,
    extract::AuthUser,
    schema::common::ApiResponse,
    schema::order::{
        CreateOrderRequest, CreateOrderResponse, UpdateOrderStatusRequest,
        UpdateOrderStatusResponse,
    },
    schema::power::PowerPackageItem,
    state::AppState,
};
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Json},
};
use serde_json::json;
use validator::Validate;
use crate::utils::time_zone::TimeZone;

// 获取算力详情
pub async fn get_package_detail(
    State(state): State<AppState>,
    Path(power_id): Path<u64>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse> {
    // 使用 PowerService 获取算力包数据
    let power_service = PowerService::new(&state);
    let power_package = power_service
        .get_power_record_by_id(power_id)
        .await?
        .ok_or_else(|| NotFound("Computing power package not found".to_string()))?;

    // 使用多语言转换函数将数据库模型转换为 API 响应模型
    let package_item: PowerPackageItem =
        convert_power_package_to_package_item(power_package, &auth_user.lang);

    let response = ApiResponse::success(package_item);
    Ok(Json(response))
}

// 提交购买订单
pub async fn create_order(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateOrderRequest>,
) -> Result<impl IntoResponse> {
    let addr = SystemConfigService::new(&state)
        .get_chain_addr(&payload.blockchain_type)
        .await?;

    let package = PowerRepo::get_power_record_by_id(&state.db, payload.power_id).await?;
    let Some(package) = package else {
        return Err(NotFound("Computing power package to purchase not found".to_string()));
    };

    let user_service = UserService::new(&state);
    let userinfo = user_service.get_user_info(auth_user.id).await?;

    // 使用 OrderService 创建订单
    let order_service = OrderService::new(&state);
    let (_order_id, order_number) = order_service
        .create_order(&userinfo, &package, &payload.blockchain_type, &addr)
        .await?;

    let response =
        ApiResponse::success_with_message(CreateOrderResponse { order_number }, "Order created successfully");
    Ok(Json(response))
}

// 获取订单详情
pub async fn get_order_detail(
    State(_state): State<AppState>,
    _auth_user: AuthUser,
    Path(_order_id): Path<String>,
) -> Result<impl IntoResponse> {
    // 使用 OrderService 获取订单详情
    // let order_service = OrderService::new(&state);
    // let mut m_order = order_service
    //     .get_order_detail(auth_user.id, &order_id)
    //     .await?;
    //
    // // // 构建响应数据
    // let order_response = CreateOrderResponse {
    //     order_id: m_order.order_id,
    //     order_number: format!("ORD{}", m_order.id),
    //     power_id: order.power_package_id,
    //     power_name: "AI智能计算".to_string(), // 这里应该从power_package获取
    //     quantity: order.quantity,
    //     amount: Decimal::from_str(&order.amount.to_string())
    //         .unwrap_or_else(|_| Decimal::ZERO)
    //         .to_f64()
    //         .unwrap_or(0.0),
    //     currency: "USDT".to_string(),
    //     blockchain_type: order.blockchain_type,
    //     blockchain_address: order.blockchain_address,
    //     transaction_hash: order.transaction_hash,
    //     is_paid: order.is_paid == 1,
    //     status: order.status,
    // };
    // // 获取算力包信息以获取多语言标题
    // let power_service = PowerService::new(&state);
    // if let Some(power_package) = power_service
    //     .get_power_record_by_id(m_order.power_package_id)
    //     .await?
    // {
    //     let package_item = convert_power_package_to_package_item(power_package, &auth_user.lang);
    //     order_response.power_name = package_item.title;
    // }
    //
    // let response = ApiResponse::success(order_response);
    // Ok(Json(response))
    Ok("")
}

// 取消订单
pub async fn cancel_order(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(order_id): Path<String>,
) -> Result<impl IntoResponse> {
    // 使用 OrderService 取消订单
    let order_service = OrderService::new(&state);
    order_service.cancel_order(auth_user.id, &order_id).await?;

    // 构建响应数据
    let cancel_data = json!({
        "orderId": order_id,
        "cancelTime": chrono::Utc::now().to_rfc3339(),
    });

    let response = ApiResponse::success_with_message(cancel_data, "Order cancelled");
    Ok(Json(response))
}

pub async fn paid_order(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(order_id): Path<String>,
) -> Result<impl IntoResponse> {
    // 使用 OrderService 取消订单
    let order_service = OrderService::new(&state);
    order_service.paid_order(auth_user.id, &order_id).await?;

    Ok(Json(ApiResponse::empty_object()))
}

/// 升级订单
pub async fn upgrade_order(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<UpgradeOrderRequest>,
) -> Result<impl IntoResponse> {
    payload.validate().map_err(|e| Validation(e.to_string()))?;
    let old_package = PowerRepo::get_power_record_by_id_and_user(
        &state.db,
        payload.old_user_power_id,
        auth_user.id,
    )
    .await?;

    if old_package.status != USER_POWER_RECORD_STATUS_ACTIVE {
        return Err(OrderNotPaid("Computing power package not paid".to_string()));
    }
    let addr = SystemConfigService::new(&state)
        .get_chain_addr(&payload.blockchain_type)
        .await?;
    let package = PowerRepo::get_power_record_by_id(&state.db, payload.power_id).await?;
    let Some(package) = package else {
        return Err(NotFound("Upgrade computing power package not found".to_string()));
    };
    if old_package.lv as u16 >= package.lv {
        return Err(Validation(
            "Upgrade computing power package has the same level as current package, cannot upgrade".to_string(),
        ));
    }

    if package.is_upgrade == POWER_PACKAGE_STATUS_NO_UPGRADE {
        return Err(NotFound("Computing power package upgrade channel not enabled".to_string()));
    }
    let user_service = UserService::new(&state);
    let userinfo = user_service.get_user_info(auth_user.id).await?;
    let total_assets = userinfo.total_assets + old_package.amount;
    let order_service = OrderService::new(&state);
    let (_order_id, num) = order_service
        .upgrade_order(
            &userinfo,
            &package,
            total_assets,
            &payload.blockchain_type,
            &addr,
            old_package.id,
        )
        .await?;
    // 构建响应数据
    let response = json!({
        "orderId": num,
        "cancelTime": TimeZone::Beijing.get_time(),
    });
    Ok(Json(response))
}


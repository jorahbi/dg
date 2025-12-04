use axum::{
    http::{HeaderName, HeaderValue, Method},
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::handler::activity::welcome_bonus;
use crate::handler::{paid_order, upgrade_order};
use crate::{
    handler::{
        about_us::get_about_us,
        airdrop::{
            check_daily_airdrop_status, claim_airdrop, get_airdrop_history, get_airdrop_stats,
            get_airdrops, get_popular_airdrops, get_user_airdrop_eligibility,
        },
        asset::{
            exchange_currency, get_asset_earnings, get_conversion_records, get_recharge_records,
            get_supported_blockchains, withdraw_asset,
        },
        auth::{
            change_password, check_security_questions, forgot_password_questions,
            forgot_password_verify, get_security_questions, login, logout, register,
            reset_password, save_security_questions,
        },
        chart::{get_asset_chart_data, get_leaderboard, get_power_chart_data, get_realtime_data},
        chat::{get_chat_messages, mark_chat_message_read, send_message, ws_chat_handler},
        cron::{get_cron_status, start_cron_scheduler, stop_cron_scheduler},
        earnings::get_earnings,
        home::get_statistics,
        invite::{get_invite_code, get_invite_records, get_invite_rewards},
        kyc::{get_kyc_status, submit_kyc, upload_id_card},
        message::{
            delete_message, get_messages, get_unread_count, mark_all_messages_read,
            mark_message_read,
        },
        power::{
            cancel_withdrawal, get_all_power_packages, get_power_records, get_power_stats,
            get_withdrawal, start_power, upgrade_level, withdraw_power,
        },
        promotion::{
            claim_reward, get_my_promotions, get_promotion_detail, get_promotion_packages,
            get_promotions, join_promotion,
        },
        purchase::{cancel_order, create_order, get_order_detail, get_package_detail},
        system_config::{create_config, delete_config, get_config_by_key, update_config},
        user::get_user_info,
        user_benefit::{claim_benefit, get_benefit_center, get_new_user_benefit},
    },
    middleware::{jwt_auth_middleware, logging_middleware},
    state::AppState,
};

pub async fn create_app(app_state: Arc<AppState>) -> Result<Router, crate::error::AppError> {
    // Public routes (no JWT verification required)
    let public_routes = Router::new().route("/about-us", get(get_about_us));

    // Protected routes requiring JWT verification
    let protected_routes = Router::new()
        // Authentication management module (requires login)
        .route(
            "/auth/security-questions/check",
            get(check_security_questions),
        )
        .route("/user/profile/password", post(change_password))
        // User management module
        // Purchase management module
        // Activities
        // Earnings management module
        .route("/earnings", get(get_earnings))
        // Airdrop activity module
        // Task management module
        // Invitation reward module
        .route("/invite/rewards", get(get_invite_rewards))
        .route("/invite/code", get(get_invite_code))
        .route("/invite/records", get(get_invite_records))
        // Asset center module
        .route("/asset/recharge-records", get(get_recharge_records))
        .route("/asset/conversion-records", get(get_conversion_records))
        .route("/asset/exchange", post(exchange_currency))
        .route(
            "/asset/supported-blockchains",
            get(get_supported_blockchains),
        )
        .route("/asset/withdraw", post(withdraw_asset))
        .route("/asset/earnings", get(get_asset_earnings))
        .route("/asset/chart/:symbol", get(get_asset_chart_data))
        // Message center module
        .route("/inbox/messages", get(get_messages))
        .route("/inbox/messages/:messageId/read", put(mark_message_read))
        .route("/inbox/messages/read-all", put(mark_all_messages_read))
        .route("/inbox/messages/:messageId", delete(delete_message))
        .route("/inbox/messages/unread-count", get(get_unread_count))
        // Chart data module (user access)
        .route("/chart/power", get(get_power_chart_data))
        .route("/chart/asset", get(get_asset_chart_data))
        .route("/chart/realtime", get(get_realtime_data))
        .route("/chart/leaderboard", get(get_leaderboard))
        // Promotion activity module
        .route("/promotions", get(get_promotions))
        .route("/promotions/:promotionId", get(get_promotion_detail))
        .route("/promotions/:promotionId/join", post(join_promotion))
        .route("/promotions/:promotionId/claim", post(claim_reward))
        .route("/promotions/my", get(get_my_promotions))
        .route("/promotions/packages", get(get_promotion_packages))
        // Chat module
        .route("/chat/messages", get(get_chat_messages))
        .route("/chat/messages", post(send_message))
        .route(
            "/chat/messages/:messageId/read",
            put(mark_chat_message_read),
        )
        // System configuration
        .route("/system/config", post(create_config))
        .route("/system/config", delete(delete_config))
        .route("/system/config", put(update_config))
        .route("/system/config/:key", get(get_config_by_key))
        // Scheduled task management
        .route("/cron/status", get(get_cron_status))
        .route("/cron/start", post(start_cron_scheduler))
        .route("/cron/stop", post(stop_cron_scheduler))
        // Apply JWT verification middleware to all protected routes
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            jwt_auth_middleware,
        ));

    let protected_routes = protected_routes
        .merge(user())
        .merge(power())
        .merge(order())
        .merge(activity())
        .merge(auth())
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            jwt_auth_middleware,
        ));

    // 创建WebSocket路由（需要JWT验证）
    let ws_routes = Router::new()
        .route("/ws/chat", get(ws_chat_handler))
        .route("/ws", get(ws_chat_handler))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            jwt_auth_middleware,
        ))
        .with_state(app_state.clone());

    // 创建健康检查路由
    let health_routes = Router::new()
        .route("/health", get(health_check))
        .with_state(app_state.clone());

    // 组合所有路由
    let app = Router::new()
        .merge(health_routes)
        .merge(ws_routes)
        .nest("/api", public_routes.merge(protected_routes))
        .layer(
            CorsLayer::new()
                .allow_origin([
                    "http://localhost:3000".parse::<HeaderValue>().unwrap(),
                    "http://localhost:8080".parse::<HeaderValue>().unwrap(),
                    "http://127.0.0.1:3000".parse::<HeaderValue>().unwrap(),
                    "http://127.0.0.1:8080".parse::<HeaderValue>().unwrap(),
                    "https://astrai.com".parse::<HeaderValue>().unwrap(),
                    "https://www.astrai.com".parse::<HeaderValue>().unwrap(),
                    "https://app.astrai.com".parse::<HeaderValue>().unwrap(),
                ])
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::DELETE,
                    Method::OPTIONS,
                ])
                .allow_headers([
                    axum::http::header::AUTHORIZATION,
                    axum::http::header::ACCEPT,
                    axum::http::header::CONTENT_TYPE,
                    axum::http::header::ORIGIN,
                    axum::http::header::USER_AGENT,
                    HeaderName::from_static("x-requested-with"),
                ])
                .allow_credentials(true),
        )
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(middleware::from_fn(logging_middleware)),
        )
        .with_state((*app_state).clone());

    Ok(app)
}

fn user() -> Router<AppState> {
    let route = Router::new()
        .route("/user/info", get(get_user_info)) //✅
        .route("/user/statistics", get(get_statistics)) //✅
        .route("/auth/security-questions", post(save_security_questions)) //✅
        .route("/auth/logout", post(logout))
        .route("/user/invite/code", get(get_invite_code))
        .route("/user/new-user-benefit", get(get_new_user_benefit))
        .route("/user/benefits", get(get_benefit_center))
        .route("/user/benefits/claim/:benefitId", post(claim_benefit))
        .route("/user/benefits/new-user", get(get_new_user_benefit))
        .route("/user/kyc/status", get(get_kyc_status))
        .route("/user/kyc/submit", post(submit_kyc))
        .route("/user/kyc/upload", post(upload_id_card));

    route
}

fn power() -> Router<AppState> {
    let route = Router::new()
        .route("/power/:powerId", get(get_package_detail)) //✅
        // Computing power management module
        .route("/power/:upp/start/", put(start_power)) //✅
        .route("/power/records", get(get_power_records)) //✅
        .route("/power/packages", get(get_all_power_packages)) //✅
        .route("/power/stats", post(get_power_stats)) //✅
        .route("/power/upgrade/:levelId", post(upgrade_level))
        .route("/power/withdraw", post(withdraw_power))
        .route("/power/withdrawal", get(get_withdrawal))
        .route(
            "/power/withdrawal/:withdrawalId/cancel",
            post(cancel_withdrawal),
        );

    route
}

fn order() -> Router<AppState> {
    let route = Router::new()
        .route("/purchase/order", post(create_order)) //✅
        .route("/purchase/orders/:orderId", get(get_order_detail))
        .route("/purchase/orders/:orderId/cancel", post(cancel_order)) //✅
        .route("/purchase/orders/:orderId/paid", post(paid_order)) //✅
        .route("/purchase/orders/upgrade", post(upgrade_order)); //✅

    return route;
}

fn activity() -> Router<AppState> {
    let route = Router::new()
        .route("/activity/welcome", get(welcome_bonus)) //✅
        .route("/airdrops", get(get_airdrops))
        .route("/airdrops/claim", post(claim_airdrop))
        .route("/airdrops/history", get(get_airdrop_history))
        .route("/airdrops/stats", get(get_airdrop_stats))
        .route("/airdrops/popular", get(get_popular_airdrops))
        .route("/airdrops/daily-status", get(check_daily_airdrop_status))
        .route("/airdrops/eligibility", get(get_user_airdrop_eligibility));

    return route;
}

fn auth() -> Router<AppState> {
    let routes = Router::new()
        // Authentication management module
        .route("/auth/register", post(register)) //✅
        .route("/auth/login", post(login)) //✅
        .route(
            "/auth/forgot-password/questions",
            post(forgot_password_questions), //✅
        )
        .route("/auth/forgot-password/verify", post(forgot_password_verify)) //✅
        .route("/auth/forgot-password/reset", post(reset_password)) //✅
        .route("/auth/security-questions", get(get_security_questions)); //✅
    return routes;
}

// Health check handler
async fn health_check() -> &'static str {
    "OK"
}

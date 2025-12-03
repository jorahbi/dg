mod app;
mod config;
mod cron;
mod error;
mod extract;
mod handler;
mod middleware;
mod model;
mod repository;
mod schema;
mod service;
mod state;
mod utils;
mod websocket;

use app::create_app;
use config::Config;
use error::{AppError, Result};
use state::AppState;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter("info,astra_ai_api=debug")
        // .with_env_filter("sqlx::query=trace")
        .with_target(false)
        .init();

    tracing::info!("Starting Astra AI API server...");

    // 加载配置
    let config = Config::from_env()?;
    tracing::info!("Configuration loaded successfully: {}", config.app.name);

    // 创建应用状态
    let app_state = AppState::new(config.clone()).await?;
    tracing::info!("Application state initialized successfully");

    // 运行数据库迁移
    // sqlx::migrate!("./migrations")
    //     .run(app_state.db.as_ref())
    //     .await
    //     .map_err(|e| {
    //         tracing::error!("数据库迁移失败: {:?}", e);
    //         e
    //     })?;
    // tracing::info!("数据库迁移完成");

    // 创建应用状态
    let app_state_arc = Arc::new(app_state);

    // 启动聊天记录批量保存后台任务
    crate::websocket::handler::start_chat_message_save_task(app_state_arc.clone()).await;
    tracing::info!("Chat message batch save background task started");

    // 启动定时任务调度器
    app_state_arc
        .cron_scheduler
        .start(app_state_arc.clone())
        .await
        .map_err(|e| {
            tracing::error!("Failed to start scheduled task manager: {}", e);
            AppError::Internal(format!("Failed to start scheduled task manager: {}", e))
        })?;
    tracing::info!("Scheduled task manager started");

    // 创建应用
    let app_state_for_server = app_state_arc.clone();
    let app = create_app(app_state_for_server).await?;

    // 构建服务器地址
    let addr = format!("{}:{}", config.server.host, config.server.port);
    tracing::info!("Server listening on address: {}", addr);

    // 启动HTTP服务器
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("HTTP server started successfully");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(app_state_arc.clone()))
        .await
        .map_err(|e| {
            tracing::error!("Server runtime error: {}", e);
            AppError::Internal(e.to_string())
        })?;

    // 强制保存所有缓冲的聊天记录
    let buffered_messages = {
        let ws_hub = app_state_arc.ws_hub.read().await;
        ws_hub.force_save_messages().await
    };
    if !buffered_messages.is_empty() {
        tracing::info!("Saved {} buffered chat messages before shutdown", buffered_messages.len());
    }

    tracing::info!("Server shutdown normally");
    Ok(())
}

/// 优雅关闭信号处理
async fn shutdown_signal(app_state: Arc<AppState>) {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.expect("Failed to listen for Ctrl+C signal");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to listen for terminate signal")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Received Ctrl+C signal, starting graceful shutdown...");
            perform_graceful_shutdown(app_state).await;
        },
        _ = terminate => {
            tracing::info!("Received terminate signal, starting graceful shutdown...");
            perform_graceful_shutdown(app_state).await;
        },
    }
}

/// 执行优雅关闭操作
async fn perform_graceful_shutdown(app_state: Arc<AppState>) {
    tracing::info!("Starting graceful shutdown process...");

    // 1. 停止定时任务调度器
    tracing::info!("Stopping scheduled task manager...");
    if let Err(e) = app_state.cron_scheduler.stop().await {
        tracing::error!("Failed to stop scheduled task manager: {}", e);
    } else {
        tracing::info!("Scheduled task manager stopped");
    }

    // 2. 等待一段时间让正在运行的任务完成
    tracing::info!("Waiting for running tasks to complete...");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // 3. 保存所有缓冲的聊天记录
    tracing::info!("Saving buffered chat messages...");
    let buffered_messages = {
        let ws_hub = app_state.ws_hub.read().await;
        ws_hub.force_save_messages().await
    };
    if !buffered_messages.is_empty() {
        tracing::info!(
            "Saved {} buffered chat messages during graceful shutdown",
            buffered_messages.len()
        );
    }

    tracing::info!("Graceful shutdown process completed");
}

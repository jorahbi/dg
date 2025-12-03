use crate::{
    error::{AppError, Result},
    state::AppState,
    websocket::hub::{ChatMessage, WsHub},
};
use axum::{
    extract::{ws::WebSocketUpgrade, State},
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde_json::json;
use std::sync::Arc;
use tokio::time::{interval, Duration};

/// 保存聊天记录到数据库
async fn save_chat_messages_to_db(
    _state: &Arc<AppState>,
    messages: Vec<ChatMessage>,
) -> Result<usize> {
    if messages.is_empty() {
        return Ok(0);
    }

    let mut saved_count = 0;

    // 这里暂时简化实现，实际应该批量插入到数据库
    // TODO: 实现真正的数据库批量插入
    for message in &messages {
        // 模拟保存到数据库
        saved_count += 1;
        tracing::debug!("保存聊天记录: {} -> {}", message.sender_id, message.content);
    }

    tracing::info!("批量保存了 {} 条聊天记录到数据库", saved_count);
    Ok(saved_count)
}

/// 启动聊天记录批量保存后台任务
pub async fn start_chat_message_save_task(state: Arc<AppState>) {
    let state_clone = state.clone();

    tokio::spawn(async move {
        let mut save_interval = interval(Duration::from_secs(30)); // 每30秒检查一次

        loop {
            save_interval.tick().await;

            // 检查是否需要保存消息
            let ws_hub = state_clone.ws_hub.read().await;
            if ws_hub.should_save_messages().await {
                let messages = ws_hub.flush_message_buffer().await;

                if !messages.is_empty() {
                    match save_chat_messages_to_db(&state_clone, messages).await {
                        Ok(saved_count) => {
                            tracing::info!("后台任务成功保存 {} 条聊天记录", saved_count);
                        }
                        Err(e) => {
                            tracing::error!("保存聊天记录失败: {}", e);
                        }
                    }
                }
            }

            // 清理断开连接的用户
            {
                let ws_hub = state_clone.ws_hub.read().await;
                ws_hub.cleanup_disconnected_users().await;
            }
        }
    });
}

// 通用WebSocket处理器
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    auth_user: crate::extract::AuthUser,
) -> Response {
    ws.on_upgrade(|socket| handle_websocket(socket, State(state), auth_user))
}

// 处理通用WebSocket连接
async fn handle_websocket(
    socket: axum::extract::ws::WebSocket,
    State(state): State<Arc<AppState>>,
    auth_user: crate::extract::AuthUser,
) {
    tracing::info!(
        "新的WebSocket连接 - 用户ID: {}, 用户名: {}",
        auth_user.id,
        auth_user.username
    );

    // 使用认证用户的ID和用户名
    let user_id = auth_user.id;
    let username = auth_user.username.clone();

    // 将用户添加到WebSocket Hub
    let user_sender = {
        let ws_hub = state.ws_hub.write().await;
        ws_hub.add_user(user_id, username.clone()).await
    };

    // 发送欢迎消息
    let welcome_msg = json!({
        "type": "system",
        "data": {
            "message": "连接成功",
            "userId": user_id,
            "username": username,
            "timestamp": chrono::Utc::now().to_rfc3339()
        }
    });

    let (mut sender, mut receiver) = socket.split();

    if let Err(_) = sender
        .send(axum::extract::ws::Message::Text(welcome_msg.to_string()))
        .await
    {
        tracing::warn!("发送欢迎消息失败");
        return;
    }

    // 监听用户发送的消息
    let ws_hub = state.ws_hub.clone();

    let receive_task = tokio::spawn({
        let ws_hub = ws_hub.clone();
        let username_for_task = username.clone();
        async move {
            while let Some(msg) = receiver.next().await {
                match msg {
                    Ok(axum::extract::ws::Message::Text(text)) => {
                        // 获取ws_hub的可变引用
                        if let Ok(mut hub) = ws_hub.try_write() {
                            if let Err(e) = handle_websocket_message(
                                &text,
                                &mut hub,
                                user_id,
                                &username_for_task,
                            )
                            .await
                            {
                                tracing::error!("处理WebSocket消息失败: {}", e);
                            }
                        } else {
                            tracing::error!("无法获取WebSocket Hub写锁");
                        }
                    }
                    Ok(axum::extract::ws::Message::Close(_)) => {
                        tracing::info!("用户 {} 断开WebSocket连接", username_for_task);
                        break;
                    }
                    Err(e) => {
                        tracing::error!("WebSocket错误: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
        }
    });

    // 向用户发送消息的任务
    let mut user_receiver = user_sender.subscribe();
    let send_task = tokio::spawn({
        let username_for_send = username.clone();
        async move {
            while let Ok(msg) = user_receiver.recv().await {
                if let Err(e) = sender.send(axum::extract::ws::Message::Text(msg)).await {
                    tracing::warn!("发送消息到用户 {} 失败: {}", username_for_send, e);
                    break;
                }
            }
        }
    });

    // 等待任一任务完成
    tokio::select! {
        _ = receive_task => {},
        _ = send_task => {},
    }

    // 从WebSocket Hub中移除用户
    {
        let ws_hub = state.ws_hub.write().await;
        ws_hub.remove_user(user_id).await;
    }

    tracing::info!("用户 {} WebSocket连接已关闭", username);
}

// 处理通用WebSocket消息
async fn handle_websocket_message(
    message_text: &str,
    ws_hub: &mut WsHub,
    user_id: u64,
    username: &str,
) -> Result<()> {
    let message: serde_json::Value = serde_json::from_str(message_text)
        .map_err(|e| AppError::Validation(format!("Message format error: {}", e)))?;

    match message.get("type").and_then(|v| v.as_str()) {
        Some("ping") => {
            // 处理心跳消息
            let pong_msg = json!({
                "type": "pong",
                "data": {
                    "timestamp": chrono::Utc::now().to_rfc3339()
                }
            });

            ws_hub
                .send_message_to_user(user_id, &pong_msg.to_string())
                .await?;
        }
        Some("join_room") => {
            // 加入房间
            if let Some(room_id) = message
                .get("data")
                .and_then(|d| d.get("roomId"))
                .and_then(|id| id.as_str())
            {
                ws_hub.add_user_to_room(room_id, user_id).await;

                let join_msg = json!({
                    "type": "room_joined",
                    "data": {
                        "roomId": room_id,
                        "userId": user_id,
                        "username": username,
                        "timestamp": chrono::Utc::now().to_rfc3339()
                    }
                });

                ws_hub
                    .send_message_to_room(room_id, &join_msg.to_string())
                    .await?;
            }
        }
        Some("leave_room") => {
            // 离开房间
            if let Some(room_id) = message
                .get("data")
                .and_then(|d| d.get("roomId"))
                .and_then(|id| id.as_str())
            {
                ws_hub.remove_user_from_room(room_id, user_id).await;

                let leave_msg = json!({
                    "type": "room_left",
                    "data": {
                        "roomId": room_id,
                        "userId": user_id,
                        "username": username,
                        "timestamp": chrono::Utc::now().to_rfc3339()
                    }
                });

                ws_hub
                    .send_message_to_room(room_id, &leave_msg.to_string())
                    .await?;
            }
        }
        Some("room_message") => {
            // 发送房间消息
            if let (Some(_room_id), Some(content)) = (
                message
                    .get("data")
                    .and_then(|d| d.get("roomId"))
                    .and_then(|id| id.as_str()),
                message
                    .get("data")
                    .and_then(|d| d.get("content"))
                    .and_then(|c| c.as_str()),
            ) {
                // 使用批量保存功能发送消息
                ws_hub
                    .send_message_with_buffer(user_id, None, content, 1)
                    .await?;
            }
        }
        Some("private_message") => {
            // 发送私聊消息
            if let (Some(receiver_id), Some(content)) = (
                message
                    .get("data")
                    .and_then(|d| d.get("receiverId"))
                    .and_then(|id| id.as_u64()),
                message
                    .get("data")
                    .and_then(|d| d.get("content"))
                    .and_then(|c| c.as_str()),
            ) {
                // 使用批量保存功能发送私聊消息
                ws_hub
                    .send_message_with_buffer(user_id, Some(receiver_id), content, 1)
                    .await?;

                // 向接收者发送通知
                let notification = json!({
                    "type": "new_private_message",
                    "data": {
                        "senderId": user_id,
                        "senderUsername": username,
                        "content": content,
                        "timestamp": chrono::Utc::now().to_rfc3339()
                    }
                });

                ws_hub
                    .send_message_to_user(receiver_id, &notification.to_string())
                    .await?;
            }
        }
        Some("get_stats") => {
            // 获取统计信息
            let stats = json!({
                "type": "stats",
                "data": {
                    "userCount": ws_hub.get_user_count().await,
                    "roomCount": ws_hub.get_room_count().await,
                    "timestamp": chrono::Utc::now().to_rfc3339()
                }
            });

            ws_hub
                .send_message_to_user(user_id, &stats.to_string())
                .await?;
        }
        Some("get_room_users") => {
            // 获取房间用户列表
            if let Some(room_id) = message
                .get("data")
                .and_then(|d| d.get("roomId"))
                .and_then(|id| id.as_str())
            {
                let users = ws_hub.get_room_users(room_id).await;
                let users_json: Vec<serde_json::Value> = users
                    .into_iter()
                    .map(|user| {
                        json!({
                            "id": user.id,
                            "username": user.username
                        })
                    })
                    .collect();

                let users_msg = json!({
                    "type": "room_users",
                    "data": {
                        "roomId": room_id,
                        "users": users_json,
                        "userCount": users_json.len(),
                        "timestamp": chrono::Utc::now().to_rfc3339()
                    }
                });

                ws_hub
                    .send_message_to_user(user_id, &users_msg.to_string())
                    .await?;
            }
        }
        _ => {
            tracing::warn!("未知WebSocket消息类型: {:?}", message.get("type"));
        }
    }

    Ok(())
}

// 广播消息到所有连接的用户
pub async fn broadcast_to_all(
    state: &Arc<AppState>,
    message_type: &str,
    data: serde_json::Value,
) -> Result<()> {
    let broadcast_msg = json!({
        "type": message_type,
        "data": {
            "content": data,
            "timestamp": chrono::Utc::now().to_rfc3339()
        }
    });

    let sent_count = state
        .ws_hub
        .read()
        .await
        .broadcast_message(&broadcast_msg.to_string())
        .await?;

    tracing::info!("广播消息发送给 {} 个用户", sent_count);
    Ok(())
}

// 发送系统通知
pub async fn send_system_notification(
    state: &Arc<AppState>,
    title: &str,
    content: &str,
    level: Option<&str>, // info, warning, error
) -> Result<()> {
    let notification = json!({
        "type": "system_notification",
        "data": {
            "title": title,
            "content": content,
            "level": level.unwrap_or("info"),
            "timestamp": chrono::Utc::now().to_rfc3339()
        }
    });

    let sent_count = state
        .ws_hub
        .read()
        .await
        .broadcast_message(&notification.to_string())
        .await?;

    tracing::info!("系统通知发送给 {} 个用户", sent_count);
    Ok(())
}

// 定期清理断开连接的用户
pub async fn cleanup_disconnected_users(state: &Arc<AppState>) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(60)); // 每分钟清理一次

    loop {
        interval.tick().await;

        let user_count_before = state.ws_hub.read().await.get_user_count().await;
        state
            .ws_hub
            .write()
            .await
            .cleanup_disconnected_users()
            .await;
        let user_count_after = state.ws_hub.read().await.get_user_count().await;

        if user_count_before != user_count_after {
            tracing::info!(
                "清理断开连接的用户: {} -> {}",
                user_count_before,
                user_count_after
            );
        }
    }
}

use crate::{
    error::{AppError, Result},
    extract::AuthUser,
    model::{MessageVO, SendMessageReq},
    schema::common::ApiResponse,
    state::AppState,
    websocket::hub::WsHub,
};
use axum::{
    extract::{ws::WebSocketUpgrade, Path, Query, State},
    response::{IntoResponse, Json},
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;

// WebSocket聊天处理器
pub async fn ws_chat_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_chat_socket(socket, state, auth_user))
}

// 处理WebSocket连接
async fn handle_chat_socket(
    socket: axum::extract::ws::WebSocket,
    state: Arc<AppState>,
    auth_user: AuthUser,
) {
    tracing::info!("User {} established WebSocket chat connection", auth_user.username);

    // 创建用户消息通道
    let (mut sender, mut receiver) = socket.split();

    // 将用户添加到WebSocket Hub
    let user_sender = {
        let ws_hub = state.ws_hub.write().await;
        ws_hub
            .add_user(auth_user.id, auth_user.username.clone())
            .await
    };

    // 发送连接成功消息
    let welcome_msg = json!({
        "type": "system",
        "data": {
            "message": "Connection successful",
            "userId": auth_user.id,
            "username": auth_user.username,
            "timestamp": chrono::Utc::now().to_rfc3339()
        }
    });

    if let Err(_) = sender
        .send(axum::extract::ws::Message::Text(welcome_msg.to_string()))
        .await
    {
        tracing::warn!("Failed to send welcome message to user: {}", auth_user.username);
        return;
    }

    // 监听用户发送的消息
    let ws_hub = state.ws_hub.clone();
    let user_id = auth_user.id;
    let username = auth_user.username.clone();

    // 消息处理任务
    let receive_task = tokio::spawn({
        let ws_hub = ws_hub.clone();
        let username_for_task = username.clone();
        async move {
            while let Some(msg) = receiver.next().await {
                match msg {
                    Ok(axum::extract::ws::Message::Text(text)) => {
                        // 获取ws_hub的可变引用
                        if let Ok(mut hub) = ws_hub.try_write() {
                            if let Err(e) =
                                handle_chat_message(&text, &mut hub, user_id, &username_for_task)
                                    .await
                            {
                                tracing::error!("Failed to handle chat message: {}", e);
                            }
                        } else {
                            tracing::error!("Failed to acquire WebSocket Hub write lock");
                        }
                    }
                    Ok(axum::extract::ws::Message::Close(_)) => {
                        tracing::info!("User {} disconnected from WebSocket", username_for_task);
                        break;
                    }
                    Err(e) => {
                        tracing::error!("WebSocket error: {}", e);
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
                    tracing::warn!("Failed to send message to user {}: {}", username_for_send, e);
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

    tracing::info!("User {} WebSocket connection closed", username);
}

// 处理聊天消息
async fn handle_chat_message(
    message_text: &str,
    ws_hub: &mut WsHub,
    user_id: u64,
    username: &str,
) -> Result<()> {
    let message: Value = serde_json::from_str(message_text)
        .map_err(|e| AppError::Validation(format!("Message format error: {}", e)))?;

    match message.get("type").and_then(|v| v.as_str()) {
        Some("message") => {
            handle_text_message(&message, ws_hub, user_id, username).await?;
        }
        Some("typing") => {
            handle_typing_message(&message, ws_hub, user_id, username).await?;
        }
        Some("read") => {
            handle_read_message(&message, ws_hub, user_id, username).await?;
        }
        Some("heartbeat") => {
            // 心跳消息，不做处理
        }
        _ => {
            tracing::warn!("Unknown message type: {:?}", message.get("type"));
        }
    }

    Ok(())
}

// 处理文本消息
async fn handle_text_message(
    message: &Value,
    ws_hub: &mut WsHub,
    user_id: u64,
    username: &str,
) -> Result<()> {
    let data = message
        .get("data")
        .ok_or_else(|| AppError::Validation("Missing message data".to_string()))?;

    let content = data
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::Validation("Missing message content".to_string()))?;

    let receiver_id = data
        .get("receiver_id")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| AppError::Validation("Missing receiver ID".to_string()))?;

    // 检查消息长度
    if content.len() > 1000 {
        return Err(AppError::Validation("Message content too long".to_string()));
    }

    // 创建消息记录（这里简化处理，实际应该保存到数据库）
    let message_id = Uuid::new_v4().to_string();
    let timestamp = chrono::Utc::now();

    // 构造消息VO
    let _message_vo = MessageVO {
        id: 0, // Should be obtained from database
        sender_id: user_id,
        content: content.to_string(),
        msg_type: 1, // Text message
        created_at: timestamp.timestamp_millis(),
    };

    // 发送消息给接收者
    let chat_message = json!({
        "type": "message",
        "data": {
            "messageId": message_id,
            "senderId": user_id,
            "senderUsername": username,
            "receiverId": receiver_id,
            "content": content,
            "msgType": 1,
            "timestamp": timestamp.to_rfc3339(),
            "status": "sent"
        }
    });

    // 发送给接收者
    if let Err(_) = ws_hub
        .send_message_to_user(receiver_id, &chat_message.to_string())
        .await
    {
        tracing::warn!("Failed to send message to user {}", receiver_id);
    }

    // 发送确认给发送者
    let confirmation = json!({
        "type": "message_sent",
        "data": {
            "messageId": message_id,
            "timestamp": timestamp.to_rfc3339(),
            "status": "sent"
        }
    });

    ws_hub
        .send_message_to_user(user_id, &confirmation.to_string())
        .await?;

    Ok(())
}

// 处理正在输入消息
async fn handle_typing_message(
    message: &Value,
    ws_hub: &mut WsHub,
    user_id: u64,
    username: &str,
) -> Result<()> {
    let data = message
        .get("data")
        .ok_or_else(|| AppError::Validation("Missing message data".to_string()))?;

    let receiver_id = data
        .get("receiver_id")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| AppError::Validation("Missing receiver ID".to_string()))?;

    let typing_msg = json!({
        "type": "typing",
        "data": {
            "userId": user_id,
            "username": username,
            "receiverId": receiver_id,
            "isTyping": true,
            "timestamp": chrono::Utc::now().to_rfc3339()
        }
    });

    // 只发送给接收者
    ws_hub
        .send_message_to_user(receiver_id, &typing_msg.to_string())
        .await?;

    Ok(())
}

// 处理已读消息
async fn handle_read_message(
    message: &Value,
    ws_hub: &mut WsHub,
    user_id: u64,
    _username: &str,
) -> Result<()> {
    let data = message
        .get("data")
        .ok_or_else(|| AppError::Validation("Missing message data".to_string()))?;

    let message_id = data
        .get("messageId")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| AppError::Validation("Missing message ID".to_string()))?;

    // 这里应该更新数据库中消息状态为已读
    // 简化处理，直接发送确认

    let read_confirmation = json!({
        "type": "read",
        "data": {
            "messageId": message_id,
            "readerId": user_id,
            "timestamp": chrono::Utc::now().to_rfc3339()
        }
    });

    // 广播已读状态
    ws_hub
        .broadcast_message(&read_confirmation.to_string())
        .await?;

    Ok(())
}

// 获取聊天消息记录
pub async fn get_chat_messages(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(params): Query<serde_json::Value>,
) -> Result<impl IntoResponse> {
    let other_user_id = params
        .get("otherUserId")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| AppError::Validation("Missing otherUserId parameter".to_string()))?;

    let page = params.get("page").and_then(|v| v.as_u64()).unwrap_or(1) as u32;

    let limit = params.get("limit").and_then(|v| v.as_u64()).unwrap_or(20) as u32;

    let (_messages, total) = crate::repository::chat_repo::ChatRepo::find_user_messages(
        &state,
        auth_user.id,
        other_user_id,
        page,
    )
    .await?;

    // Temporarily simplified implementation, avoiding complex field access
    // TODO: Implement real message VO conversion
    let messages_vo: Vec<MessageVO> = vec![];

    let pagination_data =
        crate::schema::common::PaginationData::new(page, limit, total, messages_vo);
    let response = ApiResponse::success(pagination_data);

    Ok(Json(response))
}

// 发送消息（HTTP接口）
pub async fn send_message(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<SendMessageReq>,
) -> Result<impl IntoResponse> {
    // Validate input
    if payload.content.is_empty() {
        return Err(AppError::Validation("Message content cannot be empty".to_string()));
    }

    if payload.content.len() > 1000 {
        return Err(AppError::Validation("Message content too long".to_string()));
    }

    // Create message record
    let _new_message = crate::model::chat::NewChatMessage {
        sender_id: auth_user.id,
        receiver_id: payload.receiver_id,
        content: payload.content.clone(),
        msg_type: payload.msg_type as i8,
    };

    let message_id = crate::repository::chat_repo::ChatRepo::create_message(
        &state,
        auth_user.id,
        payload.receiver_id,
        &payload.content,
    )
    .await?;

    // Send message to receiver via WebSocket
    let timestamp = chrono::Utc::now();
    let ws_message = json!({
        "type": "message",
        "data": {
            "messageId": message_id,
            "senderId": auth_user.id,
            "receiverId": payload.receiver_id,
            "content": payload.content,
            "msgType": payload.msg_type,
            "timestamp": timestamp.to_rfc3339(),
            "status": "sent"
        }
    });

    if let Err(_) = state
        .ws_hub
        .read()
        .await
        .send_message_to_user(payload.receiver_id, &ws_message.to_string())
        .await
    {
        tracing::warn!("Failed to send WebSocket message to user {}", payload.receiver_id);
    }

    let response = ApiResponse::success_with_message(
        json!({
            "messageId": message_id,
            "timestamp": timestamp.to_rfc3339(),
            "status": "sent"
        }),
        "Message sent successfully",
    );

    Ok(Json(response))
}

// 标记消息为已读
pub async fn mark_chat_message_read(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(message_id): Path<u64>,
) -> Result<impl IntoResponse> {
    crate::repository::chat_repo::ChatRepo::mark_message_as_read(&state, message_id, auth_user.id)
        .await?;

    let response = ApiResponse::success_with_message(
        json!({
            "messageId": message_id,
            "readTime": chrono::Utc::now().to_rfc3339()
        }),
        "Message marked as read",
    );

    Ok(Json(response))
}

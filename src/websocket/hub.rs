use serde_json::json;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use tokio::sync::{broadcast, RwLock};

#[derive(Debug, Clone)]
pub struct WsUser {
    pub id: u64,
    pub username: String,
    pub sender: broadcast::Sender<String>,
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub id: Option<i64>,
    pub sender_id: u64,
    pub receiver_id: Option<u64>,
    pub room_id: Option<String>,
    pub content: String,
    pub msg_type: i8,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct WsRoom {
    pub id: String,
    pub name: String,
    pub users: HashMap<u64, WsUser>,
}

pub struct WsHub {
    rooms: RwLock<HashMap<String, WsRoom>>,
    users: RwLock<HashMap<u64, WsUser>>,
    message_sender: broadcast::Sender<String>,
    // 聊天记录缓冲区
    message_buffer: RwLock<Vec<ChatMessage>>,
    // 上次保存时间
    last_save_time: RwLock<Instant>,
    // 批量保存间隔（秒）
    save_interval: Duration,
    // 批量保存阈值
    save_threshold: usize,
}

impl WsHub {
    pub fn new() -> Self {
        let (message_sender, _) = broadcast::channel(1000);
        Self {
            rooms: RwLock::new(HashMap::new()),
            users: RwLock::new(HashMap::new()),
            message_sender,
            message_buffer: RwLock::new(Vec::new()),
            last_save_time: RwLock::new(Instant::now()),
            save_interval: Duration::from_secs(60), // 1分钟
            save_threshold: 100,                    // 100条消息
        }
    }

    pub async fn add_user(&self, user_id: u64, username: String) -> broadcast::Sender<String> {
        let (sender, _) = broadcast::channel(100);
        let ws_user = WsUser {
            id: user_id,
            username,
            sender: sender.clone(),
        };

        // 添加到用户列表
        self.users.write().await.insert(user_id, ws_user);

        // 添加到默认房间
        self.add_user_to_room("general", user_id).await;

        sender
    }

    pub async fn remove_user(&self, user_id: u64) {
        // 从所有房间中移除用户
        let rooms = self.rooms.read().await;
        for (_, room) in rooms.iter() {
            let mut users = room.users.clone();
            users.remove(&user_id);
        }

        // 从用户列表中移除
        self.users.write().await.remove(&user_id);
    }

    pub async fn add_user_to_room(&self, room_id: &str, user_id: u64) {
        let mut rooms = self.rooms.write().await;
        let users = self.users.read().await;

        if let Some(user) = users.get(&user_id) {
            let room = rooms.entry(room_id.to_string()).or_insert_with(|| WsRoom {
                id: room_id.to_string(),
                name: room_id.to_string(),
                users: HashMap::new(),
            });

            room.users.insert(
                user_id,
                WsUser {
                    id: user.id,
                    username: user.username.clone(),
                    sender: user.sender.clone(),
                },
            );
        }
    }

    pub async fn remove_user_from_room(&self, room_id: &str, user_id: u64) {
        let mut rooms = self.rooms.write().await;
        if let Some(room) = rooms.get_mut(room_id) {
            room.users.remove(&user_id);
        }
    }

    pub async fn send_message_to_room(
        &self,
        room_id: &str,
        message: &str,
    ) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        let rooms = self.rooms.read().await;
        if let Some(room) = rooms.get(room_id) {
            let mut sent_count = 0;
            for (_, user) in room.users.iter() {
                if let Err(_) = user.sender.send(message.to_string()) {
                    // 用户连接已断开，将在后续清理中移除
                } else {
                    sent_count += 1;
                }
            }
            Ok(sent_count)
        } else {
            Err("Room does not exist".into())
        }
    }

    pub async fn send_message_to_user(
        &self,
        user_id: u64,
        message: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let users = self.users.read().await;
        if let Some(user) = users.get(&user_id) {
            user.sender.send(message.to_string())?;
            Ok(())
        } else {
            Err("User does not exist".into())
        }
    }

    pub async fn broadcast_message(
        &self,
        message: &str,
    ) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        let users = self.users.read().await;
        let mut sent_count = 0;
        for (_, user) in users.iter() {
            if let Err(_) = user.sender.send(message.to_string()) {
                // 用户连接已断开，将在后续清理中移除
            } else {
                sent_count += 1;
            }
        }
        Ok(sent_count)
    }

    pub async fn get_room_users(&self, room_id: &str) -> Vec<WsUser> {
        let rooms = self.rooms.read().await;
        if let Some(room) = rooms.get(room_id) {
            room.users.values().cloned().collect()
        } else {
            vec![]
        }
    }

    pub async fn get_user_count(&self) -> usize {
        self.users.read().await.len()
    }

    pub async fn get_room_count(&self) -> usize {
        self.rooms.read().await.len()
    }

    /// 添加消息到缓冲区
    pub async fn add_message_to_buffer(&self, message: ChatMessage) {
        let mut buffer = self.message_buffer.write().await;
        buffer.push(message);
    }

    /// 检查是否需要保存消息
    pub async fn should_save_messages(&self) -> bool {
        let buffer = self.message_buffer.read().await;
        let last_save = self.last_save_time.read().await;

        // 检查消息数量或时间间隔
        buffer.len() >= self.save_threshold || last_save.elapsed() >= self.save_interval
    }

    /// 获取并清空消息缓冲区
    pub async fn flush_message_buffer(&self) -> Vec<ChatMessage> {
        let mut buffer = self.message_buffer.write().await;
        let mut last_save = self.last_save_time.write().await;

        let messages: Vec<ChatMessage> = buffer.drain(..).collect();
        *last_save = Instant::now(); // 更新保存时间

        messages
    }

    /// 强制保存所有缓冲的消息
    pub async fn force_save_messages(&self) -> Vec<ChatMessage> {
        let mut buffer = self.message_buffer.write().await;
        let mut last_save = self.last_save_time.write().await;

        let messages: Vec<ChatMessage> = buffer.drain(..).collect();
        *last_save = Instant::now(); // 更新保存时间

        tracing::info!("强制保存 {} 条聊天记录", messages.len());
        messages
    }

    /// 发送消息并添加到缓冲区
    pub async fn send_message_with_buffer(
        &self,
        sender_id: u64,
        receiver_id: Option<u64>,
        content: &str,
        msg_type: i8,
    ) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        // 创建聊天消息
        let chat_message = ChatMessage {
            id: None, // 数据库生成
            sender_id,
            receiver_id,
            room_id: Some("general".to_string()), // 默认房间
            content: content.to_string(),
            msg_type,
            created_at: chrono::Utc::now(),
        };

        // 添加到缓冲区
        self.add_message_to_buffer(chat_message.clone()).await;

        // 广播消息
        let message_json = json!({
            "type": "message",
            "data": {
                "id": chat_message.id,
                "senderId": chat_message.sender_id,
                "receiverId": chat_message.receiver_id,
                "content": chat_message.content,
                "msgType": chat_message.msg_type,
                "createdAt": chat_message.created_at.to_rfc3339(),
                "timestamp": chat_message.created_at.timestamp_millis()
            }
        });

        self.broadcast_message(&message_json.to_string()).await
    }

    pub async fn cleanup_disconnected_users(&self) {
        // 清理断开连接的用户
        let mut users = self.users.write().await;
        let mut rooms = self.rooms.write().await;

        users.retain(|_, user| {
            // 检查发送器是否仍然有效
            user.sender.receiver_count() > 0
        });

        // 从房间中移除断开连接的用户
        for (_, room) in rooms.iter_mut() {
            room.users.retain(|user_id, _| users.contains_key(user_id));
        }
    }

    /// 获取当前缓冲区消息数量
    pub async fn get_buffer_size(&self) -> usize {
        self.message_buffer.read().await.len()
    }

    /// 获取距离下次保存的时间
    pub async fn get_time_until_next_save(&self) -> Duration {
        let last_save = self.last_save_time.read().await;
        let elapsed = last_save.elapsed();

        if elapsed >= self.save_interval {
            Duration::ZERO
        } else {
            self.save_interval - elapsed
        }
    }
}

impl Default for WsHub {
    fn default() -> Self {
        Self::new()
    }
}

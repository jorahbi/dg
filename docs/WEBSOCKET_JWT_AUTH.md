# WebSocket JWT 认证指南

## 概述

本项目实现了完整的WebSocket JWT认证机制，支持两种连接方式：
- **认证WebSocket**: 需要JWT token验证的聊天连接
- **公开WebSocket**: 用于登录和注册等不需要认证的连接

## 认证流程

### 1. 用户获取JWT Token

用户首先需要通过登录接口获取JWT token：

```bash
POST /api/auth/login
Content-Type: application/json

{
  "username": "your_username",
  "password": "your_password"
}
```

成功响应：
```json
{
  "code": 200,
  "message": "登录成功",
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "user": {
      "id": 123,
      "username": "your_username",
      "user_level": 1,
      "is_kyc_verified": false
    }
  }
}
```

### 2. WebSocket连接认证

#### 方式1: 通过查询参数传递JWT

```javascript
// 使用获取的JWT token连接认证WebSocket
const token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";
const ws = new WebSocket(`ws://localhost:8080/ws/chat?token=${encodeURIComponent(token)}`);

ws.onopen = function(event) {
    console.log("WebSocket连接已建立，用户已认证");

    // 发送消息
    ws.send(JSON.stringify({
        type: "SendMessage",
        conversation_id: 1,
        content: "Hello, this is a test message",
        message_type: "text"
    }));
};

ws.onmessage = function(event) {
    const data = JSON.parse(event.data);
    console.log("收到消息:", data);
};

ws.onerror = function(error) {
    console.error("WebSocket错误:", error);
};

ws.onclose = function(event) {
    console.log("WebSocket连接已关闭");
};
```

#### 方式2: 使用公开WebSocket进行登录/注册

```javascript
// 连接公开WebSocket（不需要JWT）
const ws = new WebSocket("ws://localhost:8080/ws/public");

ws.onopen = function(event) {
    console.log("公开WebSocket连接已建立");

    // 通过WebSocket登录
    ws.send(JSON.stringify({
        type: "Login",
        username: "your_username",
        password: "your_password"
    }));
};

ws.onmessage = function(event) {
    const data = JSON.parse(event.data);

    if (data.type === "AuthSuccess") {
        console.log("登录成功，收到token:", data.token);

        // 使用获取的token连接认证WebSocket
        const authenticatedWs = new WebSocket(
            `ws://localhost:8080/ws/chat?token=${encodeURIComponent(data.token)}`
        );
        // ... 处理认证WebSocket连接
    }
};
```

## JWT Payload 结构

JWT token包含以下用户信息：

```json
{
  "sub": "123",                    // 用户ID
  "username": "your_username",     // 用户名
  "user_level": 1,                // 用户等级
  "is_kyc_verified": false,        // KYC认证状态
  "exp": 1704067200,              // 过期时间
  "iat": 1703980800,              // 签发时间
  "iss": "coin-dgai-api",         // 签发者
  "aud": "coin-dgai-users",       // 受众
  "jti": "unique-jwt-id",         // JWT ID
  "session_id": "session_123"     // 会话ID
}
```

## WebSocket 消息类型

### 认证相关消息

#### Login (仅公开WebSocket)
```json
{
  "type": "Login",
  "username": "your_username",
  "password": "your_password"
}
```

#### Register (仅公开WebSocket)
```json
{
  "type": "Register",
  "username": "new_user",
  "password": "password123",
  "email": "user@example.com"
}
```

#### AuthSuccess (服务器响应)
```json
{
  "type": "AuthSuccess",
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": 123,
    "username": "your_username",
    "user_level": 1,
    "is_kyc_verified": false
  }
}
```

### 聊天相关消息

#### SendMessage (需要认证)
```json
{
  "type": "SendMessage",
  "conversation_id": 1,
  "content": "Hello, world!",
  "message_type": "text",
  "file_url": null,
  "file_name": null
}
```

#### MarkRead (需要认证)
```json
{
  "type": "MarkRead",
  "conversation_id": 1,
  "message_ids": ["msg1", "msg2"]
}
```

#### Typing (需要认证)
```json
{
  "type": "Typing",
  "conversation_id": 1,
  "is_typing": true
}
```

#### NewMessage (服务器推送)
```json
{
  "type": "NewMessage",
  "conversation_id": 1,
  "message": {
    "id": 456,
    "conversation_id": 1,
    "sender_id": 123,
    "sender_type": "user",
    "message_id": "uuid-string",
    "content": "Hello, world!",
    "message_type": "text",
    "status": "sent",
    "read_at": null,
    "file_url": null,
    "file_name": null,
    "file_size": null,
    "metadata": null,
    "created_at": "2023-12-30T10:30:00Z",
    "updated_at": "2023-12-30T10:30:00Z"
  }
}
```

## 错误处理

### 认证错误

如果JWT token无效或过期，服务器会发送错误消息：

```json
{
  "type": "Error",
  "code": "UNAUTHORIZED",
  "message": "Token has expired"
}
```

常见认证错误代码：
- `UNAUTHORIZED`: 缺少或无效的认证token
- `TOKEN_EXPIRED`: token已过期
- `INVALID_TOKEN`: token格式无效
- `AUTH_ERROR`: 其他认证错误

### 操作错误

```json
{
  "type": "Error",
  "code": "AUTH_REQUIRED",
  "message": "Authentication required for this operation"
}
```

## 安全最佳实践

### 1. Token 存储
```javascript
// 推荐：使用HttpOnly cookie存储JWT
document.cookie = `token=${token}; HttpOnly; Secure; SameSite=Strict`;

// 或者：使用localStorage（注意XSS风险）
localStorage.setItem('jwt_token', token);
```

### 2. Token 刷新
```javascript
// 监听token过期
ws.onmessage = function(event) {
    const data = JSON.parse(event.data);

    if (data.type === "Error" && data.code === "TOKEN_EXPIRED") {
        // 使用refresh token刷新JWT
        refreshTokenAndReconnect();
    }
};

async function refreshTokenAndReconnect() {
    const refreshToken = localStorage.getItem('refresh_token');

    try {
        const response = await fetch('/api/auth/refresh', {
            method: 'POST',
            headers: {
                'Authorization': `Bearer ${refreshToken}`
            }
        });

        const data = await response.json();
        const newToken = data.data.token;

        // 保存新token并重新连接
        localStorage.setItem('jwt_token', newToken);
        reconnectWebSocket(newToken);
    } catch (error) {
        console.error('Token刷新失败:', error);
        // 跳转到登录页面
        window.location.href = '/login';
    }
}
```

### 3. 连接状态管理
```javascript
class WebSocketManager {
    constructor() {
        this.ws = null;
        this.reconnectAttempts = 0;
        this.maxReconnectAttempts = 5;
        this.reconnectInterval = 1000;
    }

    connect(token) {
        this.ws = new WebSocket(`ws://localhost:8080/ws/chat?token=${encodeURIComponent(token)}`);

        this.ws.onopen = () => {
            console.log('WebSocket连接成功');
            this.reconnectAttempts = 0;
        };

        this.ws.onclose = () => {
            console.log('WebSocket连接关闭');
            this.scheduleReconnect();
        };

        this.ws.onerror = (error) => {
            console.error('WebSocket错误:', error);
        };
    }

    scheduleReconnect() {
        if (this.reconnectAttempts < this.maxReconnectAttempts) {
            setTimeout(() => {
                this.reconnectAttempts++;
                console.log(`尝试重新连接 (${this.reconnectAttempts}/${this.maxReconnectAttempts})`);
                const token = localStorage.getItem('jwt_token');
                this.connect(token);
            }, this.reconnectInterval * this.reconnectAttempts);
        }
    }

    send(message) {
        if (this.ws && this.ws.readyState === WebSocket.OPEN) {
            this.ws.send(JSON.stringify(message));
        } else {
            console.error('WebSocket未连接');
        }
    }
}

// 使用示例
const wsManager = new WebSocketManager();
const token = localStorage.getItem('jwt_token');
wsManager.connect(token);
```

## 环境配置

在开发环境中，可以通过以下环境变量调整JWT配置：

```bash
# JWT密钥（生产环境请使用强密钥）
JWT_SECRET=your_super_secret_key_here

# Token过期时间（秒）
JWT_EXPIRATION=86400  # 24小时

# 刷新Token过期时间（秒）
JWT_REFRESH_EXPIRATION=604800  # 7天

# 签发者和受众
JWT_ISSUER=coin-dgai-api
JWT_AUDIENCE=coin-dgai-users
```

## 测试示例

### 使用Postman测试WebSocket

1. 建立WebSocket连接：
   - URL: `ws://localhost:8080/ws/chat?token=YOUR_JWT_TOKEN`

2. 发送消息：
   ```json
   {
     "type": "SendMessage",
     "conversation_id": 1,
     "content": "Test message",
     "message_type": "text"
   }
   ```

3. 接收响应：
   - 服务器会返回相同格式的消息，包含完整的消息信息和生成的数据库ID

### 使用curl测试登录并获取JWT

```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "testpass"
  }'
```

## 故障排除

### 常见问题

1. **"Missing authorization token"错误**
   - 确保在WebSocket URL中包含token参数
   - 检查token是否正确编码

2. **"Token has expired"错误**
   - 使用refresh token获取新的JWT
   - 重新登录获取新token

3. **"Invalid token"错误**
   - 检查token格式是否正确
   - 确认token未被篡改

4. **连接立即断开**
   - 检查网络连接
   - 确认服务器端点地址正确
   - 查看服务器日志获取详细错误信息

### 调试技巧

```javascript
// 启用详细日志
const ws = new WebSocket("ws://localhost:8080/ws/chat?token=YOUR_TOKEN");

ws.onopen = () => console.log("连接已建立");
ws.onclose = (event) => console.log("连接已关闭:", event.code, event.reason);
ws.onerror = (error) => console.error("WebSocket错误:", error);

// 监听所有消息类型
ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    console.log("收到消息:", data.type, data);
};
```

通过以上指南，您可以完全理解和使用本项目的WebSocket JWT认证机制。
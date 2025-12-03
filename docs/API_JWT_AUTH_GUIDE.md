# API JWT 认证指南

## 概述

本项目的所有API接口（除了登录和注册）都需要JWT认证。系统采用分层认证策略，确保接口安全性和用户体验的平衡。

## 认证策略

### 🔓 无需认证的接口

**认证接口:**
- `POST /api/auth/register` - 用户注册
- `POST /api/auth/login` - 用户登录
- `POST /api/auth/logout` - 用户登出

**公开接口:**
- `GET /api/public/charts/market-data` - 市场数据（公开）
- `GET /api/public/charts/dashboard-stats` - 仪表板统计（公开）

**WebSocket:**
- `WS /ws/public` - 公开WebSocket（用于登录/注册等）

**健康检查:**
- `GET /health` - 服务健康状态

### 🔒 需要JWT认证的接口

除了上述接口外，所有其他API都需要在请求头中包含有效的JWT token。

## JWT Token 获取流程

### 1. 用户注册

```bash
POST /api/auth/register
Content-Type: application/json

{
  "username": "your_username",
  "password": "your_password",
  "email": "your_email@example.com"
}
```

成功响应：
```json
{
  "code": 200,
  "message": "注册成功",
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

### 2. 用户登录

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

## JWT Token 使用方法

### HTTP请求头认证

在所有需要认证的请求中，添加 `Authorization` 头部：

```bash
GET /api/user/info
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
Content-Type: application/json
```

### 错误响应示例

如果未提供token或token无效，会返回401错误：

```json
{
  "code": 401,
  "message": "Missing authorization token"
}
```

```json
{
  "code": 401,
  "message": "Token has expired"
}
```

## API接口分类

### 🔓 公开接口

#### 认证相关
- `POST /api/auth/register` - 用户注册
- `POST /api/auth/login` - 用户登录
- `POST /api/auth/logout` - 用户登出

#### 公开数据
- `GET /api/public/charts/market-data` - 市场数据
- `GET /api/public/charts/dashboard-stats` - 仪表板统计
- `GET /health` - 健康检查

#### WebSocket
- `WS /ws/public` - 公开WebSocket连接

### 🔒 需要JWT认证的接口

#### 用户管理 (`/api/user/*`)
- `GET /api/user/info` - 获取用户信息
- `POST /api/user/password` - 修改密码
- `POST /api/user/avatar` - 上传头像

#### 聊天系统 (`/api/chat/*`)
- `GET /api/chat/conversations` - 获取会话列表
- `POST /api/chat/conversations` - 创建新会话
- `GET /api/chat/conversations/:id/messages` - 获取会话消息
- `POST /api/chat/conversations/:id/messages` - 发送消息

#### 消息管理 (`/api/messages/*`)
- `GET /api/messages/` - 获取消息列表
- `POST /api/messages/:id/read` - 标记消息已读
- `POST /api/messages/read-all` - 标记所有消息已读

#### 空投系统 (`/api/airdrops/*`)
- `GET /api/airdrops/` - 获取空投活动列表
- `POST /api/airdrops/claim` - 参与空投
- `GET /api/airdrops/history` - 获取空投历史

#### 算力管理 (`/api/power/*`)
- `GET /api/power/packages` - 获取算力包列表
- `POST /api/power/packages/purchase` - 购买算力包
- `GET /api/power/overview` - 获取算力概览
- `GET /api/power/packages/list` - 获取用户算力包
- `GET /api/power/earnings` - 获取算力收益

#### 资产管理 (`/api/assets/*`)
- `GET /api/assets/overview` - 获取资产概览
- `GET /api/assets/list` - 获取用户资产
- `GET /api/assets/history` - 获取资产历史
- `GET /api/assets/deposit/:currency` - 获取充值地址
- `POST /api/assets/withdraw/:currency` - 申请提现
- `GET /api/assets/network/:currency` - 获取网络信息

#### 邀请系统 (`/api/invite/*`)
- `GET /api/invite/code` - 获取邀请码
- `GET /api/invite/stats` - 获取邀请统计
- `GET /api/invite/history` - 获取邀请历史
- `GET /api/invite/ranking` - 获取邀请排行榜
- `POST /api/invite/rewards/process` - 处理邀请奖励

#### 任务系统 (`/api/tasks/*`)
- `GET /api/tasks/` - 获取任务列表
- `POST /api/tasks/start` - 开始任务
- `POST /api/tasks/accelerate` - 加速任务
- `POST /api/tasks/claim` - 领取任务奖励
- `GET /api/tasks/stats` - 获取任务统计
- `GET /api/tasks/progress/:user_task_id` - 获取任务进度

#### KYC认证 (`/api/kyc/*`)
- `GET /api/kyc/status` - 获取KYC状态
- `POST /api/kyc/application` - 提交KYC申请
- `GET /api/kyc/application` - 获取KYC申请
- `POST /api/kyc/upload/:document_type` - 上传KYC文档
- `GET /api/kyc/stats` - 获取KYC统计
- `POST /api/kyc/verify/:application_id` - 模拟KYC验证

#### 图表数据 (`/api/charts/*`)
- `GET /api/charts/assets` - 用户资产图表
- `GET /api/charts/power` - 算力收益图表
- `GET /api/charts/tasks` - 任务完成图表
- `GET /api/charts/invites` - 邀请表现图表
- `GET /api/charts/market` - 市场图表
- `GET /api/charts/dashboard` - 仪表板概览

#### 限时礼包 (`/api/packages/*`)
- `GET /api/packages/` - 获取特殊礼包
- `GET /api/packages/detail/:package_id` - 获取礼包详情
- `POST /api/packages/purchase` - 购买礼包
- `GET /api/packages/user` - 获取用户礼包
- `POST /api/packages/activate/:purchase_id` - 激活礼包
- `GET /api/packages/stats` - 获取礼包统计

#### 内容管理 (`/api/content/*`)
- `GET /api/content/carousels` - 获取轮播图
- `POST /api/content/carousels/click` - 轮播图点击
- `GET /api/content/banners` - 获取横幅
- `POST /api/content/banners/click` - 横幅点击
- `GET /api/content/announcements` - 获取公告
- `POST /api/content/announcements/read` - 标记公告已读
- `GET /api/content/platform-stats` - 获取平台统计
- `GET /api/content/analytics` - 获取内容分析

#### WebSocket认证
- `WS /ws/chat?token=JWT_TOKEN` - 认证聊天WebSocket

## 客户端集成示例

### JavaScript/TypeScript

```typescript
// API客户端类
class ApiClient {
    private baseUrl: string;
    private token: string | null = null;

    constructor(baseUrl: string) {
        this.baseUrl = baseUrl;
    }

    // 设置认证token
    setToken(token: string) {
        this.token = token;
    }

    // 获取认证头
    private getAuthHeaders(): HeadersInit {
        const headers: HeadersInit = {
            'Content-Type': 'application/json',
        };

        if (this.token) {
            headers['Authorization'] = `Bearer ${this.token}`;
        }

        return headers;
    }

    // 登录
    async login(username: string, password: string) {
        const response = await fetch(`${this.baseUrl}/api/auth/login`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ username, password }),
        });

        const data = await response.json();

        if (data.code === 200) {
            this.setToken(data.data.token);
            return data.data;
        } else {
            throw new Error(data.message);
        }
    }

    // 认证API调用示例
    async getUserInfo() {
        if (!this.token) {
            throw new Error('Not authenticated');
        }

        const response = await fetch(`${this.baseUrl}/api/user/info`, {
            headers: this.getAuthHeaders(),
        });

        return response.json();
    }

    // WebSocket连接示例
    connectWebSocket() {
        if (!this.token) {
            throw new Error('Not authenticated');
        }

        const ws = new WebSocket(`${this.baseUrl.replace('http', 'ws')}/ws/chat?token=${this.token}`);

        ws.onopen = () => {
            console.log('WebSocket连接已建立');
        };

        ws.onmessage = (event) => {
            const data = JSON.parse(event.data);
            console.log('收到WebSocket消息:', data);
        };

        return ws;
    }
}

// 使用示例
const apiClient = new ApiClient('http://localhost:8080');

// 登录
await apiClient.login('username', 'password');

// 获取用户信息
const userInfo = await apiClient.getUserInfo();

// 连接WebSocket
const ws = apiClient.connectWebSocket();
```

### Python

```python
import requests
import json
import websocket
from typing import Optional

class ApiClient:
    def __init__(self, base_url: str):
        self.base_url = base_url
        self.token: Optional[str] = None

    def set_token(self, token: str):
        self.token = token

    def get_auth_headers(self) -> dict:
        headers = {
            'Content-Type': 'application/json',
        }

        if self.token:
            headers['Authorization'] = f'Bearer {self.token}'

        return headers

    def login(self, username: str, password: str):
        response = requests.post(
            f"{self.base_url}/api/auth/login",
            json={"username": username, "password": password},
            headers={'Content-Type': 'application/json'}
        )

        data = response.json()

        if data.get('code') == 200:
            self.set_token(data['data']['token'])
            return data['data']
        else:
            raise Exception(data.get('message', 'Login failed'))

    def get_user_info(self):
        if not self.token:
            raise Exception('Not authenticated')

        response = requests.get(
            f"{self.base_url}/api/user/info",
            headers=self.get_auth_headers()
        )

        return response.json()

    def connect_websocket(self):
        if not self.token:
            raise Exception('Not authenticated')

        ws_url = f"{self.base_url.replace('http', 'ws')}/ws/chat?token={self.token}"
        ws = websocket.WebSocketApp(ws_url)

        def on_message(ws, message):
            data = json.loads(message)
            print(f"收到消息: {data}")

        def on_open(ws):
            print("WebSocket连接已建立")

        ws.on_message = on_message
        ws.on_open = on_open

        return ws

# 使用示例
client = ApiClient('http://localhost:8080')
client.login('username', 'password')
user_info = client.get_user_info()
ws = client.connect_websocket()
```

## 错误代码说明

| 错误代码 | HTTP状态码 | 描述 | 解决方案 |
|---------|-----------|------|----------|
| 401 | 401 | Missing authorization token | 在请求头中添加Bearer token |
| 401 | 401 | Token has expired | 使用refresh token或重新登录 |
| 401 | 401 | Invalid token | 检查token格式和内容 |
| 403 | 403 | KYC verification required | 完成KYC认证 |
| 403 | 403 | Insufficient user level | 提升用户等级 |

## 安全最佳实践

### 1. Token 存储
```javascript
// 推荐：使用HttpOnly cookie
document.cookie = `token=${token}; HttpOnly; Secure; SameSite=Strict`;

// 避免：明文存储在localStorage（容易受到XSS攻击）
// localStorage.setItem('token', token);
```

### 2. Token 刷新
```javascript
class TokenManager {
    async refreshIfNeeded() {
        const token = this.getToken();

        // 检查token是否即将过期（30分钟内）
        if (this.isTokenExpiringSoon(token)) {
            try {
                const newToken = await this.refreshToken();
                this.setToken(newToken);
            } catch (error) {
                // 刷新失败，跳转到登录页面
                window.location.href = '/login';
            }
        }
    }
}
```

### 3. 自动添加认证头
```javascript
// 使用axios拦截器
axios.interceptors.request.use((config) => {
    const token = localStorage.getItem('jwt_token');
    if (token) {
        config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
});

axios.interceptors.response.use(
    (response) => response,
    (error) => {
        if (error.response?.status === 401) {
            // token过期，跳转到登录页面
            window.location.href = '/login';
        }
        return Promise.reject(error);
    }
);
```

## 开发调试

### 1. 获取测试Token

```bash
# 登录获取token
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"testuser","password":"testpass"}'
```

### 2. 测试认证接口

```bash
# 使用token调用认证接口
curl -X GET http://localhost:8080/api/user/info \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

### 3. 测试WebSocket

```javascript
// 测试WebSocket连接
const token = 'YOUR_JWT_TOKEN';
const ws = new WebSocket(`ws://localhost:8080/ws/chat?token=${token}`);
```

通过以上配置，您的API将具备完整的JWT认证保护，确保只有经过身份验证的用户才能访问敏感接口。
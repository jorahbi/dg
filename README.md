# Astra AI API - AI算力挖矿平台后端服务

Astra AI是一个基于Rust构建的高性能AI算力挖矿平台后端API服务，为全球用户提供加密货币挖矿、空投参与、任务管理和资产管理等综合服务。项目采用现代化技术栈，注重安全性、性能和可扩展性，致力于打造稳定可靠的Web3基础设施。

## 技术栈

- **核心语言**: Rust (Edition 2021)
- **Web框架**: Axum 0.7 + Tokio异步运行时
- **数据库**: MySQL 8.0+ (通过SQLx ORM)
- **认证**: JWT令牌认证 + bcrypt密码加密
- **实时通信**: WebSocket
- **配置管理**: 环境变量 + 配置结构体
- **日志**: tracing + tracing-subscriber
- **序列化**: Serde + serde_json

## 项目结构

```
api/
├── src/
│   ├── main.rs                  # 程序入口
│   ├── app.rs                   # 路由组装和应用配置
│   ├── lib.rs                   # 库入口
│   ├── config.rs                # 配置管理和加载
│   ├── error.rs                 # 统一错误类型
│   ├── state.rs                 # 应用状态管理
│   ├── extract/                 # 请求参数提取器
│   │   └── auth.rs              # JWT认证提取器
│   ├── middleware/              # 中间件
│   │   ├── auth.rs              # 认证中间件
│   │   └── cors.rs              # CORS中间件
│   ├── handler/                 # HTTP请求处理器
│   │   ├── mod.rs
│   │   ├── auth.rs              # 认证相关接口
│   │   ├── user.rs              # 用户管理接口
│   │   ├── power.rs             # 算力管理接口
│   │   ├── airdrop.rs           # 空投活动接口
│   │   └── ...                  # 其他模块处理器
│   ├── service/                 # 业务逻辑服务层
│   │   ├── mod.rs
│   │   ├── auth.rs              # 认证服务
│   │   └── ...                  # 其他业务服务
│   ├── repository/              # 数据访问层
│   │   ├── mod.rs
│   │   ├── user_repo.rs         # 用户数据访问
│   │   └── ...                  // 其他数据访问
│   ├── model/                   # 数据模型
│   │   ├── mod.rs
│   │   ├── user.rs              # 用户模型
│   │   ├── power.rs             # 算力模型
│   │   └── ...                  // 其他数据模型
│   ├── schema/                  # 请求/响应DTO
│   │   ├── mod.rs
│   │   ├── user.rs              # 用户相关Schema
│   │   ├── common.rs            # 通用Schema
│   │   └── ...                  // 其他Schema
│   ├── utils/                   # 工具函数
│   │   ├── mod.rs
│   │   ├── jwt.rs               # JWT工具
│   │   ├── password.rs          // 密码工具
│   │   └── file_upload.rs       // 文件上传工具
│   └── websocket/               # WebSocket实时通信
│       ├── mod.rs
│       ├── hub.rs               // WebSocket Hub
│       └── room.rs              // 房间管理
├── migrations/                  # 数据库迁移文件
│   ├── 001_create_tables.sql    # 建表SQL
│   └── 002_insert_initial_data.sql # 初始数据
├── config/                      # 配置文件目录
├── docs/                        # 项目文档
├── scripts/                     # 脚本文件
├── tests/                       # 测试文件
├── Cargo.toml                   # 项目配置
├── Dockerfile                   # Docker配置
├── docker-compose.yml           # Docker Compose配置
├── .env.example                 # 环境变量示例
└── README.md                    # 项目说明
```

## 核心功能模块

### 1. 认证管理模块
- 用户注册/登录/登出
- JWT令牌认证
- 密码重置（基于安全问题）
- 账户安全管理

### 2. 用户管理模块
- 用户信息管理
- 头像上传
- 资料修改
- 权限管理

### 3. 算力管理模块
- 算力等级系统
- 算力包购买
- 收益统计
- 提现管理

### 4. 空投活动模块
- 多种空投类型
- 实时抢空投
- 空投历史记录
- 资格验证

### 5. 任务管理模块
- 任务发布与执行
- 任务加速
- 收益结算
- 进度跟踪

### 6. 资产中心模块
- 多币种支持
- 充值/提现
- 货币兑换
- 交易记录

### 7. 实时通信模块
- WebSocket聊天
- 消息推送
- 房间管理
- 在线状态

### 8. KYC认证模块
- 身份证上传
- 认证状态管理
- 审核流程

## 🛠️ 技术栈

- **语言**: Rust 1.70+
- **Web框架**: Axum 0.7
- **数据库**: MySQL 8.0+ (通过 SQLx)
- **缓存**: Redis 6.0+
- **认证**: JWT (jsonwebtoken)
- **序列化**: Serde
- **异步运行时**: Tokio
- **日志**: tracing
- **文件上传**: multipart support
- **邮件**: lettre

## 📁 项目结构

```
src/
├── app.rs              # 应用程序入口和路由配置
├── config.rs           # 配置管理
├── error.rs            # 错误处理和响应格式
├── main.rs             # 主程序入口
├── middleware/         # 中间件
│   ├── auth.rs        # 认证中间件
│   └── mod.rs
├── model/              # 数据模型
│   ├── user.rs        # 用户相关模型
│   ├── message.rs     # 消息模型
│   ├── chat.rs        # 聊天模型
│   └── mod.rs
├── handler/            # 请求处理器
│   ├── auth.rs        # 认证相关
│   ├── user.rs        # 用户相关
│   ├── chat.rs        # 聊天相关
│   ├── message.rs     # 消息相关
│   └── mod.rs
├── utils/              # 工具函数
│   ├── jwt.rs         # JWT工具
│   ├── password.rs    # 密码工具
│   ├── file_upload.rs # 文件上传工具
│   ├── email.rs       # 邮件工具
│   ├── validation.rs  # 验证工具
│   └── mod.rs
└── websocket/          # WebSocket相关
    ├── mod.rs
    └── handler.rs
```

## 🚀 快速开始

### 环境要求

- Rust 1.70+
- MySQL 8.0+
- Redis 6.0+
- Git

### 安装步骤

1. **克隆项目**
   ```bash
   git clone <repository-url>
   cd api
   ```

2. **安装 Rust**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. **配置环境变量**
   ```bash
   cp .env.example .env
   # 编辑 .env 文件，配置数据库连接等
   ```

4. **启动依赖服务**
   ```bash
   # 启动 MySQL
   mysql -u root -p

   # 启动 Redis
   redis-server
   ```

5. **创建数据库**
   ```sql
   CREATE DATABASE coin_dgai CHARACTER SET utf8mb4 COLLATE utf8mb4_bin;
   ```

6. **运行迁移**
   ```bash
   # 如果使用 sqlx-cli
   sqlx database create --database-url "mysql://root:password@localhost:3306/coin_dgai"
   sqlx migrate run --database-url "mysql://root:password@localhost:3306/coin_dgai"
   ```

7. **启动服务**
   ```bash
   cargo run
   ```

## 📡 API 文档

### 认证接口

#### 用户注册
```http
POST /api/auth/register
Content-Type: application/json

{
  "username": "testuser",
  "password": "Password123!",
  "confirmPassword": "Password123!",
  "email": "test@example.com",
  "inviteCode": "INVITE123"
}
```

#### 用户登录
```http
POST /api/auth/login
Content-Type: application/json

{
  "username": "testuser",
  "password": "Password123!"
}
```

### 用户接口

#### 获取用户信息
```http
GET /api/user/info
Authorization: Bearer <token>
```

#### 更新密码
```http
POST /api/user/password
Authorization: Bearer <token>
Content-Type: application/json

{
  "currentPassword": "OldPassword123!",
  "newPassword": "NewPassword456!"
}
```

### 聊天接口

#### 创建会话
```http
POST /api/chat/conversations
Authorization: Bearer <token>
Content-Type: application/json

{
  "title": "技术支持咨询",
  "initialMessage": "我需要帮助解决技术问题"
}
```

#### 发送消息
```http
POST /api/chat/conversations/{id}/messages
Authorization: Bearer <token>
Content-Type: application/json

{
  "content": "我遇到了一个登录问题",
  "messageType": "text"
}
```

## 🔧 配置说明

### 环境变量

| 变量名 | 描述 | 默认值 |
|--------|------|--------|
| `APP_SERVER__HOST` | 服务器地址 | `0.0.0.0` |
| `APP_SERVER__PORT` | 服务器端口 | `8080` |
| `APP_DATABASE__URL` | 数据库连接URL | `mysql://root:password@localhost:3306/coin_dgai` |
| `APP_REDIS__URL` | Redis连接URL | `redis://localhost:6379` |
| `APP_JWT__SECRET` | JWT密钥 | `your-super-secret-jwt-key` |
| `APP_UPLOAD__MAX_FILE_SIZE` | 最大文件大小 | `5242880` (5MB) |

### 数据库配置

项目使用 SQLx 进行数据库操作，支持类型安全的 SQL 查询和迁移。

```rust
// 示例查询
let user = sqlx::query_as!(
    User,
    "SELECT * FROM users WHERE username = ? AND is_active = true",
    username
)
.fetch_one(&pool)
.await?;
```

## 🔒 安全特性

- **密码安全**: bcrypt 加盐哈希
- **JWT认证**: 无状态令牌认证
- **限流保护**: API 请求频率限制
- **输入验证**: 严格的输入验证和清理
- **SQL注入防护**: 参数化查询
- **CORS配置**: 跨域请求安全控制

## 🧪 测试

运行测试：

```bash
# 运行所有测试
cargo test

# 运行特定模块测试
cargo test auth
cargo test user
cargo test chat

# 运行测试并显示输出
cargo test -- --nocapture
```

## 📊 性能优化

- **连接池**: 数据库连接池管理
- **缓存策略**: Redis 缓存热点数据
- **异步处理**: 全异步 I/O 处理
- **批量操作**: 支持批量数据库操作
- **索引优化**: 数据库索引优化

## 📚 开发文档

### 多语言数据转换模式
项目实现了完整的多语言数据转换方案，支持数据库 JSON 字段到前端字符串的国际化转换。

- **[完整文档](docs/multilingual-data-conversion-pattern.md)** - 详细实现说明和最佳实践
- **[快速模板](docs/i18n-conversion-template.rs)** - 可直接复用的代码模板
- **[参考卡片](docs/i18n-conversion-cheatsheet.md)** - 快速参考和关键点

**核心特性**:
- 类型安全的 JSON 多语言字段转换
- 智能语言回退机制（指定语言 → 英文 → 默认值）
- 批量数据转换支持
- 完整的错误处理和默认值
- 高性能的批量转换函数

**使用示例**:
```rust
// API Handler 中的使用
let records = convert_user_power_records(db_records, &auth_user.lang);
```

## 📝 日志

项目使用 `tracing` 进行结构化日志：

```rust
tracing::info!("User registered: {}", user_id);
tracing::error!("Database error: {}", error);
tracing::warn!("Rate limit exceeded for IP: {}", ip);
```

## 🤝 贡献指南

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🆘 支持

如有问题或建议，请：

1. 查看 [Issues](https://github.com/your-repo/issues) 页面
2. 创建新的 Issue
3. 联系开发团队

---

© 2025 Astra Ai. All rights reserved.
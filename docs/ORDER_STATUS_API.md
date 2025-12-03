# 订单状态修改 API 接口文档

## 概述

根据 CODING_CONVENTIONS.md 规范，本文档描述新增的订单状态修改接口，使用事务同时更新 `orders` 和 `user_power_records` 表的状态。

## 接口列表

### 1. 更新订单状态

**接口**: `POST /api/purchase/orders/status`

**描述**: 使用事务更新订单状态和对应的算力记录状态

**请求头**:
```
Content-Type: application/json
Authorization: Bearer <jwt_token>
```

**请求体**:
```json
{
  "orderId": "ORD123456789",
  "status": "paid",
  "transactionHash": "0x1234567890abcdef1234567890abcdef12345678"
}
```

**请求参数**:
| 参数名 | 类型 | 必填 | 描述 |
|--------|------|------|------|
| orderId | string | 是 | 订单唯一标识符 |
| status | string | 是 | 新的订单状态 (pending/paid/completed/cancelled/expired) |
| transactionHash | string | 否 | 区块链交易哈希，支付完成时必填 |

**状态映射规则**:
| 订单状态 | 算力状态 | 说明 |
|----------|----------|------|
| pending | no_pay | 待支付 → 未支付 |
| paid | active | 已支付 → 激活 |
| completed | expired | 已完成 → 过期 |
| cancelled | cancelled | 已取消 → 取消 |

**响应**:
```json
{
  "code": 200,
  "message": "订单状态已更新为: paid",
  "data": {
    "order_id": "ORD123456789",
    "old_status": "pending",
    "new_status": "paid",
    "updated_at": 1701234567890,
    "transaction_hash": "0x1234567890abcdef1234567890abcdef12345678"
  }
}
```

### 2. 完成订单支付

**接口**: `POST /api/purchase/orders/:orderId/complete`

**描述**: 完成订单支付，自动激活对应的算力记录

**请求头**:
```
Content-Type: application/json
Authorization: Bearer <jwt_token>
```

**请求体**:
```json
{
  "transactionHash": "0x1234567890abcdef1234567890abcdef12345678"
}
```

**响应**:
```json
{
  "code": 200,
  "message": "订单支付完成",
  "data": {
    "orderId": "ORD123456789",
    "status": "paid",
    "paidAt": 1701234567890,
    "transactionHash": "0x1234567890abcdef1234567890abcdef12345678",
    "message": "订单支付已完成，算力已激活"
  }
}
```

### 3. 完成订单

**接口**: `POST /api/purchase/orders/:orderId/finish`

**描述**: 完成订单生命周期，设置对应的算力记录为过期状态

**请求头**:
```
Content-Type: application/json
Authorization: Bearer <jwt_token>
```

**请求体**: 无

**响应**:
```json
{
  "code": 200,
  "message": "订单完成",
  "data": {
    "orderId": "ORD123456789",
    "status": "completed",
    "completedAt": 1701234567890,
    "message": "订单已完成，算力已到期"
  }
}
```

## 数据库事务操作

### 事务处理流程

```rust
async fn update_order_and_power_status_in_tx(
    tx: &mut sqlx::Transaction<'_, MySql>,
    order_id: &str,
    new_order_status: &str,
    new_power_status: &str,
    transaction_hash: Option<String>,
) -> Result<()> {
    // 1. 更新 orders 表
    sqlx::query!(
        r#"
        UPDATE orders
        SET status = ?,
            transaction_hash = ?,
            is_paid = ?,
            updated_at = ?
        WHERE order_id = ?
        "#,
        new_order_status,
        transaction_hash,
        new_order_status == "paid",
        chrono::Utc::now(),
        order_id
    )
    .execute(&mut **tx)
    .await?;

    // 2. 获取订单内部ID
    let order_internal_id = sqlx::query_scalar!(
        "SELECT id FROM orders WHERE order_id = ?",
        order_id
    )
    .fetch_one(&mut **tx)
    .await?;

    // 3. 更新 user_power_records 表
    sqlx::query!(
        r#"
        UPDATE user_power_records
        SET status = ?,
            updated_at = ?
        WHERE order_id = ?
        "#,
        new_power_status,
        chrono::Utc::now(),
        order_internal_id
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}
```

### 事务特性

- **原子性 (Atomicity)**: 订单和算力状态要么同时更新成功，要么同时回滚
- **一致性 (Consistency)**: 状态映射遵循业务规则，数据库始终保持一致状态
- **隔离性 (Isolation)**: 并发事务不会互相干扰
- **持久性 (Durability)**: 事务提交后，状态变更永久保存

## 错误处理

### HTTP 状态码

| 状态码 | 描述 | 场景 |
|--------|------|------|
| 200 | 成功 | 订单状态更新成功 |
| 400 | 请求参数错误 | 缺少必需参数或参数格式错误 |
| 401 | 未授权 | 无效的 JWT token |
| 403 | 权限不足 | 用户无权操作该订单 |
| 404 | 订单不存在 | 指定的订单ID不存在 |
| 409 | 状态冲突 | 订单状态不允许转换到目标状态 |
| 500 | 服务器内部错误 | 数据库操作失败或其他内部错误 |

### 错误响应格式

```json
{
  "code": 400,
  "message": "订单状态不能为空",
  "data": null
}
```

### 常见错误场景

1. **无效订单ID**
   ```json
   {
     "code": 404,
     "message": "订单不存在"
   }
   ```

2. **状态转换不允许**
   ```json
   {
     "code": 409,
     "message": "只能取消待支付的订单"
   }
   ```

3. **权限不足**
   ```json
   {
     "code": 403,
     "message": "无权操作此订单"
   }
   ```

## 代码实现结构

### Repository 层 (src/repository/order_repo.rs)

```rust
impl OrderRepo {
    // 在事务中更新订单状态和对应的算力记录状态
    pub async fn update_order_and_power_status_in_tx(
        tx: &mut sqlx::Transaction<'_, MySql>,
        order_id: &str,
        new_order_status: &str,
        new_power_status: &str,
        transaction_hash: Option<String>,
    ) -> Result<()>

    // 在事务中根据order_id更新订单状态和对应的算力记录状态
    pub async fn update_order_and_power_status(
        pool: &Pool<MySql>,
        order_id: &str,
        new_order_status: &str,
        new_power_status: &str,
        transaction_hash: Option<String>,
    ) -> Result<Option<Order>>

    // 根据order_id完成订单支付并激活算力记录
    pub async fn complete_order_payment(
        pool: &Pool<MySql>,
        order_id: &str,
        transaction_hash: Option<String>,
    ) -> Result<Option<Order>>

    // 根据order_id取消订单并停用算力记录
    pub async fn cancel_order_with_power(
        pool: &Pool<MySql>,
        order_id: &str,
    ) -> Result<Option<Order>>

    // 根据order_id完成订单并设置算力记录为已完成
    pub async fn complete_order(
        pool: &Pool<MySql>,
        order_id: &str,
    ) -> Result<Option<Order>>
}
```

### Service 层 (src/service/order.rs)

```rust
impl OrderService {
    // 更新订单状态（使用事务同时更新算力记录状态）
    pub async fn update_order_status(
        &self,
        user_id: u64,
        order_id: &str,
        new_status: &str,
        transaction_hash: Option<String>,
    ) -> Result<Order>

    // 完成订单支付
    pub async fn complete_order_payment(
        &self,
        user_id: u64,
        order_id: &str,
        transaction_hash: Option<String>,
    ) -> Result<Order>

    // 完成订单
    pub async fn complete_order(
        &self,
        user_id: u64,
        order_id: &str,
    ) -> Result<Order>
}
```

### Schema 层 (src/schema/order.rs)

```rust
// 订单状态更新请求
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateOrderStatusRequest {
    #[serde(rename = "orderId")]
    #[validate(length(min = 1, message = "订单ID不能为空"))]
    pub order_id: String,

    #[serde(rename = "status")]
    #[validate(length(min = 1, message = "订单状态不能为空"))]
    pub status: String,

    #[serde(rename = "transactionHash", skip_serializing_if = "Option::is_none")]
    pub transaction_hash: Option<String>,
}

// 订单状态更新响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateOrderStatusResponse {
    pub order_id: String,
    pub old_status: String,
    pub new_status: String,
    pub updated_at: i64,
    pub transaction_hash: Option<String>,
}
```

### Handler 层 (src/handler/purchase.rs)

```rust
// 更新订单状态
pub async fn update_order_status(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<UpdateOrderStatusRequest>,
) -> Result<impl IntoResponse>

// 完成订单支付
pub async fn complete_order_payment(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(order_id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse>

// 完成订单
pub async fn complete_order(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(order_id): Path<String>,
) -> Result<impl IntoResponse>
```

### 路由配置 (src/app.rs)

```rust
// 购买管理模块
.route("/purchase/orders/status", post(update_order_status)) // 更新订单状态
.route("/purchase/orders/:orderId/complete", post(complete_order_payment)) // 完成订单支付
.route("/purchase/orders/:orderId/finish", post(complete_order)) // 完成订单
```

## 数据验证规则

### 请求验证

- **orderId**: 必填，长度至少1个字符
- **status**: 必填，有效值为 [pending, paid, completed, cancelled, expired]
- **transactionHash**: 可选，仅在支付完成时需要

### 业务规则

1. 只有订单所有者可以修改订单状态
2. 状态转换必须遵循业务规则：
   - `pending` → `paid` (支付完成)
   - `pending` → `cancelled` (取消订单)
   - `paid` → `completed` (订单完成)
   - 其他转换为非法操作
3. 支付完成时必须提供交易哈希
4. 订单状态和算力记录状态必须在同一事务中更新

## 测试用例

详细的测试用例和运行指南请参考：
- `/tests/order_status_integration_tests.rs` - 集成测试用例
- `/tests/README.md` - 测试运行指南

## 性能考虑

1. **数据库索引**: 确保 `orders.order_id` 和 `user_power_records.order_id` 有唯一索引
2. **事务隔离**: 使用适当的隔离级别平衡并发性和一致性
3. **连接池**: 配置合适的数据库连接池大小
4. **错误重试**: 对于并发冲突实现适当的重试机制
5. **日志监控**: 记录关键操作和性能指标

## 安全注意事项

1. **权限验证**: 严格验证用户对订单的操作权限
2. **参数验证**: 对所有输入参数进行严格验证
3. **SQL注入防护**: 使用参数化查询，避免SQL注入
4. **敏感信息**: 不在日志中记录敏感的交易哈希
5. **访问控制**: 实现适当的访问频率限制

## 版本信息

- **API版本**: v1.0
- **创建时间**: 2025-11-29
- **更新时间**: 2025-11-29
- **兼容性**: 遵循项目 CODING_CONVENTIONS.md 规范
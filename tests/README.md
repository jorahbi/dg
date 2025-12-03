# 订单状态修改接口集成测试

## 概述

此测试套件验证新实现的订单状态修改接口，包括事务处理、数据一致性和错误处理。

## 功能特性

### 1. 核心功能

- **订单状态更新**: 使用 `POST /api/purchase/orders/status` 接口更新订单状态
- **订单支付完成**: 使用 `POST /api/purchase/orders/:orderId/complete` 接口完成订单支付
- **订单完成**: 使用 `POST /api/purchase/orders/:orderId/finish` 接口完成订单

### 2. 事务特性

- **原子性**: 订单状态和算力记录状态在同一事务中更新
- **一致性**: 状态映射遵循业务规则：
  - `pending` → `paid` (激活算力)
  - `paid` → `completed` (算力到期)
  - `pending` → `cancelled` (取消算力)
- **隔离性**: 并发操作不会互相干扰
- **持久性**: 事务提交后状态变更永久保存

### 3. API接口

#### 更新订单状态
```http
POST /api/purchase/orders/status
Content-Type: application/json
Authorization: Bearer <jwt_token>

{
  "orderId": "ORD123456789",
  "status": "paid",
  "transactionHash": "0x1234567890abcdef1234567890abcdef12345678"
}
```

#### 完成订单支付
```http
POST /api/purchase/orders/ORD123456789/complete
Content-Type: application/json
Authorization: Bearer <jwt_token>

{
  "transactionHash": "0x1234567890abcdef1234567890abcdef12345678"
}
```

#### 完成订单
```http
POST /api/purchase/orders/ORD123456789/finish
Authorization: Bearer <jwt_token>
```

### 4. 响应格式

成功响应:
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

### 5. 错误处理

| 错误代码 | 描述 | 原因 |
|----------|------|------|
| 400 | 请求参数错误 | 缺失必需参数或参数格式错误 |
| 401 | 未授权 | 无效的JWT token |
| 403 | 权限不足 | 用户无权操作该订单 |
| 404 | 订单不存在 | 指定的订单ID不存在 |
| 500 | 服务器内部错误 | 数据库操作失败 |

### 6. 数据库表结构

#### orders 表
```sql
CREATE TABLE `orders` (
    `id` BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    `order_id` VARCHAR(64) NOT NULL UNIQUE,
    `user_id` BIGINT UNSIGNED NOT NULL,
    `power_package_id` BIGINT UNSIGNED NOT NULL,
    `quantity` INT UNSIGNED NOT NULL DEFAULT 1,
    `amount` DECIMAL(20, 8) NOT NULL,
    `currency` VARCHAR(20) NOT NULL DEFAULT 'USDT',
    `status` ENUM('pending', 'paid', 'completed', 'cancelled', 'expired') NOT NULL DEFAULT 'pending',
    `transaction_hash` VARCHAR(128) NULL,
    `paid_at` TIMESTAMP NULL DEFAULT NULL,
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
```

#### user_power_records 表
```sql
CREATE TABLE `user_power_records` (
    `id` BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    `user_id` BIGINT UNSIGNED NOT NULL,
    `power_package_id` BIGINT UNSIGNED NOT NULL,
    `order_id` BIGINT UNSIGNED NULL,
    `status` ENUM('active', 'expired', 'cancelled') NOT NULL DEFAULT 'active',
    `start_time` TIMESTAMP NOT NULL,
    `end_time` TIMESTAMP NOT NULL,
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
```

## 运行测试

### 前置条件

1. **设置环境变量**:
   ```bash
   export TEST_DATABASE_URL="mysql://user:password@localhost/coin_dgai_test"
   export JWT_SECRET="your_test_secret"
   ```

2. **准备测试数据库**:
   ```bash
   # 创建测试数据库
   mysql -u root -p -e "CREATE DATABASE IF NOT EXISTS coin_dgai_test;"

   # 运行迁移
   sqlx migrate run --database-url $TEST_DATABASE_URL
   ```

3. **安装测试依赖**:
   ```bash
   cargo install cargo-nextest
   ```

### 运行测试

#### 运行所有集成测试
```bash
# 使用 nextest (推荐)
cargo nextest run --test integration

# 或使用标准 cargo
cargo test --test integration -- --nocapture
```

#### 运行特定测试
```bash
# API结构测试
cargo test test_order_status_update_api_structure -- --nocapture

# 请求载荷测试
cargo test test_order_status_request_payload -- --nocapture

# 响应结构测试
cargo test test_order_status_response_structure -- --nocapture

# 状态映射测试
cargo test test_order_status_mapping -- --nocapture

# 事务需求测试
cargo test test_transaction_requirements -- --nocapture

# 错误场景测试
cargo test test_error_scenarios -- --nocapture

# 性能测试
cargo test test_order_status_update_performance -- --nocapture
```

### 测试覆盖率

运行测试并生成覆盖率报告:
```bash
# 安装 cargo-llvm-cov
cargo install cargo-llvm-cov

# 运行测试并生成覆盖率
cargo llvm-cov --lcov --test integration

# 生成 HTML 报告
cargo llvm-cov --html --test integration
open target/llvm-cov/html/index.html
```

## 测试场景

### 1. 正常流程测试

- ✅ 订单状态从 `pending` 更新为 `paid`
- ✅ 订单状态从 `paid` 更新为 `completed`
- ✅ 订单状态从 `pending` 更新为 `cancelled`

### 2. 异常情况测试

- ✅ 无效的订单ID
- ✅ 不允许的状态转换
- ✅ 未授权的访问
- ✅ 缺失必需参数
- ✅ 数据库连接失败

### 3. 并发测试

- ✅ 多个并发状态更新请求
- ✅ 死锁检测和预防
- ✅ 数据一致性验证

### 4. 性能测试

- ✅ 单个状态更新响应时间
- ✅ 批量状态更新吞吐量
- ✅ 并发处理能力

## 实现细节

### Repository层

- `OrderRepo::update_order_and_power_status_in_tx()`: 在事务中同时更新两个表
- `OrderRepo::update_order_and_power_status()`: 事务包装器
- `OrderRepo::complete_order_payment()`: 支付完成接口
- `OrderRepo::cancel_order_with_power()`: 取消订单接口
- `OrderRepo::complete_order()`: 完成订单接口

### Service层

- `OrderService::update_order_status()`: 业务逻辑和验证
- 状态映射逻辑和错误处理
- 权限验证和业务规则检查

### Handler层

- `update_order_status()`: 通用状态更新处理器
- `complete_order_payment()`: 支付完成处理器
- `complete_order()`: 订单完成处理器
- 统一错误处理和响应格式

### Schema层

- `UpdateOrderStatusRequest`: 请求验证和反序列化
- `UpdateOrderStatusResponse`: 响应序列化
- `OrderStatusInfo`: 订单状态信息传输对象

## 注意事项

1. **事务安全**: 所有状态更新都在数据库事务中执行
2. **状态一致性**: 订单状态和算力状态同步更新
3. **错误回滚**: 任何步骤失败都会回滚整个事务
4. **并发控制**: 使用适当的数据库隔离级别
5. **性能考虑**: 索引优化和批量操作支持

## 维护和扩展

- 新增状态转换需要在 `OrderService::update_order_status()` 中添加映射
- 数据库变更需要同步更新测试用例
- API接口变更需要更新文档和测试
- 性能优化需要定期基准测试
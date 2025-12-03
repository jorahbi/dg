# WebSocket 聊天记录批量处理机制

## 概述

本项目实现了高效的WebSocket聊天记录批量插入机制，通过以下两种触发条件来优化数据库性能：
- **数量触发**: 当待处理消息达到50条时自动插入
- **时间触发**: 每1分钟自动插入一次待处理消息

## 架构设计

### 1. 批量处理器 (MessageBatchProcessor)

```rust
use crate::websocket::{MessageBatchProcessor, BatchConfig};

// 默认配置：50条消息或60秒间隔
let config = BatchConfig::default();
let processor = MessageBatchProcessor::new(config, db_pool);
```

### 2. 配置选项

#### 默认配置
- 最大批量大小: 50条消息
- 刷新间隔: 60秒

#### 预设配置
```rust
// 高性能配置：20条消息或30秒间隔
let config = WebSocketConfig::high_performance();

// 低延迟配置：10条消息或15秒间隔
let config = WebSocketConfig::low_latency();

// 高吞吐量配置：100条消息或120秒间隔
let config = WebSocketConfig::high_throughput();

// 自定义配置
let config = WebSocketConfig::custom(30, 45); // 30条消息或45秒间隔

// 从环境变量配置
let config = WebSocketConfig::from_env();
// 环境变量：WS_BATCH_SIZE=50, WS_FLUSH_INTERVAL=60
```

### 3. 使用方法

#### 集成到WebSocket Handler
```rust
use crate::websocket::{websocket_handler, WebSocketConfig};

// 在应用启动时创建WebSocket处理器
let ws_config = WebSocketConfig::low_latency(); // 低延迟配置
app.route("/ws/chat", get(websocket_handler));
```

#### 手动添加消息到批处理队列
```rust
let message = PendingMessage {
    id: Uuid::new_v4().to_string(),
    conversation_id: 123,
    sender_id: 456,
    sender_type: "user".to_string(),
    content: "Hello, world!".to_string(),
    message_type: "text".to_string(),
    file_url: None,
    file_name: None,
    created_at: chrono::Utc::now(),
};

// 添加到批处理队列
processor.add_message(message).await;
```

#### 强制刷新消息
```rust
// 刷新所有待处理消息
processor.force_flush_all().await?;

// 刷新特定会话的消息
processor.flush_conversation_messages(conversation_id).await?;

// 获取待处理消息数量
let pending_count = processor.get_pending_count().await;
println!("待处理消息数量: {}", pending_count);
```

## 性能优势

### 1. 数据库性能提升
- **减少I/O操作**: 批量插入相比逐条插入可减少90%以上的数据库I/O操作
- **事务优化**: 使用事务确保批量操作的原子性和一致性
- **连接复用**: 减少数据库连接建立和释放的开销

### 2. 系统响应性
- **即时响应**: WebSocket消息立即广播给客户端，无需等待数据库插入完成
- **异步处理**: 数据库插入在后台异步进行，不阻塞用户交互

### 3. 资源优化
- **内存控制**: 待处理消息缓存在内存中，避免频繁的内存分配
- **CPU节省**: 减少数据库操作的CPU开销

## 监控和调试

### 1. 日志输出
批量处理器会输出详细的日志信息：
```
INFO  Flushed 50 messages to database
INFO  Flushed 15 timeout messages to database
INFO  Flushed 8 messages for conversation 123 to database
ERROR Failed to flush messages: Database connection error
```

### 2. 性能监控
```rust
// 监控待处理消息数量
let pending_count = processor.get_pending_count().await;
if pending_count > 1000 {
    warn!("待处理消息积压过多: {}", pending_count);
}
```

## 故障处理

### 1. 消息丢失防护
- **断开连接时强制刷新**: 客户端断开时自动刷新所有待处理消息
- **会话结束时强制刷新**: 离开会话时刷新该会话的相关消息
- **定时器保障**: 即使消息数量不足，定时器也会定期刷新

### 2. 错误恢复
- **重试机制**: 数据库插入失败时会记录错误日志
- **优雅降级**: 批量插入失败不影响WebSocket消息的实时广播

## 环境变量配置

可以通过以下环境变量调整批量处理行为：

```bash
# 批量大小（默认: 50）
export WS_BATCH_SIZE=30

# 刷新间隔秒数（默认: 60）
export WS_FLUSH_INTERVAL=45

# 应用启动时会自动读取这些环境变量
```

## 注意事项

1. **内存使用**: 待处理消息会暂存在内存中，需要监控内存使用情况
2. **数据一致性**: 消息广播和数据库插入是异步的，需要处理可能的时序问题
3. **故障恢复**: 服务器重启时内存中的待处理消息会丢失，需要根据业务需求决定是否需要持久化队列

## 最佳实践

1. **根据负载调整配置**: 高并发时使用更大的批量大小，低延迟场景时使用更短的刷新间隔
2. **监控性能指标**: 定期检查批量处理的性能和待处理消息数量
3. **设置告警**: 当待处理消息数量异常高时设置告警
4. **定期清理**: 定期检查和清理可能的内存泄漏
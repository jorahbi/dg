# 多语言数据转换模式文档

## 概述

本文档记录了在 Rust + Axum + SQLx 项目中实现数据库多语言 JSON 字段到前端响应字符串的转换模式。该模式适用于需要国际化支持的应用程序。

## 场景描述

### 数据库层面
- 数据库中的多语言字段以 JSON 格式存储
- 支持多种语言的内容，如：`{"zh": "初级矿工", "en": "Beginner Miner"}`
- 字段类型：`JSON` 或 `TEXT` 存储 JSON 字符串

### 响应层面
- 前端需要根据用户语言偏好获取对应的字符串
- 如果指定语言不存在，需要回退到默认语言
- 最终输出为普通的 `String` 类型

## 核心转换模式

### 1. 数据结构定义

#### 数据库模型（使用 SQLx）
```rust
// src/model/power.rs
use sqlx::types::{Json, JsonValue};
use serde_json::Value;

#[derive(Debug, Clone, FromRow)]
pub struct UserPowerRecord {
    pub id: u64,
    pub title: Option<JsonValue>,        // JSON 字段
    pub description: Option<JsonValue>,  // JSON 字段
    // ... 其他字段
}
```

#### 响应模型
```rust
// src/schema/power.rs
#[derive(Debug, Serialize, Deserialize)]
pub struct PowerRecord {
    pub id: u64,
    pub title: String,        // 转换后的字符串
    pub description: String,  // 转换后的字符串
    // ... 其他字段
}
```

### 2. 核心转换函数

#### 主要转换函数
```rust
/// 从 UserPowerRecord 转换为 PowerRecord，处理多语言 JSON 字段
pub fn convert_user_power_record_to_power_record(
    record: UserPowerRecord,
    lang: &str,
) -> PowerRecord {
    let title_str = match record.title {
        Some(json_title) => extract_localized_string(&json_title, lang),
        None => "Unknown Package".to_string(),  // 默认值
    };

    let description_str = match record.description {
        Some(json_desc) => extract_localized_string(&json_desc, lang),
        None => "No description available".to_string(),  // 默认值
    };

    PowerRecord {
        id: record.id,
        title: title_str,
        description: description_str,
        // ... 其他字段转换
    }
}
```

#### 多语言字符串提取函数
```rust
/// 从 JSON 值中提取指定语言的文本
fn extract_localized_string(json_value: &JsonValue, lang: &str) -> String {
    match json_value {
        // 如果是纯字符串，直接返回
        Value::String(s) => s.clone(),

        // 如果是对象（多语言映射），按优先级提取
        Value::Object(map) => {
            // 优先级：指定语言 > 英文 > 第一个值
            if let Some(value) = map.get(lang) {
                extract_string_value(value)
            } else if let Some(value) = map.get("en") {
                extract_string_value(value)
            } else if let Some(value) = map.values().next() {
                extract_string_value(value)
            } else {
                String::new()  // 空字符串作为最后回退
            }
        }

        // 其他类型转换为字符串
        _ => String::new(),
    }
}

/// 从 JSON 值中提取字符串
fn extract_string_value(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        _ => value.to_string(),  // 其他类型转为字符串
    }
}
```

#### 批量转换函数
```rust
/// 将 Vec<UserPowerRecord> 转换为 Vec<PowerRecord>
pub fn convert_user_power_records(
    records: Vec<UserPowerRecord>,
    lang: &str,
) -> Vec<PowerRecord> {
    records
        .into_iter()
        .map(|record| convert_user_power_record_to_power_record(record, lang))
        .collect()
}
```

### 3. Handler 层应用

#### 在 API Handler 中使用
```rust
// src/handler/power.rs
pub async fn get_power_records(
    State(state): State<AppState>,
    auth_user: AuthUser,  // 包含语言信息
    Query(pagination): Query<PaginationRequest>,
) -> Result<impl IntoResponse> {
    // 1. 从数据库获取原始数据
    let (records, total) = PowerRepo::get_user_power_records(
        &state.db,
        auth_user.id,
        page,
        limit,
    ).await?;

    // 2. 应用多语言转换
    let power_records = convert_user_power_records(records, &auth_user.lang);

    // 3. 构建响应
    let response = PowerRecordsResponse {
        records: power_records,
        pagination: pagination_info,
    };

    Ok(Json(ApiResponse::success(response)))
}
```

### 4. 用户语言信息获取

#### AuthUser 结构
```rust
// src/extract/auth.rs
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: u64,
    pub username: String,
    pub user_level: i32,
    pub lang: String,  // 用户语言偏好
}

impl AuthUser {
    pub fn from_claims(claims: Claims) -> Self {
        Self {
            id: claims.sub,
            username: claims.username,
            user_level: claims.user_level,
            lang: "en".to_string(),  // 默认语言
        }
    }

    pub fn from_claims_with_lang(claims: Claims, lang: String) -> Self {
        Self {
            id: claims.sub,
            username: claims.username,
            user_level: claims.user_level,
            lang,
        }
    }
}
```

## 转换模式特点

### 1. 类型安全
- 使用 SQLx 的 `Json<JsonValue>` 类型映射数据库 JSON 字段
- 编译时类型检查，避免运行时错误

### 2. 灵活的语言回退机制
```rust
// 回退策略：
// 1. 用户指定语言（如 "zh"）
// 2. 英文（"en"）
// 3. 第一个可用值
// 4. 空字符串或默认值
```

### 3. 错误处理
- 对可能为空的字段提供合理的默认值
- 类型转换安全（如 `BigDecimal` 到 `f64`）

### 4. 可扩展性
- 轻松添加新的支持语言
- 支持不同格式的 JSON 数据结构

## 数据库设计建议

### JSON 字段结构
```json
// 推荐的多语言 JSON 结构
{
    "zh": "初级矿工",
    "en": "Beginner Miner",
    "ja": "初心者マイナー"
}

// 简单字符串格式（向后兼容）
"简单标题"
```

### 数据库列定义
```sql
-- MySQL 示例
CREATE TABLE power_packages (
    id BIGINT PRIMARY KEY,
    title JSON NOT NULL,
    description JSON NOT NULL,
    -- 其他字段
);
```

## 应用场景

### 1. 国际化支持
- 产品名称、描述的多语言显示
- 错误消息的本地化
- 用户界面文本的国际化

### 2. 动态内容
- 配置项的多语言存储
- 模板内容的国际化
- 邮件/SMS 模板的本地化

### 3. 元数据管理
- 分类标签的多语言支持
- 资源描述的国际化
- 系统提示信息的本地化

## 性能考虑

### 1. JSON 解析开销
- JSON 解析有一定性能开销，建议在数据访问层完成转换
- 避免在热路径中重复解析相同的 JSON 数据

### 2. 缓存策略
```rust
// 可考虑添加缓存层
use std::collections::HashMap;
static mut I18N_CACHE: Option<HashMap<String, String>> = None;

fn extract_localized_string_cached(json_value: &JsonValue, lang: &str) -> String {
    // 缓存实现示例
    let cache_key = format!("{}:{}", json_value.to_string(), lang);

    // 检查缓存
    if let Some(cached) = unsafe {
        I18N_CACHE.as_ref().and_then(|c| c.get(&cache_key).cloned())
    } {
        return cached;
    }

    // 解析并缓存
    let result = extract_localized_string(json_value, lang);
    // 缓存结果...
    result
}
```

### 3. 数据库优化
- 为经常查询的 JSON 字段创建适当的索引
- 考虑使用虚拟列提取常用语言的值

## 测试策略

### 1. 单元测试
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_extract_localized_string() {
        // 测试多语言对象
        let json_obj = json!({
            "zh": "初级矿工",
            "en": "Beginner Miner"
        });

        assert_eq!(
            extract_localized_string(&json_obj, "zh"),
            "初级矿工"
        );
        assert_eq!(
            extract_localized_string(&json_obj, "en"),
            "Beginner Miner"
        );

        // 测试回退到英文
        assert_eq!(
            extract_localized_string(&json_obj, "ja"),
            "Beginner Miner"  // 回退到英文
        );

        // 测试纯字符串
        let json_str = json!("Simple Title");
        assert_eq!(
            extract_localized_string(&json_str, "zh"),
            "Simple Title"
        );
    }
}
```

### 2. 集成测试
- 测试完整的数据流：数据库 → 转换函数 → API 响应
- 验证不同语言设置下的响应格式
- 测试边界情况和错误处理

## 最佳实践

### 1. 命名规范
```rust
// 良好的函数命名
convert_[source_model]_to_[target_model]     // 单个转换
convert_[source_model]_s_to_[target_model]_s // 批量转换
extract_[field_name]_localized               // 字段提取
```

### 2. 代码组织
```
src/
├── model/           # 数据库模型和转换函数
│   └── power.rs     # 与 power 相关的转换逻辑
├── schema/          # API 响应模型
│   └── power.rs
├── handler/         # API 处理器
│   └── power.rs
└── extract/         # 请求提取器
    └── auth.rs       # 用户信息（包括语言）
```

### 3. 文档和注释
- 为转换函数添加详细的文档注释
- 说明语言回退策略
- 标注函数的复杂度和性能特征

### 4. 错误处理
- 对 JSON 解析错误进行适当处理
- 为数据缺失提供合理的默认值
- 记录转换过程中的异常情况

## 扩展模式

### 1. 支持更多数据类型
```rust
// 支持日期时间的本地化
fn extract_localized_datetime(json_value: &JsonValue, lang: &str, timezone: &str) -> String {
    // 根据语言和时区格式化日期时间
}

// 支持货币的本地化
fn extract_localized_currency(json_value: &JsonValue, lang: &str) -> String {
    // 根据语言格式化货币显示
}
```

### 2. 复杂嵌套结构
```rust
// 支持嵌套对象的转换
fn extract_nested_localized_string(
    json_value: &JsonValue,
    path: &[&str],
    lang: &str
) -> String {
    // 根据路径提取嵌套字段并本地化
}
```

### 3. 动态语言检测
```rust
// 根据请求头自动检测语言
fn detect_accept_language(headers: &HeaderMap) -> String {
    // 解析 Accept-Language 头部
    // 返回最适合的语言代码
}
```

## 总结

该多语言数据转换模式提供了一个类型安全、可扩展、性能良好的解决方案，用于处理数据库 JSON 字段的多语言内容到前端字符串的转换。通过合理的分层设计和错误处理，可以很好地支持国际化应用的需求。

在实现类似功能时，可以参考这个模式的结构和最佳实践，根据具体项目需求进行调整和扩展。
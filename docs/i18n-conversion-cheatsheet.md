# 多语言数据转换快速参考卡

## 🎯 核心模式

### 数据库 → API 转换
```
JSON字段（数据库） → 多语言提取函数 → String字段（API响应）
```

## 📋 实现步骤

### 1️⃣ 定义数据结构
```rust
// 数据库模型
#[derive(Debug, Clone, FromRow)]
pub struct DatabaseModel {
    #[sqlx(json)]
    pub name: Option<JsonValue>,  // JSON 多语言字段
}

// API 响应模型
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseModel {
    pub name: String,  // 转换后的字符串
}
```

### 2️⃣ 核心转换函数
```rust
fn extract_localized_string(json_value: &JsonValue, lang: &str) -> String {
    match json_value {
        Value::String(s) => s.clone(),
        Value::Object(map) => {
            map.get(lang)       // 1. 指定语言
                .or_else(|| map.get("en"))  // 2. 英文回退
                .and_then(|v| v.as_str())
                .unwrap_or("")     // 3. 空字符串回退
                .to_string()
        }
        _ => String::new(),
    }
}
```

### 3️⃣ 批量转换
```rust
pub fn convert_records(
    records: Vec<DatabaseModel>,
    lang: &str,
) -> Vec<ResponseModel> {
    records
        .into_iter()
        .map(|r| convert_single(r, lang))
        .collect()
}
```

### 4️⃣ API Handler 使用
```rust
pub async fn get_items(
    auth_user: AuthUser,  // 包含 .lang
) -> Result<impl IntoResponse> {
    let (db_records, _) = repo::get_items(...).await?;

    // 🔑 关键步骤：多语言转换
    let items = convert_records(db_records, &auth_user.lang);

    Ok(Json(ApiResponse::success(items)))
}
```

## 🌐 语言回退策略

```
用户指定语言 (zh/ja/fr/...)
    ↓
英文 (en)
    ↓
第一个可用值
    ↓
空字符串/默认值
```

## 📊 JSON 数据格式

### 多语言对象（推荐）
```json
{
    "zh": "初级矿工",
    "en": "Beginner Miner",
    "ja": "初心者マイナー"
}
```

### 简单字符串（向后兼容）
```json
"Simple Title"
```

## 🗄️ 数据库迁移

### MySQL
```sql
-- 添加 JSON 列
ALTER TABLE items ADD COLUMN name JSON;

-- 转换现有数据
UPDATE items SET
    name = JSON_OBJECT('zh', name, 'en', name)
WHERE JSON_TYPE(name) != 'OBJECT';
```

### PostgreSQL
```sql
ALTER TABLE items ADD COLUMN name JSONB;
UPDATE items SET
    name = jsonb_build_object('zh', name, 'en', name)
WHERE jsonb_typeof(name) != 'object';
```

## 🧪 测试用例

```rust
#[test]
fn test_i18n_extraction() {
    let json_obj = json!({
        "zh": "初级矿工",
        "en": "Beginner Miner"
    });

    assert_eq!(extract_localized_string(&json_obj, "zh"), "初级矿工");
    assert_eq!(extract_localized_string(&json_obj, "en"), "Beginner Miner");
    assert_eq!(extract_localized_string(&json_obj, "fr"), "Beginner Miner"); // 回退
}
```

## ⚡ 性能优化

- 在数据访问层完成转换
- 避免在热路径重复解析
- 考虑添加缓存层
- 为常用语言创建虚拟列索引

## 🎨 代码模板文件

- `multilingual-data-conversion-pattern.md` - 完整文档
- `i18n-conversion-template.rs` - 代码模板

## 📁 文件结构建议

```
src/
├── model/          # 数据库模型 + 转换函数
├── schema/         # API 响应模型
├── handler/        # API 处理器
└── extract/        # 用户信息（包含语言）
```

## 🔧 常用函数命名

```rust
convert_[source]_to_[target]     // 单个转换
convert_[source]_s_to_[target]_s // 批量转换
extract_[field]_localized         // 字段提取
```

## 🚀 快速开始

1. 复制 `i18n-conversion-template.rs` 中的代码
2. 根据你的数据结构调整类型定义
3. 在 API Handler 中应用转换函数
4. 添加测试用例验证转换逻辑

---

**💡 提示**: 这个模式可以扩展到任何需要多语言支持的 JSON 字段！
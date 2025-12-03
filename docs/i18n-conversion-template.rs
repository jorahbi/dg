// ===== 多语言数据转换模板 =====
// 用于数据库 JSON 字段到前端字符串的国际化转换

// 1. 导入依赖
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sqlx::types::{Json, JsonValue};
use sqlx::FromRow;
use std::collections::HashMap;

// ===== 数据库模型示例 =====
#[derive(Debug, Clone, FromRow)]
pub struct DatabaseModel {
    pub id: u64,
    #[sqlx(json)]
    pub name: Option<JsonValue>,        // JSON 多语言字段
    #[sqlx(json)]
    pub description: Option<JsonValue>, // JSON 多语言字段
    // ... 其他字段
}

// ===== API 响应模型示例 =====
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseModel {
    pub id: u64,
    pub name: String,        // 转换后的字符串
    pub description: String, // 转换后的字符串
    // ... 其他字段
}

// ===== 核心转换函数 =====

/// 从 JSON 值中提取指定语言的文本
///
/// # 语言回退策略
/// 1. 优先使用指定语言（如 "zh", "en", "ja"）
/// 2. 回退到英文（"en"）
/// 3. 回退到第一个可用值
/// 4. 返回空字符串
pub fn extract_localized_string(json_value: &JsonValue, lang: &str) -> String {
    match json_value {
        // 纯字符串直接返回
        Value::String(s) => s.clone(),

        // 多语言对象按优先级提取
        Value::Object(map) => {
            if let Some(value) = map.get(lang) {
                extract_string_value(value)
            } else if let Some(value) = map.get("en") {
                extract_string_value(value)
            } else if let Some(value) = map.values().next() {
                extract_string_value(value)
            } else {
                String::new()
            }
        }

        // 其他类型转换为字符串
        _ => String::new(),
    }
}

/// 从 JSON 值中提取字符串
pub fn extract_string_value(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        _ => value.to_string(),
    }
}

/// 单个记录转换函数
pub fn convert_database_to_response(
    record: DatabaseModel,
    lang: &str,
) -> ResponseModel {
    ResponseModel {
        id: record.id,
        name: extract_localized_string(
            &record.name.unwrap_or_else(|| Value::String("Unknown".to_string())),
            lang
        ),
        description: extract_localized_string(
            &record.description.unwrap_or_else(|| Value::String("No description".to_string())),
            lang
        ),
        // ... 其他字段转换
    }
}

/// 批量记录转换函数
pub fn convert_database_records(
    records: Vec<DatabaseModel>,
    lang: &str,
) -> Vec<ResponseModel> {
    records
        .into_iter()
        .map(|record| convert_database_to_response(record, lang))
        .collect()
}

// ===== 使用示例 =====

/*
#[cfg(test)]
mod usage_example {
    use super::*;
    use serde_json::json;

    fn example_usage() {
        // 模拟数据库返回的数据
        let db_record = DatabaseModel {
            id: 1,
            name: Some(json!({
                "zh": "初级矿工",
                "en": "Beginner Miner",
                "ja": "初心者マイナー"
            })),
            description: Some(json!({
                "zh": "适合新手的入门级算力包",
                "en": "Entry-level mining package for beginners",
                "ja": "初心者向けのエントリーレベルマイニングパッケージ"
            })),
        };

        // 转换为中文
        let zh_response = convert_database_to_response(&db_record, "zh");
        assert_eq!(zh_response.name, "初级矿工");
        assert_eq!(zh_response.description, "适合新手的入门级算力包");

        // 转换为英文
        let en_response = convert_database_to_response(&db_record, "en");
        assert_eq!(en_response.name, "Beginner Miner");
        assert_eq!(en_response.description, "Entry-level mining package for beginners");

        // 转换为不存在的语言（会回退到英文）
        let fr_response = convert_database_to_response(&db_record, "fr");
        assert_eq!(fr_response.name, "Beginner Miner"); // 回退到英文
        assert_eq!(fr_response.description, "Entry-level mining package for beginners");

        // 批量转换
        let records = vec![db_record];
        let responses = convert_database_records(records, "zh");
        assert_eq!(responses.len(), 1);
    }
}
*/

// ===== API Handler 使用模板 =====

/*
use crate::{
    extract::AuthUser,
    schema::common::{ApiResponse, PaginationRequest},
    model::{convert_database_records, DatabaseModel},
    schema::{ResponseModel, ResponsePagination},
    state::AppState,
    error::Result,
    repository::your_repo::YourRepo,
};

pub async fn get_items(
    State(state): State<AppState>,
    auth_user: AuthUser,  // 包含语言信息
    Query(pagination): Query<PaginationRequest>,
) -> Result<impl IntoResponse> {
    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(20);

    // 1. 从数据库获取原始数据
    let (records, total) = YourRepo::get_user_items(
        &state.db,
        auth_user.id,
        page,
        limit,
    ).await?;

    // 2. 应用多语言转换（关键步骤）
    let items = convert_database_records(records, &auth_user.lang);

    // 3. 构建分页信息
    let total_pages = ((total as f64) / (limit as f64)).ceil() as u32;
    let pagination_info = ResponsePagination {
        page,
        limit,
        total,
        total_pages,
    };

    // 4. 构建响应
    let response = ApiResponse::success(ResponseModel {
        items,
        pagination: pagination_info,
    });

    Ok(Json(response))
}
*/

// ===== 数据库迁移示例 =====

/*
-- MySQL
ALTER TABLE your_table
ADD COLUMN name JSON COMMENT '多语言名称',
ADD COLUMN description JSON COMMENT '多语言描述';

-- 更新现有数据为多语言格式
UPDATE your_table SET
    name = JSON_OBJECT('zh', name, 'en', name),
    description = JSON_OBJECT('zh', description, 'en', description)
WHERE JSON_TYPE(name) != 'OBJECT';

-- 为 JSON 字段创建索引（如果需要）
CREATE INDEX idx_your_table_name_zh ON your_table ((JSON_UNQUOTE(JSON_EXTRACT(name, '$.zh'))));
CREATE INDEX idx_your_table_name_en ON your_table ((JSON_UNQUOTE(JSON_EXTRACT(name, '$.en'))));
*/
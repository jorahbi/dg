use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
// 通用API响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: T,
}

#[derive(Default, Serialize, Deserialize)]
pub struct EmptyObject;

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            message: "success".to_string(),
            data,
        }
    }

    pub fn success_with_message(data: T, message: &str) -> Self {
        Self {
            code: 200,
            message: message.to_string(),
            data,
        }
    }

    pub fn error(code: i32, message: &str) -> ApiResponse<String> {
        ApiResponse {
            code,
            message: message.to_string(),
            data: String::new(),
        }
    }
}

impl ApiResponse<EmptyObject> {
    pub fn empty_object() -> Self {
        Self::success(EmptyObject {})
    }
}

impl ApiResponse<Vec<EmptyObject>> {
    pub fn empty_list() -> Self {
        Self::success(vec![])
    }
}


// 分页请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationRequest {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

impl Default for PaginationRequest {
    fn default() -> Self {
        Self {
            page: Some(1),
            limit: Some(20),
        }
    }
}

// 分页响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationData<T> {
    pub page: u32,
    pub limit: u32,
    pub total: u64,
    pub total_pages: u32,
    pub data: Vec<T>,
}

impl<T> PaginationData<T> {
    pub fn new(page: u32, limit: u32, total: u64, data: Vec<T>) -> Self {
        let total_pages = if limit == 0 {
            0
        } else {
            ((total as f64) / (limit as f64)).ceil() as u32
        };

        Self {
            page,
            limit,
            total,
            total_pages,
            data,
        }
    }
}

// 错误响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub code: String,
    pub message: String,
    pub status_code: i32,
    pub timestamp: DateTime<Utc>,
}

impl ErrorResponse {
    pub fn new(code: &str, message: &str, status_code: i32) -> Self {
        Self {
            success: false,
            code: code.to_string(),
            message: message.to_string(),
            status_code,
            timestamp: Utc::now(),
        }
    }
}

// 文件上传响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploadResponse {
    pub url: String,
    pub file_size: u64,
    pub upload_time: DateTime<Utc>,
    pub status: String,
    pub validity_check: Option<FileValidityCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileValidityCheck {
    pub is_valid: bool,
    pub confidence: f64,
    pub issues: Vec<String>,
}

// 时间范围查询参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRangeQuery {
    pub start_date: Option<String>, // YYYY-MM-DD
    pub end_date: Option<String>,   // YYYY-MM-DD
}

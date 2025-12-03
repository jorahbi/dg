use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Authentication error: {0}")]
    Auth(String),
    
    #[error("Authorization error: {0}")]
    Authorization(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Business logic error: {0}")]
    Business(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Resource already exists: {0}")]
    Conflict(String),

    #[error("Too many requests")]
    RateLimit,

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Order not paid: {0}")]
    OrderNotPaid(String),

    #[error("JWT error: {0}")]
    JWT(#[from] jsonwebtoken::errors::Error),

    #[error("Password hash error: {0}")]
    PasswordHash(#[from] bcrypt::BcryptError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Environment variable error: {0}")]
    EnvVar(#[from] std::env::VarError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("WebSocket error: {0}")]
    WebSocket(String),
}


impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message, error_code) = match &self {
            AppError::Database(err) => {
                tracing::error!("Database error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string(), "DATABASE_ERROR")
            }
            AppError::Auth(msg) => {
                (StatusCode::UNAUTHORIZED, msg.clone(), "AUTH_ERROR")
            }
            AppError::OrderNotPaid(msg) => {
                (StatusCode::PAYMENT_REQUIRED, msg.clone(), "ORDER_NOT_PAID")
            }
            AppError::Authorization(msg) => {
                (StatusCode::FORBIDDEN, msg.clone(), "AUTHORIZATION_ERROR")
            }
            AppError::Validation(msg) => {
                (StatusCode::BAD_REQUEST, msg.clone(), "VALIDATION_ERROR")
            }
            AppError::Business(msg) => {
                (StatusCode::BAD_REQUEST, msg.clone(), "BUSINESS_ERROR")
            }
            AppError::NotFound(msg) => {
                (StatusCode::NOT_FOUND, msg.clone(), "NOT_FOUND")
            }
            AppError::Conflict(msg) => {
                (StatusCode::CONFLICT, msg.clone(), "CONFLICT")
            }
            AppError::RateLimit => {
                (StatusCode::TOO_MANY_REQUESTS, "Too many requests".to_string(), "RATE_LIMIT")
            }
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string(), "INTERNAL_ERROR")
            }
            AppError::JWT(err) => {
                tracing::warn!("JWT error: {}", err);
                (StatusCode::UNAUTHORIZED, "Authentication token invalid".to_string(), "JWT_ERROR")
            }
            AppError::PasswordHash(err) => {
                tracing::error!("Password hash error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Password processing error".to_string(), "PASSWORD_HASH_ERROR")
            }
            AppError::Serialization(err) => {
                tracing::error!("Serialization error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Data serialization error".to_string(), "SERIALIZATION_ERROR")
            }
            AppError::EnvVar(err) => {
                tracing::error!("Environment variable error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error".to_string(), "ENV_VAR_ERROR")
            }
            AppError::Io(err) => {
                tracing::error!("IO error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "File operation error".to_string(), "IO_ERROR")
            }
            AppError::WebSocket(msg) => {
                tracing::error!("WebSocket error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "WebSocket communication error".to_string(), "WEBSOCKET_ERROR")
            }
        };

        let body = Json(json!({
            "success": false,
            "code": error_code,
            "message": error_message,
            "status_code": status.as_u16(),
            "timestamp": chrono::Utc::now().to_rfc3339()
        }));

        (status, body).into_response()
    }
}

// 自定义结果类型
pub type Result<T> = std::result::Result<T, AppError>;

// 为WebSocket错误添加转换
impl From<Box<dyn std::error::Error + Send + Sync>> for AppError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        AppError::WebSocket(err.to_string())
    }
}

// 移除重复的From实现，因为已经有#[from] std::io::Error

// 为multipart错误添加转换
impl From<axum::extract::multipart::MultipartError> for AppError {
    fn from(err: axum::extract::multipart::MultipartError) -> Self {
        AppError::Validation(format!("File upload error: {}", err))
    }
}

// 为Request错误添加转换
impl From<axum::extract::rejection::JsonRejection> for AppError {
    fn from(err: axum::extract::rejection::JsonRejection) -> Self {
        AppError::Validation(format!("JSON parsing error: {}", err))
    }
}

// 为anyhow错误添加转换
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}

// 为数据库迁移错误添加转换
impl From<sqlx::migrate::MigrateError> for AppError {
    fn from(err: sqlx::migrate::MigrateError) -> Self {
        AppError::Database(err.into())
    }
}
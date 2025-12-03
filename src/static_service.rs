use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, Response},
    routing::Router,
};
use std::path::PathBuf;
use std::sync::Arc;
use crate::state::AppState;

/// 简单的静态文件服务
pub async fn serve_static_file(Path(path): Path<String>) -> Result<Response, StatusCode> {
    let file_path = format!("assets/{}", path);

    // 安全检查：防止目录遍历
    if path.contains("..") || path.starts_with('.') {
        return Err(StatusCode::BAD_REQUEST);
    }

    let full_path = PathBuf::from(&file_path);

    // 检查文件是否存在
    match tokio::fs::metadata(&full_path).await {
        Ok(metadata) => {
            if !metadata.is_file() {
                return Err(StatusCode::NOT_FOUND);
            }

            // 读取文件内容
            match tokio::fs::read(&full_path).await {
                Ok(contents) => {
                    // 根据文件扩展名设置Content-Type
                    let extension = full_path.extension()
                        .and_then(|ext| ext.to_str())
                        .unwrap_or("");

                    let content_type = match extension {
                        Some("css") => "text/css",
                        Some("js") => "application/javascript",
                        Some("jpg") | Some("jpeg") => "image/jpeg",
                        Some("png") => "image/png",
                        Some("gif") => "image/gif",
                        Some("svg") => "image/svg+xml",
                        Some("pdf") => "application/pdf",
                        Some("html") => "text/html",
                        Some("json") => "application/json",
                        _ => "application/octet-stream",
                    };

                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .header("Content-Type", content_type)
                        .header("Cache-Control", "public, max-age=3600")
                        .body(axum::body::Body::from(contents))
                        .unwrap())
                }
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

/// 静态文件路由
pub fn create_static_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/static/*path", axum::routing::get(serve_static_file))
}
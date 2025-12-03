use axum::{
    body::{Body, Bytes},
    extract::Request,
    http::{HeaderMap, Method},
    middleware::Next,
    response::Response,
};
use std::time::Duration;
use tracing::{error, info, warn, Level};

// 请求日志中间件
pub async fn logging_middleware(request: Request, next: Next) -> Response {
    let start = std::time::Instant::now();

    // 提取请求信息
    let method = request.method().clone();
    let uri = request.uri().clone();
    let path = uri.path().to_string();
    let query = uri.query();
    let headers = request.headers().clone();

    // 获取客户端IP
    let client_ip = get_client_ip(&headers);

    // 获取用户代理
    let user_agent = headers
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown");

    // 记录请求开始
    info!(
        method = %method,
        path = %path,
        query = ?query,
        client_ip = %client_ip,
        user_agent = %user_agent,
        "Request started"
    );

    // 执行请求
    let response = next.run(request).await;

    // 计算响应时间
    let duration = start.elapsed();
    let status = response.status();
    let status_code = status.as_u16();

    // 根据状态码选择日志级别
    let (log_level, message) = match status_code {
        200..=299 => (Level::INFO, "Request completed successfully"),
        400..=499 => (Level::WARN, "Request completed with client error"),
        500..=599 => (Level::ERROR, "Request completed with server error"),
        _ => (Level::INFO, "Request completed"),
    };

    match log_level {
        Level::INFO => {
            info!(
                method = %method,
                path = %path,
                status = %status_code,
                duration_ms = duration.as_millis(),
                client_ip = %client_ip,
                "{}", message
            );
        }
        Level::WARN => {
            warn!(
                method = %method,
                path = %path,
                status = %status_code,
                duration_ms = duration.as_millis(),
                client_ip = %client_ip,
                "{}", message
            );
        }
        Level::ERROR => {
            error!(
                method = %method,
                path = %path,
                status = %status_code,
                duration_ms = duration.as_millis(),
                client_ip = %client_ip,
                "{}", message
            );
        }
        _ => {}
    }

    response
}

// 详细的请求日志中间件（包含请求体）
pub async fn detailed_logging_middleware(request: Request, next: Next) -> Response {
    let start = std::time::Instant::now();

    let (method, uri) = (request.method().clone(), request.uri().clone());
    let path = uri.path().to_string();
    let client_ip = get_client_ip(request.headers());

    // 对于非文件上传请求，尝试读取请求体
    if !is_multipart_request(request.headers()) {
        let (req, body) = request.into_parts();

        // 限制记录的请求体大小
        let body_bytes = match axum::body::to_bytes(body, 1024 * 1024).await {
            Ok(bytes) => bytes,
            Err(_) => Bytes::new(),
        };

        let body_str = if !body_bytes.is_empty() && should_log_body(&path) {
            String::from_utf8_lossy(&body_bytes[..body_bytes.len().min(1024)]).to_string()
        } else {
            String::new()
        };

        let request = Request::from_parts(req, Body::from(body_bytes));
        let response = next.run(request).await;
        let duration = start.elapsed();
        let status = response.status();

        log_request_details(
            &method,
            &path,
            &client_ip,
            Some(&body_str),
            status.as_u16(),
            duration,
        );

        response
    } else {
        // 对于multipart请求，不读取请求体
        let response = next.run(request).await;
        let duration = start.elapsed();
        let status = response.status();

        log_request_details(&method, &path, &client_ip, None, status.as_u16(), duration);

        response
    }
}

// 错误日志中间件
pub async fn error_logging_middleware(request: Request, next: Next) -> Response {
    // 直接运行请求，Next.run在Axum中不会返回Result
    let response = next.run(request).await;

    response
}

// 安全日志中间件（记录可疑请求）
pub async fn security_logging_middleware(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let headers = request.headers().clone();
    let client_ip = get_client_ip(&headers);

    // 检查可疑模式
    if is_suspicious_request(&method, &path, &headers) {
        warn!(
            method = %method,
            path = %path,
            client_ip = %client_ip,
            "Suspicious request detected"
        );

        // 可以在这里添加更多安全检查，如频率限制、IP黑名单等
    }

    // 检查认证尝试
    if path.starts_with("/api/auth/") {
        info!(
            method = %method,
            path = %path,
            client_ip = %client_ip,
            user_agent = ?headers.get("user-agent"),
            "Authentication attempt"
        );
    }

    next.run(request).await
}

// 获取客户端IP
fn get_client_ip(headers: &HeaderMap) -> String {
    headers
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.split(',').next())
        .or_else(|| headers.get("x-real-ip").and_then(|h| h.to_str().ok()))
        .or_else(|| {
            headers
                .get("cf-connecting-ip")
                .and_then(|h| h.to_str().ok())
        })
        .unwrap_or("unknown")
        .to_string()
}

// 检查是否为multipart请求
fn is_multipart_request(headers: &HeaderMap) -> bool {
    headers
        .get("content-type")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.starts_with("multipart/"))
        .unwrap_or(false)
}

// 判断是否应该记录请求体
fn should_log_body(path: &str) -> bool {
    // 敏感路径不记录请求体
    let sensitive_paths = [
        "/api/auth/login",
        "/api/auth/register",
        "/api/user/change-password",
        "/api/kyc/submit",
    ];

    !sensitive_paths
        .iter()
        .any(|sensitive| path.starts_with(sensitive))
}

// 记录详细的请求信息
fn log_request_details(
    method: &Method,
    path: &str,
    client_ip: &str,
    body: Option<&str>,
    status_code: u16,
    duration: Duration,
) {
    let log_entry = serde_json::json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "method": method.to_string(),
        "path": path,
        "client_ip": client_ip,
        "status_code": status_code,
        "duration_ms": duration.as_millis(),
        "body": body.unwrap_or(""),
    });

    if status_code >= 400 {
        warn!(
            request = ?log_entry,
            "Request completed with error"
        );
    } else {
        info!(
            request = ?log_entry,
            "Request completed"
        );
    }
}

// 检查可疑请求模式
fn is_suspicious_request(method: &Method, path: &str, headers: &HeaderMap) -> bool {
    // 检查常见攻击模式
    let suspicious_patterns = [
        "../",
        "..\\",
        "etc/passwd",
        "etc/shadow",
        "web.config",
        ".env",
        "<script",
        "javascript:",
        "eval(",
        "exec(",
        "system(",
    ];

    let path_lower = path.to_lowercase();

    // 检查路径中的可疑模式
    for pattern in &suspicious_patterns {
        if path_lower.contains(&pattern.to_lowercase()) {
            return true;
        }
    }

    // 检查User-Agent中的可疑模式
    if let Some(user_agent) = headers.get("user-agent").and_then(|h| h.to_str().ok()) {
        let suspicious_uas = [
            "sqlmap", "nikto", "nmap", "masscan", "sqlninja", "havij", "pangolin",
        ];

        let ua_lower = user_agent.to_lowercase();
        for sus_ua in &suspicious_uas {
            if ua_lower.contains(sus_ua) {
                return true;
            }
        }
    }

    // 检查异常的请求方法组合
    if matches!(method, &Method::GET | &Method::HEAD | &Method::OPTIONS) && path.contains("admin")
        || path.contains("wp-admin")
        || path.contains("wp-login")
    {
        return true;
    }

    false
}

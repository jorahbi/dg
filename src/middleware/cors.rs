use axum::{
    http::{HeaderValue, HeaderName, Method, StatusCode},
    response::Response,
};
use tower_http::cors::{AllowHeaders, AllowOrigin, CorsLayer};

// 创建CORS中间件
pub fn create_cors_layer(allowed_origins: Vec<&str>) -> CorsLayer {
    let origins: Vec<HeaderValue> = allowed_origins
        .into_iter()
        .filter_map(|origin| origin.parse().ok())
        .collect();

    CorsLayer::new()
        .allow_origin(AllowOrigin::list(origins))
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
            Method::PATCH,
        ])
        .allow_headers(AllowHeaders::list([
            axum::http::header::AUTHORIZATION,
            axum::http::header::ACCEPT,
            axum::http::header::CONTENT_TYPE,
            axum::http::header::ORIGIN,
            axum::http::header::USER_AGENT,
            HeaderName::from_static("x-requested-with"),
        ]))
        .allow_credentials(true)
        .expose_headers([
            axum::http::header::CONTENT_LENGTH,
            HeaderName::from_static("x-total-count"),
        ])
        .max_age(Duration::from_secs(86400)) // 24小时
}

// 自定义CORS处理中间件（用于更复杂的CORS逻辑）
use std::time::Duration;
use axum::{
    extract::Request,
    middleware::Next,
    http::header,
};

pub async fn custom_cors_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let origin = request
        .headers()
        .get(header::ORIGIN)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let method = request.method().clone();

    // 处理预检请求
    if method == Method::OPTIONS {
        let mut response = Response::new(axum::body::Body::empty());

        // 设置CORS头
        if let Some(origin) = origin {
            if is_origin_allowed(&origin) {
                response
                    .headers_mut()
                    .insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin.parse().unwrap());
            }
        }

        response.headers_mut().insert(
            header::ACCESS_CONTROL_ALLOW_METHODS,
            "GET, POST, PUT, DELETE, OPTIONS, PATCH".parse().unwrap(),
        );

        response.headers_mut().insert(
            header::ACCESS_CONTROL_ALLOW_HEADERS,
            "Authorization, Accept, Content-Type, Origin, User-Agent, X-Requested-With".parse().unwrap(),
        );

        response.headers_mut().insert(
            header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
            "true".parse().unwrap(),
        );

        response.headers_mut().insert(
            header::ACCESS_CONTROL_MAX_AGE,
            "86400".parse().unwrap(),
        );

        return Ok(response);
    }

    // 处理实际请求
    let mut response = next.run(request).await;

    // 为实际响应添加CORS头
    if let Some(origin) = origin {
        if is_origin_allowed(&origin) {
            response
                .headers_mut()
                .insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin.parse().unwrap());
        }
    }

    response.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
        "true".parse().unwrap(),
    );

    response.headers_mut().insert(
        header::ACCESS_CONTROL_EXPOSE_HEADERS,
        "Content-Length, X-Total-Count".parse().unwrap(),
    );

    Ok(response)
}

fn is_origin_allowed(origin: &str) -> bool {
    // 这里可以根据配置文件或环境变量来设置允许的源
    let allowed_origins = vec![
        "http://localhost:3000",
        "http://localhost:8080",
        "https://astrai.com",
        "https://www.astrai.com",
        // 添加更多允许的源
    ];

    allowed_origins.contains(&origin) || origin.ends_with(".astrai.com")
}

// 为不同环境创建CORS配置
pub fn create_cors_for_environment() -> CorsLayer {
    #[cfg(debug_assertions)]
    {
        // 开发环境：允许本地开发服务器
        create_cors_layer(vec![
            "http://localhost:3000",
            "http://localhost:8080",
            "http://127.0.0.1:3000",
            "http://127.0.0.1:8080",
        ])
    }

    #[cfg(not(debug_assertions))]
    {
        // 生产环境：只允许生产域名
        create_cors_layer(vec![
            "https://astrai.com",
            "https://www.astrai.com",
            "https://app.astrai.com",
        ])
    }
}
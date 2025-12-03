use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
    pub cors: CorsConfig,
    pub upload: UploadConfig,
    pub security: SecurityConfig,
    pub app: AppConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub keep_alive: u64,
    pub timeout: u64,
    pub site_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout: u64,
    pub idle_timeout: u64,
    pub max_lifetime: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration: i64,         // 秒
    pub refresh_expiration: i64, // 秒
    pub issuer: Option<HashSet<String>>,
    pub audience: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
    pub expose_headers: Vec<String>,
    pub max_age: u64,
    pub credentials: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadConfig {
    pub max_file_size: usize,
    pub allowed_extensions: Vec<String>,
    pub upload_path: String,
    pub qrcord_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub bcrypt_cost: u32,
    pub max_login_attempts: i32,
    pub account_lock_duration: i64, // 秒
    pub password_min_length: usize,
    pub password_require_special_chars: bool,
    pub password_require_numbers: bool,
    pub password_require_uppercase: bool,
    pub rate_limit_requests: u32,
    pub rate_limit_window: u64, // 秒
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub environment: String,
    pub log_level: String,
    pub debug: bool,
    pub timezone: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let config = Config {
            server: ServerConfig {
                host: env::var("APP_SERVER__HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: env::var("APP_SERVER__PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()?,
                workers: env::var("APP_SERVER__WORKERS")
                    .unwrap_or_else(|_| "4".to_string())
                    .parse()?,
                keep_alive: env::var("APP_SERVER__KEEP_ALIVE")
                    .unwrap_or_else(|_| "75".to_string())
                    .parse()?,
                timeout: env::var("APP_SERVER__TIMEOUT")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()?,
                site_url: env::var("APP_SERVER_URL").context("APP_SERVER_URL not set")?,
            },
            database: DatabaseConfig {
                url: env::var("APP_DATABASE__URL").expect("APP_DATABASE__URL must be set"),
                max_connections: env::var("APP_DATABASE__MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "20".to_string())
                    .parse()?,
                min_connections: env::var("APP_DATABASE__MIN_CONNECTIONS")
                    .unwrap_or_else(|_| "5".to_string())
                    .parse()?,
                connection_timeout: env::var("APP_DATABASE__CONNECTION_TIMEOUT")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()?,
                idle_timeout: env::var("APP_DATABASE__IDLE_TIMEOUT")
                    .unwrap_or_else(|_| "600".to_string())
                    .parse()?,
                max_lifetime: env::var("APP_DATABASE__MAX_LIFETIME")
                    .unwrap_or_else(|_| "1800".to_string())
                    .parse()?,
            },
            jwt: JwtConfig {
                secret: env::var("APP_JWT__SECRET").expect("APP_JWT__SECRET must be set"),
                expiration: env::var("APP_JWT__EXPIRATION")
                    .unwrap_or_else(|_| "86400".to_string())
                    .parse()?,
                refresh_expiration: env::var("APP_JWT__REFRESH_EXPIRATION")
                    .unwrap_or_else(|_| "604800".to_string())
                    .parse()?,
                issuer: Some(HashSet::from([
                    env::var("APP_JWT__ISSUER").unwrap_or_else(|_| "coin-dgai-api".to_string())
                ])),
                audience: Vec::from([
                    env::var("APP_JWT__AUDIENCE").unwrap_or_else(|_| "coin-dgai-users".to_string())
                ]),
            },
            cors: CorsConfig {
                allowed_origins: env::var("APP_CORS__ALLOWED_ORIGINS")
                    .unwrap_or_else(|_| "http://localhost:3000,http://localhost:8080".to_string())
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect(),
                allowed_methods: env::var("APP_CORS__ALLOWED_METHODS")
                    .unwrap_or_else(|_| "GET,POST,PUT,DELETE,OPTIONS".to_string())
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect(),
                allowed_headers: env::var("APP_CORS__ALLOWED_HEADERS")
                    .unwrap_or_else(|_| "*".to_string())
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect(),
                expose_headers: env::var("APP_CORS__EXPOSE_HEADERS")
                    .unwrap_or_else(|_| "".to_string())
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect(),
                max_age: env::var("APP_CORS__MAX_AGE")
                    .unwrap_or_else(|_| "86400".to_string())
                    .parse()?,
                credentials: env::var("APP_CORS__CREDENTIALS")
                    .unwrap_or_else(|_| "true".to_string())
                    .parse()
                    .unwrap_or(true),
            },
            upload: UploadConfig {
                max_file_size: env::var("APP_UPLOAD__MAX_FILE_SIZE")
                    .unwrap_or_else(|_| "5242880".to_string())
                    .parse()?,
                allowed_extensions: env::var("APP_UPLOAD__ALLOWED_EXTENSIONS")
                    .unwrap_or_else(|_| "jpg,jpeg,png,gif,webp".to_string())
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect(),
                upload_path: env::var("APP_UPLOAD__UPLOAD_PATH")
                    .unwrap_or_else(|_| "uploads".to_string()),
                qrcord_size: env::var("APP_UPLOAD__QRCODE_SIZE")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(200),
            },
            security: SecurityConfig {
                bcrypt_cost: env::var("APP_SECURITY__BCRYPT_COST")
                    .unwrap_or_else(|_| "12".to_string())
                    .parse()?,
                max_login_attempts: env::var("APP_SECURITY__MAX_LOGIN_ATTEMPTS")
                    .unwrap_or_else(|_| "5".to_string())
                    .parse()?,
                account_lock_duration: env::var("APP_SECURITY__ACCOUNT_LOCK_DURATION")
                    .unwrap_or_else(|_| "1800".to_string())
                    .parse()?,
                password_min_length: env::var("APP_SECURITY__PASSWORD_MIN_LENGTH")
                    .unwrap_or_else(|_| "8".to_string())
                    .parse()?,
                password_require_special_chars: env::var(
                    "APP_SECURITY__PASSWORD_REQUIRE_SPECIAL_CHARS",
                )
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
                password_require_numbers: env::var("APP_SECURITY__PASSWORD_REQUIRE_NUMBERS")
                    .unwrap_or_else(|_| "true".to_string())
                    .parse()
                    .unwrap_or(true),
                password_require_uppercase: env::var("APP_SECURITY__PASSWORD_REQUIRE_UPPERCASE")
                    .unwrap_or_else(|_| "true".to_string())
                    .parse()
                    .unwrap_or(true),
                rate_limit_requests: env::var("APP_SECURITY__RATE_LIMIT_REQUESTS")
                    .unwrap_or_else(|_| "100".to_string())
                    .parse()?,
                rate_limit_window: env::var("APP_SECURITY__RATE_LIMIT_WINDOW")
                    .unwrap_or_else(|_| "60".to_string())
                    .parse()?,
            },
            app: AppConfig {
                name: env::var("APP_APP__NAME").unwrap_or_else(|_| "Astra Ai API".to_string()),
                version: env::var("APP_APP__VERSION").unwrap_or_else(|_| "1.0.0".to_string()),
                environment: env::var("APP_APP__ENVIRONMENT")
                    .unwrap_or_else(|_| "development".to_string()),
                log_level: env::var("APP_APP__LOG_LEVEL").unwrap_or_else(|_| "debug".to_string()),
                debug: env::var("APP_APP__DEBUG")
                    .unwrap_or_else(|_| "true".to_string())
                    .parse()
                    .unwrap_or(true),
                timezone: env::var("APP_APP__TIMEZONE").unwrap_or_else(|_| "UTC".to_string()),
            },
        };

        Ok(config)
    }
}

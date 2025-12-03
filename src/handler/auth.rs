use crate::{
    error::{AppError, Result},
    extract::AuthUser,
    schema::{
        common::ApiResponse,
        user::{
            ChangePasswordReq, ForgotPasswordQuestionsReq, ForgotPasswordVerifyReq, LoginReq,
            LogoutReq, RegisterReq, ResetPasswordReq, SaveSecurityQuestionsReq,
        },
    },
    service::AuthService,
    state::AppState,
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;
use validator::Validate;

// 用户注册
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterReq>,
) -> Result<impl IntoResponse> {
    // 验证输入参数
    payload
        .validate()
        .map_err(|e| AppError::Validation(format!("Parameter validation failed: {}", e)))?;

    // 检查密码确认
    if payload.password != payload.confirm_password {
        return Err(AppError::Validation("Password confirmation does not match".to_string()));
    }

    // 创建认证服务
    let auth_service = AuthService::new(&state);

    // 执行注册
    let user_id = auth_service
        .register(&payload.username, &payload.password, &payload.invite_code)
        .await?;

    let response = ApiResponse::success_with_message(
        json!({
            "userId": user_id,
            "username": payload.username,
            "message": "注册成功，请设置密保问题"
        }),
        "Registration successful",
    );

    Ok((StatusCode::CREATED, Json(response)))
}

// 用户登录
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginReq>,
) -> Result<impl IntoResponse> {
    // 验证输入参数
    payload
        .validate()
        .map_err(|e| AppError::Validation(format!("Parameter validation failed: {}", e)))?;

    // 创建认证服务
    let auth_service = AuthService::new(&state);

    // 执行登录
    let login_result = auth_service
        .login(state, &payload.username, &payload.password)
        .await?;

    // login_result直接就是LoginRes，无需再次构造
    let response = ApiResponse::success(login_result);

    Ok(Json(response))
}

// 用户登出
pub async fn logout(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(_payload): Json<LogoutReq>,
) -> Result<impl IntoResponse> {
    // 创建认证服务
    let auth_service = AuthService::new(&state);

    // 执行登出（可以在这里实现token黑名单机制）
    auth_service.logout(auth_user.id).await?;

    let response = ApiResponse::success_with_message(
        json!({
            "success": true
        }),
        "Logout successful",
    );

    Ok(Json(response))
}

// 获取安全问题列表
pub async fn get_security_questions(State(state): State<AppState>) -> Result<impl IntoResponse> {
    let auth_service = AuthService::new(&state);
    let questions = auth_service.get_security_questions().await?;

    let response = ApiResponse::success_with_message(questions, "Retrieved successfully");
    Ok(Json(response))
}

// 保存安全问题答案
pub async fn save_security_questions(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<SaveSecurityQuestionsReq>,
) -> Result<impl IntoResponse> {
    // 验证输入参数
    payload
        .validate()
        .map_err(|e| AppError::Validation(format!("Parameter validation failed: {}", e)))?;

    let auth_service = AuthService::new(&state);
    // 暂时使用固定的user_id，实际应该根据username查询user_id
    auth_service
        .save_security_questions(auth_user.id, payload.questions)
        .await?;

    let response = ApiResponse::success_with_message(
        json!({
            "success": true,
        }),
        "Saved successfully",
    );

    Ok(Json(response))
}

// 检查密保问题设置状态
pub async fn check_security_questions(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(params): Query<serde_json::Value>,
) -> Result<impl IntoResponse> {
    let _username = params
        .get("username")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::Validation("Missing username parameter".to_string()))?;

    let auth_service = AuthService::new(&state);
    // 暂时使用固定的user_id，实际应该根据username查询
    let result = auth_service.check_security_questions(auth_user.id).await?;

    let response = ApiResponse::success(result);
    Ok(Json(response))
}

// 忘记密码 - 获取用户密保问题
pub async fn forgot_password_questions(
    State(state): State<AppState>,
    Json(payload): Json<ForgotPasswordQuestionsReq>,
) -> Result<impl IntoResponse> {
    // 验证输入参数
    payload
        .validate()
        .map_err(|e| AppError::Validation(format!("Parameter validation failed: {}", e)))?;

    let auth_service = AuthService::new(&state);

    // 根据用户名获取用户的安全问题
    let questions = auth_service
        .get_user_security_questions_by_username(&payload.username)
        .await?;

    let response = ApiResponse::success_with_message(
        json!({
            "questions": questions
        }),
        "Retrieved successfully",
    );

    Ok(Json(response))
}

// 忘记密码 - 验证密保问题答案
pub async fn forgot_password_verify(
    State(state): State<AppState>,
    Json(payload): Json<ForgotPasswordVerifyReq>,
) -> Result<impl IntoResponse> {
    // 验证输入参数
    payload
        .validate()
        .map_err(|e| AppError::Validation(format!("Parameter validation failed: {}", e)))?;
    if payload.answers.len() < 3 {
        return Err(AppError::Validation("Minimum 3 security questions required".to_string()));
    }
    let auth_service = AuthService::new(&state);

    // 验证安全问题答案
    let reset_token = auth_service
        .forgot_password_verify(&payload.username, payload.answers)
        .await?;

    let response = ApiResponse::success_with_message(
        json!({
            "verified": true,
            "username": payload.username,
            "token": reset_token
        }),
        "Verification successful",
    );

    Ok(Json(response))
}

// 重置密码
pub async fn reset_password(
    State(state): State<AppState>,
    Json(payload): Json<ResetPasswordReq>,
) -> Result<impl IntoResponse> {
    // 验证输入参数
    payload
        .validate()
        .map_err(|e| AppError::Validation(format!("Parameter validation failed: {}", e)))?;

    let auth_service = AuthService::new(&state);

    // 执行密码重置
    auth_service
        .reset_password(
            &payload.username,
            &payload.new_password,
            &payload.reset_token,
        )
        .await?;

    let response = ApiResponse::success_with_message(
        json!({
            "success": true,
            "username": payload.username,
            "timestamp": chrono::Utc::now().to_rfc3339()
        }),
        "Password reset successful",
    );

    Ok(Json(response))
}

// 修改密码
pub async fn change_password(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<ChangePasswordReq>,
) -> Result<impl IntoResponse> {
    // 验证输入参数
    payload
        .validate()
        .map_err(|e| AppError::Validation(format!("Parameter validation failed: {}", e)))?;

    let auth_service = AuthService::new(&state);
    auth_service
        .change_password(
            auth_user.id,
            &payload.current_password,
            &payload.new_password,
        )
        .await?;

    let response = ApiResponse::success_with_message(
        json!({
            "success": true,
            "updateTime": chrono::Utc::now().to_rfc3339()
        }),
        "Password change successful",
    );

    Ok(Json(response))
}

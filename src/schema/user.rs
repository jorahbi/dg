use serde::{Deserialize, Serialize};
use validator::Validate;

// 用户注册请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RegisterReq {
    #[validate(length(
        min = 3,
        max = 32,
        message = "Username length must be between 3-32 characters"
    ))]
    pub username: String,

    #[validate(length(min = 6, message = "Password length cannot be less than 6 characters"))]
    pub password: String,

    #[validate(must_match(other = "password", message = "Password confirmation does not match"))]
    #[serde(rename = "confirmPassword")]
    pub confirm_password: String,

    #[validate(length(min = 8, max = 8, message = "Invite code incorrect"))]
    #[serde(rename = "inviteCode")]
    pub invite_code: String,
}

// 用户登录请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LoginReq {
    #[validate(length(min = 1, message = "Username cannot be empty"))]
    pub username: String,

    #[validate(length(min = 1, message = "Password cannot be empty"))]
    pub password: String,
}

// 登录响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRes {
    pub username: String,
    pub token: String,
    #[serde(rename = "hasSecurityQuestions")]
    pub has_security_questions: bool,
}

// 用户信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfoRes {
    pub username: String,
    pub total_assets: f64,
    pub dg_amount: f64,
    pub user_level: i32,
    pub is_kyc_verified: bool,
    pub is_logged_in: bool,
    pub qr_code_url: String,
    pub invite_code: String,
}

impl From<crate::model::User> for UserInfoRes {
    fn from(user: crate::model::User) -> Self {
        let qr = user.qr_code_url;
        Self {
            username: user.username,
            total_assets: user.total_assets.to_string().parse::<f64>().unwrap_or(0.0),
            dg_amount: user.dg_amount.to_string().parse::<f64>().unwrap_or(0.0),
            user_level: user.user_level as i32,
            is_kyc_verified: user.is_kyc_verified == 1,
            is_logged_in: true, // 能到达这里说明已登录
            qr_code_url: qr.unwrap_or_else(|| "https://example.com/qrcode/default".to_string()),
            invite_code: user.invite_code,
        }
    }
}

// 用户资料响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileRes {
    pub avatar_url: Option<String>,
    pub security_questions: Vec<SecurityQuestionRes>,
}

// 安全问题响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityQuestionRes {
    pub id: u32,
    pub question: String,
}

// 保存安全问题请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SaveSecurityQuestionsReq {
    #[validate(length(min = 1, message = "Security question cannot be empty"))]
    pub questions: Vec<SecurityQuestionAnswerReq>,
}

// 安全问题答案请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SecurityQuestionAnswerReq {
    #[serde(rename = "id")]
    pub question_id: i64,

    #[serde(rename = "question")]
    #[validate(length(min = 1, message = "Answer cannot be empty"))]
    pub answer: String,
}

// 检查密保问题响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckSecurityQuestionsRes {
    pub username: String,
    pub has_security_questions: bool,
    pub questions_count: i32,
    pub message: String,
}

// 忘记密码 - 获取问题请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ForgotPasswordQuestionsReq {
    #[validate(length(min = 1, message = "Username cannot be empty"))]
    pub username: String,
}

// 忘记密码 - 获取问题响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgotPasswordQuestionsRes {
    pub username: String,
    pub questions: Vec<SecurityQuestionRes>,
}

// 忘记密码 - 验证答案请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ForgotPasswordVerifyReq {
    #[validate(length(min = 1, message = "Username cannot be empty"))]
    pub username: String,

    #[validate(length(min = 1, message = "Answer cannot be empty"))]
    pub answers: Vec<SecurityQuestionAnswerReq>,
}

// 忘记密码 - 验证答案响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgotPasswordVerifyRes {
    pub verified: bool,
    pub username: String,
    pub token: String,
}

// 重置密码请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ResetPasswordReq {
    #[validate(length(min = 1, message = "Username cannot be empty"))]
    pub username: String,

    #[validate(length(
        min = 6,
        message = "New password length cannot be less than 6 characters"
    ))]
    #[serde(rename = "newPassword")]
    pub new_password: String,

    #[validate(length(min = 1, message = "Reset token cannot be empty"))]
    #[serde(rename = "resetToken")]
    pub reset_token: String,
}

// 修改密码请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ChangePasswordReq {
    #[validate(length(min = 1, message = "Current password cannot be empty"))]
    pub current_password: String,

    #[validate(length(
        min = 6,
        message = "New password length cannot be less than 6 characters"
    ))]
    pub new_password: String,
}

// 用户登出请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoutReq {
    pub token: String,
}

//
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfoResponse {
    pub id: i64,
    pub is_kyc_verified: bool,
    pub user_level: i32,
    pub total_assets: String,
    pub dg_amount: String,
    pub invite_code: Option<String>,
    pub created_at: String,
    pub last_login_at: Option<String>,
}

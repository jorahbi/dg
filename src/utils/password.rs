use crate::config::SecurityConfig;
use crate::error::{AppError, Result};
use bcrypt::{hash, verify};

pub struct PasswordService {
    config: SecurityConfig,
}

impl PasswordService {
    pub fn new(config: SecurityConfig) -> Self {
        Self { config }
    }

    pub fn hash_password(&self, password: &str) -> Result<String> {
        let hashed = hash(password, self.config.bcrypt_cost)?;
        Ok(hashed)
    }

    pub fn verify_password(&self, password: &str, hashed: &str) -> Result<bool> {
        let is_valid = verify(password, hashed)?;
        Ok(is_valid)
    }

    pub fn validate_password_strength(&self, password: &str) -> Result<()> {
        // 检查密码长度
        if password.len() < self.config.password_min_length {
            return Err(AppError::Validation(format!(
                "Password length cannot be less than {} characters",
                self.config.password_min_length
            )));
        }

        // 检查是否包含数字
        if self.config.password_require_numbers && !password.chars().any(|c| c.is_ascii_digit()) {
            return Err(AppError::Validation("Password must contain at least one number".to_string()));
        }

        // 检查是否包含大写字母
        if self.config.password_require_uppercase
            && !password.chars().any(|c| c.is_ascii_uppercase())
        {
            return Err(AppError::Validation(
                "Password must contain at least one uppercase letter".to_string(),
            ));
        }

        // 检查是否包含特殊字符
        if self.config.password_require_special_chars {
            let special_chars = "!@#$%^&*()_+-=[]{}|;:,.<>?";
            if !password.chars().any(|c| special_chars.contains(c)) {
                return Err(AppError::Validation(
                    "Password must contain at least one special character".to_string(),
                ));
            }
        }

        Ok(())
    }

    pub fn generate_reset_token() -> String {
        use rand::distributions::Alphanumeric;
        use rand::{thread_rng, Rng};

        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect()
    }

    pub fn generate_invite_code() -> String {
        use rand::distributions::Alphanumeric;
        use rand::{thread_rng, Rng};

        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_security_config() -> SecurityConfig {
        SecurityConfig {
            bcrypt_cost: 4, // 降低测试时的计算成本
            max_login_attempts: 5,
            account_lock_duration: 1800,
            password_min_length: 8,
            password_require_special_chars: true,
            password_require_numbers: true,
            password_require_uppercase: true,
            rate_limit_requests: 100,
            rate_limit_window: 60,
        }
    }

    #[test]
    fn test_hash_and_verify_password() {
        let password_service = PasswordService::new(create_test_security_config());
        let password = "TestPassword123!";

        let hashed = password_service.hash_password(password).unwrap();
        assert_ne!(password, hashed); // 哈希后的密码应该不同

        let is_valid = password_service.verify_password(password, &hashed).unwrap();
        assert!(is_valid);

        let is_invalid = password_service
            .verify_password("WrongPassword", &hashed)
            .unwrap();
        assert!(!is_invalid);
    }

    #[test]
    fn test_validate_password_strength() {
        let password_service = PasswordService::new(create_test_security_config());

        // 测试符合要求的密码
        assert!(password_service
            .validate_password_strength("TestPassword123!")
            .is_ok());

        // 测试太短的密码
        assert!(password_service
            .validate_password_strength("Test1!")
            .is_err());

        // 测试没有数字的密码
        assert!(password_service
            .validate_password_strength("TestPassword!")
            .is_err());

        // 测试没有大写字母的密码
        assert!(password_service
            .validate_password_strength("testpassword123!")
            .is_err());

        // 测试没有特殊字符的密码
        assert!(password_service
            .validate_password_strength("TestPassword123")
            .is_err());
    }

    #[test]
    fn test_generate_tokens() {
        let reset_token = PasswordService::generate_reset_token();
        assert_eq!(reset_token.len(), 32);

        let invite_code = PasswordService::generate_invite_code();
        assert_eq!(invite_code.len(), 8);

        // 确保生成的令牌不同
        let reset_token2 = PasswordService::generate_reset_token();
        assert_ne!(reset_token, reset_token2);

        let invite_code2 = PasswordService::generate_invite_code();
        assert_ne!(invite_code, invite_code2);
    }
}

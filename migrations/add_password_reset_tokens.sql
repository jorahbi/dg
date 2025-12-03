-- 创建密码重置令牌表
CREATE TABLE `password_reset_tokens` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '重置令牌记录ID，主键',
  `user_id` bigint unsigned NOT NULL COMMENT '用户ID，外键关联users表',
  `token_hash` varchar(255) COLLATE utf8mb4_bin NOT NULL COMMENT '重置令牌的哈希值，用于验证',
  `expires_at` timestamp NOT NULL COMMENT '令牌过期时间，超过此时间令牌无效',
  `is_used` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否已使用，0=未使用，1=已使用',
  `used_at` timestamp NULL DEFAULT NULL COMMENT '使用时间，null表示未使用',
  `ip_address` varchar(45) COLLATE utf8mb4_bin DEFAULT NULL COMMENT '请求重置时的IP地址，用于安全审计',
  `user_agent` text COLLATE utf8mb4_bin COMMENT '用户代理信息，用于安全审计',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '令牌创建时间，自动创建时间戳',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '令牌最后更新时间，自动更新时间戳',
  PRIMARY KEY (`id`),
  UNIQUE KEY `idx_user_token` (`user_id`,`token_hash`),
  KEY `idx_token_hash` (`token_hash`),
  KEY `idx_expires_at` (`expires_at`),
  KEY `idx_is_used` (`is_used`),
  CONSTRAINT `fk_password_reset_user_id` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='密码重置令牌表，存储用户密码重置的临时令牌';
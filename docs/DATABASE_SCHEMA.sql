-- Coin项目数据库设计
-- 基于需求文档和API接口文档生成
-- 适用于MySQL 8.0+
-- 创建时间：2025-11-23 18:05:00
-- 版本：1.0.0

-- 创建数据库
CREATE DATABASE IF NOT EXISTS `coin_dgai`
DEFAULT CHARACTER SET utf8mb4
DEFAULT COLLATE utf8mb4_bin;

USE `coin_dgai`;

-- ================================
-- 用户相关表
-- ================================

-- 用户基本信息表
CREATE TABLE `users` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '用户唯一标识ID，主键',
    `username` VARCHAR(50) NOT NULL UNIQUE COMMENT '用户名，唯一索引，用于登录',
    `email` VARCHAR(255) NULL COMMENT '用户邮箱地址，可用于找回密码和通知',
    `password_hash` VARCHAR(255) NOT NULL COMMENT '用户密码的哈希值，使用bcrypt加密',
    `salt` VARCHAR(255) NOT NULL COMMENT '密码盐值，用于增强密码安全性',
    `user_level` TINYINT UNSIGNED NOT NULL DEFAULT 0 COMMENT '用户等级，0-5级对应LV.0-LV.5',
    `invite_code` VARCHAR(20) NOT NULL UNIQUE COMMENT '用户邀请码，用于邀请好友注册',
    `inviter_id` BIGINT UNSIGNED NULL COMMENT '邀请人的用户ID，外键关联users表',
    `total_assets` DECIMAL(20, 8) NOT NULL DEFAULT 0.00000000 COMMENT '用户总资产价值（USDT计价），8位小数精度',
    `dg_amount` DECIMAL(20, 8) NOT NULL DEFAULT 0.00000000 COMMENT '用户DG代币余额，8位小数精度',
    `is_kyc_verified` BOOLEAN NOT NULL DEFAULT FALSE COMMENT 'KYC认证状态，true=已认证，false=未认证',
    `is_email_verified` BOOLEAN NOT NULL DEFAULT FALSE COMMENT '邮箱验证状态，true=已验证，false=未验证',
    `has_security_questions` BOOLEAN NOT NULL DEFAULT FALSE COMMENT '是否设置密保问题，true=已设置，false=未设置',
    `is_active` BOOLEAN NOT NULL DEFAULT TRUE COMMENT '账户激活状态，true=正常，false=禁用',
    `is_locked` BOOLEAN NOT NULL DEFAULT FALSE COMMENT '账户锁定状态，true=锁定，false=正常',
    `login_attempts` TINYINT UNSIGNED NOT NULL DEFAULT 0 COMMENT '登录失败次数，超过阈值将锁定账户',
    `locked_until` TIMESTAMP NULL DEFAULT NULL COMMENT '账户锁定到期时间，null表示未锁定或永久锁定',
    `avatar_url` VARCHAR(500) NULL COMMENT '用户头像图片URL地址',
    `qr_code_url` VARCHAR(500) NULL COMMENT '用户二维码图片URL地址，用于邀请和收款',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '用户注册时间，自动创建时间戳',
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '用户信息最后更新时间，自动更新时间戳',
    `last_login_at` TIMESTAMP NULL DEFAULT NULL COMMENT '用户最后登录时间，null表示从未登录',
    PRIMARY KEY (`id`),
    UNIQUE KEY `idx_username` (`username`),
    UNIQUE KEY `idx_invite_code` (`invite_code`),
    KEY `idx_email` (`email`),
    KEY `idx_inviter_id` (`inviter_id`),
    KEY `idx_user_level` (`user_level`),
    KEY `idx_is_active` (`is_active`),
    KEY `idx_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='用户基本信息表';

-- 用户安全问题表
CREATE TABLE `user_security_questions` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '安全问题记录ID，主键',
    `user_id` BIGINT UNSIGNED NOT NULL COMMENT '用户ID，外键关联users表',
    `question_id` TINYINT UNSIGNED NOT NULL COMMENT '安全问题ID，对应系统预定义的问题列表',
    `answer_hash` VARCHAR(255) NOT NULL COMMENT '问题答案的哈希值，使用bcrypt加密',
    `answer_salt` VARCHAR(255) NOT NULL COMMENT '答案盐值，用于增强答案安全性',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '安全问题设置时间，自动创建时间戳',
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '安全问题最后更新时间，自动更新时间戳',
    PRIMARY KEY (`id`),
    UNIQUE KEY `idx_user_question` (`user_id`, `question_id`),
    KEY `idx_user_id` (`user_id`),

) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='用户安全问题表，存储用户设置的密保问题和答案';

-- 用户会话表
CREATE TABLE `user_sessions` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '会话记录ID，主键',
    `user_id` BIGINT UNSIGNED NOT NULL COMMENT '用户ID，外键关联users表',
    `token` VARCHAR(512) NOT NULL UNIQUE COMMENT 'JWT会话令牌，用于API认证，唯一索引',
    `device_info` JSON NULL COMMENT '设备信息JSON，包含设备类型、操作系统、设备型号等',
    `ip_address` VARCHAR(45) NULL COMMENT '用户登录IP地址，支持IPv4和IPv6格式',
    `user_agent` TEXT NULL COMMENT '用户代理字符串，包含浏览器和客户端信息',
    `is_active` BOOLEAN NOT NULL DEFAULT TRUE COMMENT '会话激活状态，true=有效，false=已失效',
    `expires_at` TIMESTAMP NOT NULL COMMENT '会话过期时间，超过此时间会话自动失效',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '会话创建时间，自动创建时间戳',
    `last_used_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '会话最后使用时间，用于延长会话有效期',
    PRIMARY KEY (`id`),
    UNIQUE KEY `idx_token` (`token`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_expires_at` (`expires_at`),
    KEY `idx_is_active` (`is_active`),

) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='用户会话表，管理用户登录状态和令牌';

-- ================================
-- 资产相关表
-- ================================

-- 用户资产表
CREATE TABLE `user_assets` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '资产记录ID，主键',
    `user_id` BIGINT UNSIGNED NOT NULL COMMENT '用户ID，外键关联users表',
    `currency` VARCHAR(20) NOT NULL COMMENT '货币类型代码，如USDT、DG、BTC、ETH等',
    `balance` DECIMAL(20, 8) NOT NULL DEFAULT 0.00000000 COMMENT '可用余额，8位小数精度，用户可以自由支配的金额',
    `frozen_balance` DECIMAL(20, 8) NOT NULL DEFAULT 0.00000000 COMMENT '冻结余额，8位小数精度，如提现中、订单待支付等冻结的金额',
    `total_earned` DECIMAL(20, 8) NOT NULL DEFAULT 0.00000000 COMMENT '总收益金额，8位小数精度，历史累计获得的总收益',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '资产账户创建时间，自动创建时间戳',
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '资产最后更新时间，自动更新时间戳',
    PRIMARY KEY (`id`),
    UNIQUE KEY `idx_user_currency` (`user_id`, `currency`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_currency` (`currency`),

) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='用户资产表，存储用户各种数字货币的余额和收益信息';

-- 交易记录表
CREATE TABLE `transactions` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '交易记录ID，主键',
    `user_id` BIGINT UNSIGNED NOT NULL COMMENT '用户ID，外键关联users表',
    `transaction_id` VARCHAR(64) NOT NULL UNIQUE COMMENT '交易唯一标识，系统生成的交易ID，唯一索引',
    `type` ENUM('deposit', 'withdraw', 'exchange', 'purchase', 'airdrop', 'referral', 'task_earning', 'mining_earning') NOT NULL COMMENT '交易类型：充值/提现/兑换/购买/空投/邀请/任务收益/挖矿收益',
    `from_currency` VARCHAR(20) NULL COMMENT '源货币代码，兑换交易的转出货币，如BTC、USDT等',
    `to_currency` VARCHAR(20) NULL COMMENT '目标货币代码，兑换交易的转入货币，如ETH、DG等',
    `amount` DECIMAL(20, 8) NOT NULL COMMENT '交易金额，8位小数精度，交易的主要数值',
    `fee` DECIMAL(20, 8) NOT NULL DEFAULT 0.00000000 COMMENT '交易手续费，8位小数精度，平台收取的服务费用',
    `exchange_rate` DECIMAL(20, 8) NULL COMMENT '兑换汇率，用于币种兑换交易，表示两个货币之间的兑换比例',
    `status` ENUM('pending', 'processing', 'completed', 'failed', 'cancelled') NOT NULL DEFAULT 'pending' COMMENT '交易状态：pending待处理/processing处理中/completed已完成/failed失败/cancelled已取消',
    `blockchain_type` VARCHAR(20) NULL COMMENT '区块链类型，如TRC20、ERC20等，用于链上交易',
    `transaction_hash` VARCHAR(128) NULL COMMENT '区块链交易哈希，用于追踪链上交易状态',
    `from_address` VARCHAR(255) NULL COMMENT '转出地址，充值或提现的区块链钱包地址',
    `to_address` VARCHAR(255) NULL COMMENT '转入地址，充值或提现的区块链钱包地址',
    `confirmations` INT UNSIGNED NULL COMMENT '区块链确认数，表示交易被区块确认的次数，越多越安全',
    `description` TEXT NULL COMMENT '交易描述，用户友好的交易说明',
    `metadata` JSON NULL COMMENT '交易元数据JSON，存储额外的交易相关信息',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '交易创建时间，自动创建时间戳',
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '交易最后更新时间，自动更新时间戳',
    `completed_at` TIMESTAMP NULL DEFAULT NULL COMMENT '交易完成时间，null表示未完成',
    PRIMARY KEY (`id`),
    UNIQUE KEY `idx_transaction_id` (`transaction_id`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_type` (`type`),
    KEY `idx_status` (`status`),
    KEY `idx_currency` (`from_currency`, `to_currency`),
    KEY `idx_created_at` (`created_at`),

) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='交易记录表';

-- ================================
-- 算力相关表
-- ================================

-- 算力包配置表
CREATE TABLE `power_packages` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '算力包配置ID，主键',
  `title` json NOT NULL COMMENT '算力包名称，如"基础算力包"、"高级算力包"等',
  `lv` SMALLINT NOT NULL DEFAULT '0' COMMENT '购买所需的最低用户等级，0-5对应LV.0-LV.5',
  `daily_yield_percentage` decimal(10,4) NOT NULL COMMENT '收益率百分比，表示每日收益的百分比，如0.5表示日收益率0.5%',
  `amount` decimal(20,8) NOT NULL COMMENT '算力包价格，8位小数精度，用户需要支付的费用',
  `description` json NOT NULL COMMENT '算力包详细描述，包含服务内容和收益说明，帮助用户了解产品',
  `status` tinyint(1) NOT NULL COMMENT '算力包状态，true=在售，false=停售',
  `sort_order` int unsigned NOT NULL DEFAULT '0' COMMENT '排序字段，用于前端显示顺序',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '算力包创建时间，自动创建时间戳',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '算力包最后更新时间，自动更新时间戳',
  PRIMARY KEY (`id`),
  KEY `idx_sort_order` (`sort_order`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='算力包配置表，定义可购买的算力产品规格和价格';

-- 用户算力记录表
CREATE TABLE `user_power_records` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '算力记录ID，主键',
    `user_id` BIGINT UNSIGNED NOT NULL COMMENT '用户ID，外键关联users表',
    `power_package_id` BIGINT UNSIGNED NOT NULL COMMENT '算力包ID，外键关联power_packages表',
    `order_id` BIGINT UNSIGNED NULL COMMENT '关联的订单ID，外键关联orders表，null表示非订单购买',
    `type` VARCHAR(50) NOT NULL COMMENT '算力类型，继承自算力包的task_type字段',
    `amount` DECIMAL(20, 8) NOT NULL COMMENT '算力包价格，8位小数精度',
    `currency` VARCHAR(20) NOT NULL DEFAULT 'USDT' COMMENT '价格货币单位，默认USDT',
    `start_time` TIMESTAMP NOT NULL COMMENT '算力开始生效时间',
    `end_time` TIMESTAMP NOT NULL COMMENT '算力到期时间',
    `status` ENUM('active', 'expired', 'cancelled') NOT NULL DEFAULT 'active' COMMENT '算力状态：生效中/已过期/已取消',
    `earnings` DECIMAL(20, 8) NOT NULL DEFAULT 0.00000000 COMMENT '累计收益金额，8位小数精度',
    `current_hashrate` DECIMAL(20, 8) NULL COMMENT '当前算力值，8位小数精度，单位TH/s',
    `total_hashrate` DECIMAL(20, 8) NULL COMMENT '总算力值，8位小数精度，单位TH/s',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '算力记录创建时间，自动创建时间戳',
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '算力记录最后更新时间，自动更新时间戳',
    PRIMARY KEY (`id`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_power_package_id` (`power_package_id`),
    KEY `idx_order_id` (`order_id`),
    KEY `idx_status` (`status`),
    KEY `idx_start_time` (`start_time`),
    KEY `idx_end_time` (`end_time`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='用户算力记录表，记录用户购买的算力包使用情况和收益';

-- 用户等级表
CREATE TABLE `user_levels` (
    `id` TINYINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '用户等级ID，主键，0-5对应LV.0-LV.5',
    `name` VARCHAR(50) NOT NULL COMMENT '等级名称，如"LV.1"、"LV.2"等',
    `description` TEXT NULL COMMENT '等级描述，说明该等级的特权和要求',
    `required_invites` INT UNSIGNED NOT NULL DEFAULT 0 COMMENT '升级所需邀请人数，邀请好友达到此数量可升级',
    `reward_multiplier` DECIMAL(10, 4) NOT NULL DEFAULT 1.0000 COMMENT '收益倍数，该等级用户的收益乘数，如1.2000表示20%加成',
    `min_power` DECIMAL(20, 8) NOT NULL DEFAULT 0.00000000 COMMENT '最低算力要求，达到此算力金额才能维持或升级到该等级',
    `icon_url` VARCHAR(500) NULL COMMENT '等级图标URL地址，用于前端显示等级徽章',
    `is_active` BOOLEAN NOT NULL DEFAULT TRUE COMMENT '等级状态，true=启用，false=停用，停用等级不可升级',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '等级创建时间，自动创建时间戳',
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '等级最后更新时间，自动更新时间戳',
    PRIMARY KEY (`id`),
    KEY `idx_required_invites` (`required_invites`),
    KEY `idx_is_active` (`is_active`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='用户等级表，定义用户等级体系、升级条件和权益奖励';

-- 提现记录表
CREATE TABLE `withdrawal_requests` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '提现记录ID，主键',
    `user_id` BIGINT UNSIGNED NOT NULL COMMENT '提现用户ID，外键关联users表',
    `withdrawal_id` VARCHAR(64) NOT NULL UNIQUE COMMENT '提现唯一标识，系统生成的提现单号，唯一索引',
    `amount` DECIMAL(20, 8) NOT NULL COMMENT '提现金额，8位小数精度，用户申请提现的具体金额',
    `currency` VARCHAR(20) NOT NULL COMMENT '提现货币类型，如USDT、DG、BTC、ETH等',
    `blockchain_type` VARCHAR(20) NOT NULL COMMENT '区块链网络类型，如TRC20、ERC20、BEP20等，用于区分不同网络提现',
    `destination_address` VARCHAR(255) NOT NULL COMMENT '目标接收地址，用户提供的数字货币钱包地址',
    `status` ENUM('pending', 'processing', 'completed', 'failed', 'cancelled') NOT NULL DEFAULT 'pending' COMMENT '提现状态：pending待审核/processing处理中/completed已完成/failed失败/cancelled已取消',
    `fee` DECIMAL(20, 8) NOT NULL DEFAULT 0.00000000 COMMENT '提现手续费，8位小数精度，实际到账金额 = amount - fee',
    `transaction_hash` VARCHAR(128) NULL COMMENT '区块链交易哈希，用于查询和验证链上交易状态',
    `confirmations` INT UNSIGNED NULL COMMENT '区块链确认数，表示交易被区块确认的次数',
    `reviewer_id` BIGINT UNSIGNED NULL COMMENT '审核人员ID，外键关联管理员表，null表示未指定审核人',
    `rejection_reason` TEXT NULL COMMENT '拒绝原因，提现申请被拒绝时的详细说明',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '提现申请创建时间，自动创建时间戳',
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '提现记录最后更新时间，自动更新时间戳',
    `processed_at` TIMESTAMP NULL DEFAULT NULL COMMENT '提现处理完成时间，null表示未处理',
    PRIMARY KEY (`id`),
    UNIQUE KEY `idx_withdrawal_id` (`withdrawal_id`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_status` (`status`),
    KEY `idx_currency` (`currency`),
    KEY `idx_created_at` (`created_at`),

) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='提现记录表，记录用户数字货币提现申请、审核和处理的全过程';

-- ================================
-- 订单相关表
-- ================================

-- 订单表
CREATE TABLE `orders` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '订单记录ID，主键',
    `order_id` VARCHAR(64) NOT NULL UNIQUE COMMENT '订单唯一标识，系统生成的订单ID，用于API查询和验证',
    `order_number` VARCHAR(50) NOT NULL COMMENT '订单编号，用户友好的订单展示编号，具有业务可读性',
    `user_id` BIGINT UNSIGNED NOT NULL COMMENT '下单用户ID，外键关联users表',
    `power_package_id` BIGINT UNSIGNED NOT NULL COMMENT '算力包产品ID，外键关联power_packages表',
    `quantity` INT UNSIGNED NOT NULL DEFAULT 1 COMMENT '购买数量，默认为1份，支持批量购买',
    `amount` DECIMAL(20, 8) NOT NULL COMMENT '订单总金额，8位小数精度，amount = quantity × 算力包单价',
    `currency` VARCHAR(20) NOT NULL DEFAULT 'USDT' COMMENT '支付货币类型，默认USDT，支持多种数字货币支付',
    `blockchain_type` VARCHAR(50) NOT NULL COMMENT '区块链网络类型，如TRC20、ERC20、BEP20等，用于区分不同网络支付',
    `blockchain_address` VARCHAR(255) NOT NULL COMMENT '支付接收地址，用户需要向此地址转账支付订单',
    `transaction_hash` VARCHAR(128) NULL COMMENT '区块链交易哈希，用于验证支付完成状态，支付成功后填写',
    `is_paid` BOOLEAN NOT NULL DEFAULT FALSE COMMENT '支付状态标识，true=已支付，false=待支付',
    `status` ENUM('pending', 'paid', 'completed', 'cancelled', 'expired') NOT NULL DEFAULT 'pending' COMMENT '订单状态：pending待支付/paid已支付/completed已完成/cancelled已取消/expired已过期',
    `paid_at` TIMESTAMP NULL DEFAULT NULL COMMENT '支付完成时间，null表示未支付或支付失败',
    `expired_at` TIMESTAMP NOT NULL COMMENT '订单过期时间，超过此时间订单自动失效',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '订单创建时间，自动创建时间戳',
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '订单最后更新时间，自动更新时间戳',
    PRIMARY KEY (`id`),
    UNIQUE KEY `idx_order_id` (`order_id`),
    UNIQUE KEY `idx_order_number` (`order_number`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_power_package_id` (`power_package_id`),
    KEY `idx_status` (`status`),
    KEY `idx_is_paid` (`is_paid`),
    KEY `idx_created_at` (`created_at`),
    KEY `idx_expired_at` (`expired_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='订单表，记录用户购买算力包的订单信息，包括支付状态和订单生命周期管理';

-- ================================
-- 空投相关表
-- ================================

-- 空投活动配置表
CREATE TABLE `airdrop_configs` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '空投活动配置ID，主键',
    `type` VARCHAR(50) NOT NULL COMMENT '空投类型：daily每日空投/vip会员空投/newbie新手空投/limited限时空投/public公开奖/premium高级算力空投',
    `title` VARCHAR(200) NOT NULL COMMENT '空投活动标题',
    `subtitle` VARCHAR(500) NULL COMMENT '空投活动副标题或说明',
    `description` TEXT NOT NULL COMMENT '空投活动详细描述，包含规则和奖励说明',
    `activity_start_time` TIMESTAMP NOT NULL COMMENT '活动开始时间',
    `activity_end_time` TIMESTAMP NOT NULL COMMENT '活动结束时间',
    `total_rounds` INT UNSIGNED NOT NULL COMMENT '总轮次数，如999轮表示有999次抢空投机会',
    `round_duration` INT UNSIGNED NOT NULL COMMENT '每轮持续时间（秒），如60秒每轮',
    `interval_duration` INT UNSIGNED NOT NULL COMMENT '轮次间隔时间（秒），如20秒间隔',
    `participation_type` ENUM('allUsers', 'memberLevel', 'newUsers', 'powerHolders') NOT NULL COMMENT '参与条件：所有用户/指定等级/新用户/算力持有者',
    `required_level` TINYINT UNSIGNED NULL COMMENT '参与所需的最低用户等级，仅当participation_type=memberLevel时有效',
    `min_power_amount` DECIMAL(20, 8) NULL COMMENT '参与所需的最小算力金额，仅当participation_type=powerHolders时有效',
    `color` VARCHAR(20) NULL COMMENT '活动主题色，用于前端显示，十六进制颜色值',
    `status` ENUM('active', 'inactive', 'expired') NOT NULL DEFAULT 'active' COMMENT '活动状态：激活/停用/已过期',
    `participant_count` BIGINT UNSIGNED NOT NULL DEFAULT 0 COMMENT '累计参与人数',
    `success_rate` DECIMAL(5, 4) NULL COMMENT '成功率，如0.856表示85.6%的成功率',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '空投配置创建时间，自动创建时间戳',
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '空投配置最后更新时间，自动更新时间戳',
    PRIMARY KEY (`id`),
    KEY `idx_type` (`type`),
    KEY `idx_participation_type` (`participation_type`),
    KEY `idx_status` (`status`),
    KEY `idx_activity_time` (`activity_start_time`, `activity_end_time`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='空投活动配置表，定义各种空投活动的规则和参数';

-- 空投轮次表
CREATE TABLE `airdrop_rounds` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '空投轮次ID，主键',
    `airdrop_config_id` BIGINT UNSIGNED NOT NULL COMMENT '空投活动配置ID，外键关联airdrop_configs表',
    `round_number` INT UNSIGNED NOT NULL COMMENT '轮次编号，从1开始的连续数字，表示当前是第几轮空投',
    `start_time` TIMESTAMP NOT NULL COMMENT '轮次开始时间，用户可以开始参与抢空投的时间',
    `end_time` TIMESTAMP NOT NULL COMMENT '轮次结束时间，本轮次空投抢购结束时间',
    `total_dg_amount` DECIMAL(20, 8) NOT NULL COMMENT '本轮次DG代币总奖励金额，8位小数精度',
    `participant_count` INT UNSIGNED NOT NULL DEFAULT 0 COMMENT '参与人数，本轮次参与抢空投的总用户数',
    `success_count` INT UNSIGNED NOT NULL DEFAULT 0 COMMENT '成功人数，本轮次成功抢到空投的用户数',
    `status` ENUM('pending', 'active', 'completed') NOT NULL DEFAULT 'pending' COMMENT '轮次状态：pending待开始/active进行中/completed已完成',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '轮次创建时间，自动创建时间戳',
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '轮次最后更新时间，自动更新时间戳',
    PRIMARY KEY (`id`),
    UNIQUE KEY `idx_config_round` (`airdrop_config_id`, `round_number`),
    KEY `idx_airdrop_config_id` (`airdrop_config_id`),
    KEY `idx_status` (`status`),
    KEY `idx_start_time` (`start_time`),

) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='空投轮次表，记录空投活动的每一轮次详细信息和统计数据';

-- 空投参与记录表
CREATE TABLE `airdrop_participations` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '空投参与记录ID，主键',
    `user_id` BIGINT UNSIGNED NOT NULL COMMENT '参与用户ID，外键关联users表',
    `airdrop_config_id` BIGINT UNSIGNED NOT NULL COMMENT '空投活动配置ID，外键关联airdrop_configs表',
    `airdrop_round_id` BIGINT UNSIGNED NULL COMMENT '空投轮次ID，外键关联airdrop_rounds表，null表示未关联具体轮次',
    `transaction_id` VARCHAR(64) NULL COMMENT '关联交易ID，用于追踪空投奖励发放的交易记录',
    `status` ENUM('success', 'failed', 'pending') NOT NULL DEFAULT 'pending' COMMENT '参与状态：success成功/failed失败/pending处理中',
    `dg_amount` DECIMAL(20, 8) NULL COMMENT '获得的DG代币数量，8位小数精度，成功抢到空投时填写',
    `round` INT UNSIGNED NULL COMMENT '参与轮次编号，标识用户参与的是第几轮空投',
    `failure_reason` TEXT NULL COMMENT '失败原因，空投抢购失败时的详细说明，如"已参与过本轮""未中签"等',
    `claimed_at` TIMESTAMP NULL DEFAULT NULL COMMENT '领取时间，用户成功领取空投奖励的时间，null表示未领取',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '参与记录创建时间，自动创建时间戳',
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '参与记录最后更新时间，自动更新时间戳',
    PRIMARY KEY (`id`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_airdrop_config_id` (`airdrop_config_id`),
    KEY `idx_airdrop_round_id` (`airdrop_round_id`),
    KEY `idx_status` (`status`),
    KEY `idx_claimed_at` (`claimed_at`),

) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='空投参与记录表，记录用户参与空投活动的详细情况和奖励发放状态';

-- ================================
-- 任务相关表
-- ================================

-- 任务配置表
CREATE TABLE `task_configs` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '任务配置ID，主键',
    `name` VARCHAR(200) NOT NULL COMMENT '任务名称，用户友好的任务展示名称',
    `task_type` ENUM('text_training', 'table_training', 'image_training', 'video_training') NOT NULL COMMENT '任务类型：text_training文本训练/table_training表格训练/image_training图像训练/video_training视频训练',
    `required_level` TINYINT UNSIGNED NOT NULL DEFAULT 0 COMMENT '所需最低用户等级，0-5对应LV.0-LV.5，用户达到此等级才能接取任务',
    `earnings_percent` DECIMAL(10, 4) NOT NULL COMMENT '收益率百分比，任务完成后的收益比例',
    `amount` DECIMAL(20, 8) NOT NULL COMMENT '任务奖励金额，8位小数精度，完成任务后获得的奖励',
    `currency` VARCHAR(20) NOT NULL DEFAULT 'USDT' COMMENT '奖励货币类型，默认USDT',
    `description` TEXT NOT NULL COMMENT '任务详细描述，说明任务内容、要求和注意事项',
    `difficulty` ENUM('easy', 'medium', 'hard') NOT NULL DEFAULT 'easy' COMMENT '任务难度：easy简单/medium中等/hard困难，影响奖励金额和完成时间',
    `completion_time` VARCHAR(50) NOT NULL COMMENT '预计完成时间，如"2-3小时"，给用户的参考时间',
    `is_active` BOOLEAN NOT NULL DEFAULT TRUE COMMENT '任务状态，true=可用，false=停用，停用任务不对外显示',
    `sort_order` INT UNSIGNED NOT NULL DEFAULT 0 COMMENT '排序字段，用于前端显示顺序，数字越小排序越靠前',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '任务配置创建时间，自动创建时间戳',
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '任务配置最后更新时间，自动更新时间戳',
    PRIMARY KEY (`id`),
    KEY `idx_task_type` (`task_type`),
    KEY `idx_required_level` (`required_level`),
    KEY `idx_difficulty` (`difficulty`),
    KEY `idx_is_active` (`is_active`),
    KEY `idx_sort_order` (`sort_order`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='任务配置表，定义AI训练任务的具体信息、奖励规则和参与条件';

-- 用户任务记录表
CREATE TABLE `user_tasks` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '用户任务记录ID，主键',
    `user_id` BIGINT UNSIGNED NOT NULL COMMENT '用户ID，外键关联users表',
    `task_config_id` BIGINT UNSIGNED NOT NULL COMMENT '任务配置ID，外键关联task_configs表',
    `status` ENUM('available', 'running', 'completed', 'failed', 'cancelled') NOT NULL DEFAULT 'available' COMMENT '任务状态：available可用/running进行中/completed已完成/failed失败/cancelled已取消',
    `start_time` TIMESTAMP NULL DEFAULT NULL COMMENT '任务开始时间，null表示未开始',
    `end_time` TIMESTAMP NULL DEFAULT NULL COMMENT '任务结束时间，null表示未结束',
    `estimated_completion_time` TIMESTAMP NULL DEFAULT NULL COMMENT '预计完成时间，根据任务难度和历史数据估算',
    `is_accelerating` BOOLEAN NOT NULL DEFAULT FALSE COMMENT '是否使用加速，true=使用积分加速，false=正常速度',
    `acceleration_multiplier` DECIMAL(10, 4) NOT NULL DEFAULT 1.0000 COMMENT '加速倍数，使用加速后的速度倍率，如1.5000表示1.5倍速',
    `points_used` BIGINT UNSIGNED NOT NULL DEFAULT 0 COMMENT '已使用积分数量，用于加速任务的积分消耗',
    `earnings_rate` DECIMAL(20, 8) NULL COMMENT '收益率，用户完成任务的实时收益速率，8位小数精度',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '任务记录创建时间，自动创建时间戳',
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '任务记录最后更新时间，自动更新时间戳',
    PRIMARY KEY (`id`),
    UNIQUE KEY `idx_user_task` (`user_id`, `task_config_id`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_task_config_id` (`task_config_id`),
    KEY `idx_status` (`status`),
    KEY `idx_start_time` (`start_time`),

) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='用户任务记录表，记录用户接取和执行AI训练任务的详细过程和状态';

-- ================================
-- 邀请相关表
-- ================================

-- 邀请记录表
CREATE TABLE `invite_records` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '邀请记录ID，主键',
    `inviter_id` BIGINT UNSIGNED NOT NULL COMMENT '邀请人用户ID，外键关联users表，发出邀请的用户',
    `invitee_id` BIGINT UNSIGNED NOT NULL COMMENT '被邀请人用户ID，外键关联users表，接受邀请的用户',
    `invite_code` VARCHAR(20) NOT NULL COMMENT '使用的邀请码，与邀请人的invite_code对应',
    `status` ENUM('pending', 'registered', 'completed') NOT NULL DEFAULT 'pending' COMMENT '邀请状态：pending待注册/registered已注册/completed已完成（完成KYC或其他条件）',
    `invite_level` TINYINT UNSIGNED NOT NULL DEFAULT 1 COMMENT '邀请层级，1表示直接邀请，2表示二级邀请，依此类推',
    `join_date` TIMESTAMP NULL DEFAULT NULL COMMENT '被邀请人注册时间，null表示未注册',
    `direct_reward` DECIMAL(20, 8) NOT NULL DEFAULT 0.00000000 COMMENT '直接邀请奖励，邀请人从被邀请人获得的直接奖励，8位小数精度',
    `indirect_reward` DECIMAL(20, 8) NOT NULL DEFAULT 0.00000000 COMMENT '间接邀请奖励，从下级邀请中获得的间接奖励，8位小数精度',
    `total_reward` DECIMAL(20, 8) NOT NULL DEFAULT 0.00000000 COMMENT '总奖励金额，直接奖励+间接奖励的总和，8位小数精度',
    `reward_currency` VARCHAR(20) NOT NULL DEFAULT 'DG' COMMENT '奖励货币类型，默认DG代币',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '邀请记录创建时间，自动创建时间戳',
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '邀请记录最后更新时间，自动更新时间戳',
    PRIMARY KEY (`id`),
    KEY `idx_inviter_id` (`inviter_id`),
    KEY `idx_invitee_id` (`invitee_id`),
    KEY `idx_invite_code` (`invite_code`),
    KEY `idx_status` (`status`),
    KEY `idx_invite_level` (`invite_level`),
    KEY `idx_join_date` (`join_date`),

) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='邀请记录表，记录用户邀请关系、奖励发放和邀请状态跟踪';

-- 邀请奖励配置表
CREATE TABLE `invite_reward_configs` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '邀请奖励配置ID，主键',
    `reward_id` VARCHAR(50) NOT NULL UNIQUE COMMENT '奖励唯一标识，用于系统内部引用和追踪',
    `title` VARCHAR(200) NOT NULL COMMENT '奖励标题，用户友好的奖励名称，如"邀请1位好友奖励"',
    `description` TEXT NOT NULL COMMENT '奖励描述，详细说明获得奖励的条件和奖励内容',
    `level` TINYINT UNSIGNED NOT NULL COMMENT '奖励等级，对应邀请等级或里程碑，如1级、2级等',
    `required_progress` INT UNSIGNED NOT NULL COMMENT '所需进度值，达到此数值可获得奖励，如邀请人数、任务完成数等',
    `reward_amount` DECIMAL(20, 8) NOT NULL COMMENT '奖励数量，发放给用户的奖励金额或积分数量，8位小数精度',
    `reward_type` ENUM('points', 'dg', 'usdt') NOT NULL COMMENT '奖励类型：points积分/dg代币/usdt稳定币',
    `is_active` BOOLEAN NOT NULL DEFAULT TRUE COMMENT '奖励状态，true=启用，false=停用，停用奖励不可领取',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '奖励配置创建时间，自动创建时间戳',
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '奖励配置最后更新时间，自动更新时间戳',
    PRIMARY KEY (`id`),
    UNIQUE KEY `idx_reward_id` (`reward_id`),
    KEY `idx_level` (`level`),
    KEY `idx_is_active` (`is_active`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='邀请奖励配置表，定义邀请好友活动的奖励规则和发放标准';

-- 用户邀请奖励进度表
CREATE TABLE `user_invite_rewards` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '用户奖励进度记录ID，主键',
    `user_id` BIGINT UNSIGNED NOT NULL COMMENT '用户ID，外键关联users表，追踪邀请进度的用户',
    `reward_config_id` BIGINT UNSIGNED NOT NULL COMMENT '奖励配置ID，外键关联invite_reward_configs表',
    `current_progress` INT UNSIGNED NOT NULL DEFAULT 0 COMMENT '当前进度值，用户在当前奖励目标下的实际进度数值',
    `status` ENUM('pending', 'progress', 'completed', 'claimed') NOT NULL DEFAULT 'pending' COMMENT '进度状态：pending未开始/progress进行中/completed已完成/claimed已领取',
    `claimed_at` TIMESTAMP NULL DEFAULT NULL COMMENT '奖励领取时间，null表示未领取',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '进度记录创建时间，自动创建时间戳',
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '进度记录最后更新时间，自动更新时间戳',
    PRIMARY KEY (`id`),
    UNIQUE KEY `idx_user_reward` (`user_id`, `reward_config_id`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_reward_config_id` (`reward_config_id`),
    KEY `idx_status` (`status`),

) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='用户邀请奖励进度表，追踪用户完成邀请奖励目标的进度和状态';

-- ================================
-- 消息相关表
-- ================================

-- 消息配置表
CREATE TABLE `message_configs` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '消息配置ID，主键',
    `type` ENUM('system', 'transaction', 'promotion', 'announcement') NOT NULL COMMENT '消息类型：system系统消息/transaction交易消息/promotion推广消息/announcement公告消息',
    `title` VARCHAR(200) NOT NULL COMMENT '消息标题，消息的主要标题或主题',
    `content_template` TEXT NOT NULL COMMENT '消息内容模板，支持变量替换的消息正文模板',
    `priority` ENUM('low', 'normal', 'high', 'urgent') NOT NULL DEFAULT 'normal' COMMENT '消息优先级：low低/normal普通/high高/urgent紧急，影响显示顺序和推送方式',
    `is_global` BOOLEAN NOT NULL DEFAULT TRUE COMMENT '是否全局消息，true=发送给所有用户，false=指定用户',
    `is_active` BOOLEAN NOT NULL DEFAULT TRUE COMMENT '配置状态，true=启用，false=停用，停用后不再使用此模板',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '消息配置创建时间，自动创建时间戳',
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '消息配置最后更新时间，自动更新时间戳',
    PRIMARY KEY (`id`),
    KEY `idx_type` (`type`),
    KEY `idx_priority` (`priority`),
    KEY `idx_is_active` (`is_active`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='消息配置表，定义各种类型消息的模板、优先级和发送规则';

-- 用户消息表
CREATE TABLE `user_messages` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '用户消息记录ID，主键',
    `user_id` BIGINT UNSIGNED NOT NULL COMMENT '接收消息的用户ID，外键关联users表',
    `message_config_id` BIGINT UNSIGNED NULL COMMENT '消息配置ID，外键关联message_configs表，null表示自定义消息',
    `title` VARCHAR(200) NOT NULL COMMENT '消息标题，显示给用户的消息主题',
    `content` TEXT NOT NULL COMMENT '消息内容，完整的消息正文内容',
    `type` ENUM('system', 'transaction', 'promotion', 'announcement') NOT NULL COMMENT '消息类型：system系统消息/transaction交易消息/promotion推广消息/announcement公告消息',
    `priority` ENUM('low', 'normal', 'high', 'urgent') NOT NULL DEFAULT 'normal' COMMENT '消息优先级：low低/normal普通/high高/urgent紧急',
    `is_read` BOOLEAN NOT NULL DEFAULT FALSE COMMENT '阅读状态，true=已读，false=未读',
    `read_at` TIMESTAMP NULL DEFAULT NULL COMMENT '阅读时间，用户首次阅读消息的时间，null表示未读',
    `actions` JSON NULL COMMENT '操作按钮JSON，包含消息相关的操作按钮，如"查看详情""确认"等',
    `metadata` JSON NULL COMMENT '消息元数据JSON，存储额外的消息相关信息，如链接、图片等',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '消息创建时间，自动创建时间戳',
    PRIMARY KEY (`id`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_message_config_id` (`message_config_id`),
    KEY `idx_type` (`type`),
    KEY `idx_priority` (`priority`),
    KEY `idx_is_read` (`is_read`),
    KEY `idx_created_at` (`created_at`),

) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='用户消息表，存储发送给用户的各种消息和通知';

-- ================================
-- 聊天相关表
-- ================================

-- 聊天会话表
CREATE TABLE `chat_conversations` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '聊天会话ID，主键',
    `user_id` BIGINT UNSIGNED NOT NULL COMMENT '用户ID，外键关联users表，发起聊天的用户',
    `support_agent_id` BIGINT UNSIGNED NULL COMMENT '客服ID，外键关联客服表，null表示未分配客服',
    `title` VARCHAR(200) NULL COMMENT '会话标题，可以是问题描述或客服分配信息',
    `status` ENUM('active', 'closed', 'archived') NOT NULL DEFAULT 'active' COMMENT '会话状态：活跃/已关闭/已归档',
    `last_message_at` TIMESTAMP NULL DEFAULT NULL COMMENT '最后一条消息时间，用于会话排序',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '会话创建时间，自动创建时间戳',
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '会话最后更新时间，自动更新时间戳',
    `closed_at` TIMESTAMP NULL DEFAULT NULL COMMENT '会话关闭时间，null表示未关闭',
    PRIMARY KEY (`id`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_support_agent_id` (`support_agent_id`),
    KEY `idx_status` (`status`),
    KEY `idx_last_message_at` (`last_message_at`),

) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='聊天会话表，管理用户与客服的聊天对话会话';

-- 聊天消息表
CREATE TABLE `chat_messages` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '聊天消息ID，主键',
    `conversation_id` BIGINT UNSIGNED NOT NULL COMMENT '会话ID，外键关联chat_conversations表',
    `sender_id` BIGINT UNSIGNED NOT NULL COMMENT '发送者ID，用户或客服的ID',
    `sender_type` ENUM('user', 'agent', 'system') NOT NULL COMMENT '发送者类型：用户/客服/系统',
    `message_id` VARCHAR(64) NOT NULL UNIQUE COMMENT '消息唯一标识，系统生成的消息ID，唯一索引',
    `content` TEXT NOT NULL COMMENT '消息内容，文本消息的正文或文件消息的描述',
    `message_type` ENUM('text', 'image', 'file', 'system') NOT NULL DEFAULT 'text' COMMENT '消息类型：文本/图片/文件/系统消息',
    `status` ENUM('sending', 'sent', 'delivered', 'read', 'failed') NOT NULL DEFAULT 'sent' COMMENT '消息状态：发送中/已发送/已送达/已读/发送失败',
    `read_at` TIMESTAMP NULL DEFAULT NULL COMMENT '消息阅读时间，null表示未读',
    `file_url` VARCHAR(500) NULL COMMENT '文件URL，用于图片和文件类型消息',
    `file_name` VARCHAR(255) NULL COMMENT '文件名，用于文件类型消息',
    `file_size` BIGINT UNSIGNED NULL COMMENT '文件大小（字节），用于文件类型消息',
    `metadata` JSON NULL COMMENT '消息元数据JSON，存储额外的消息信息，如图片尺寸、文件类型等',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '消息创建时间，自动创建时间戳',
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '消息最后更新时间，自动更新时间戳',
    PRIMARY KEY (`id`),
    UNIQUE KEY `idx_message_id` (`message_id`),
    KEY `idx_conversation_id` (`conversation_id`),
    KEY `idx_sender_id` (`sender_id`),
    KEY `idx_sender_type` (`sender_type`),
    KEY `idx_status` (`status`),
    KEY `idx_created_at` (`created_at`),

) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='聊天消息表，存储用户与客服的聊天消息记录';

-- ================================
-- KYC认证相关表
-- ================================

-- KYC申请表
CREATE TABLE `kyc_applications` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT 'KYC申请记录ID，主键',
    `user_id` BIGINT UNSIGNED NOT NULL COMMENT '申请用户ID，外键关联users表',
    `submission_id` VARCHAR(64) NOT NULL UNIQUE COMMENT '提交唯一标识，系统生成的KYC申请编号，唯一索引',
    `full_name` VARCHAR(100) NOT NULL COMMENT '申请人真实姓名，与身份证件姓名一致',
    `id_number` VARCHAR(50) NOT NULL COMMENT '身份证号码，用于身份验证的核心信息',
    `id_card_front_url` VARCHAR(500) NOT NULL COMMENT '身份证正面照片URL，上传到云存储的访问地址',
    `id_card_back_url` VARCHAR(500) NOT NULL COMMENT '身份证背面照片URL，上传到云存储的访问地址',
    `selfie_url` VARCHAR(500) NULL COMMENT '手持身份证自拍照片URL，用于活体检测，null表示未提供',
    `status` ENUM('submitted', 'pending_review', 'approved', 'rejected') NOT NULL DEFAULT 'submitted' COMMENT 'KYC状态：submitted已提交/pending_review审核中/approved已通过/rejected已拒绝',
    `rejection_reason` TEXT NULL COMMENT '拒绝原因，KYC申请被拒绝时的详细说明，如"照片模糊""信息不符"等',
    `reviewer_id` BIGINT UNSIGNED NULL COMMENT '审核人员ID，外键关联管理员表，null表示未指定审核人',
    `kyc_level` ENUM('level_1', 'level_2', 'level_3') NULL COMMENT 'KYC等级：level_1基础认证/level_2高级认证/level_3企业认证，null表示未确定等级',
    `confidence_score` DECIMAL(5, 4) NULL COMMENT 'AI识别置信度分数，0.0000-1.0000，表示身份证识别的可信度，null表示未进行AI识别',
    `submitted_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '申请提交时间，用户提交KYC材料的时间',
    `reviewed_at` TIMESTAMP NULL DEFAULT NULL COMMENT '审核完成时间，null表示未审核完成',
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '申请记录最后更新时间，自动更新时间戳',
    PRIMARY KEY (`id`),
    UNIQUE KEY `idx_submission_id` (`submission_id`),
    UNIQUE KEY `idx_user_id` (`user_id`),
    KEY `idx_status` (`status`),
    KEY `idx_kyc_level` (`kyc_level`),
    KEY `idx_submitted_at` (`submitted_at`),

) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='KYC申请表，记录用户身份认证申请的详细信息和审核状态';

-- ================================
-- 图表和统计相关表
-- ================================

-- 价格数据表
CREATE TABLE `price_data` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '价格数据记录ID，主键',
    `symbol` VARCHAR(20) NOT NULL COMMENT '交易对符号，如BTC/USDT、ETH/USDT等',
    `timestamp` BIGINT UNSIGNED NOT NULL COMMENT '时间戳，Unix时间戳格式，表示该K线数据的时间点',
    `open_price` DECIMAL(20, 8) NOT NULL COMMENT '开盘价格，8位小数精度，该时间段的起始价格',
    `high_price` DECIMAL(20, 8) NOT NULL COMMENT '最高价格，8位小数精度，该时间段内的最高成交价',
    `low_price` DECIMAL(20, 8) NOT NULL COMMENT '最低价格，8位小数精度，该时间段内的最低成交价',
    `close_price` DECIMAL(20, 8) NOT NULL COMMENT '收盘价格，8位小数精度，该时间段的结束价格',
    `volume` DECIMAL(20, 8) NOT NULL COMMENT '成交量，8位小数精度，该时间段内的交易数量',
    `amount` DECIMAL(20, 8) NOT NULL COMMENT '成交额，8位小数精度，该时间段内的交易总金额',
    `interval` ENUM('1m', '5m', '15m', '30m', '1h', '4h', '1d', '1w') NOT NULL COMMENT '时间间隔：1m分钟/5m五分钟/15m十五分钟/30m半小时/1h小时/4h四小时/1d天/1w周',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '数据创建时间，自动创建时间戳',
    PRIMARY KEY (`id`),
    UNIQUE KEY `idx_symbol_timestamp_interval` (`symbol`, `timestamp`, `interval`),
    KEY `idx_symbol` (`symbol`),
    KEY `idx_timestamp` (`timestamp`),
    KEY `idx_interval` (`interval`),
    KEY `idx_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='价格数据表，存储各种加密货币的K线价格数据，用于图表展示和技术分析';

-- 实时价格表
CREATE TABLE `real_time_prices` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '实时价格记录ID，主键',
    `symbol` VARCHAR(20) NOT NULL UNIQUE COMMENT '交易对符号，如BTC/USDT、ETH/USDT等，唯一索引',
    `current_price` DECIMAL(20, 8) NOT NULL COMMENT '当前价格，8位小数精度，最新的市场价格',
    `price_change` DECIMAL(20, 8) NOT NULL DEFAULT 0.00000000 COMMENT '价格变动，8位小数精度，相比24小时前的绝对价格变化',
    `price_change_percent` DECIMAL(10, 4) NOT NULL DEFAULT 0.0000 COMMENT '价格变动百分比，4位小数精度，相比24小时前的相对价格变化',
    `high_24h` DECIMAL(20, 8) NOT NULL COMMENT '24小时最高价，8位小数精度，过去24小时内的最高成交价',
    `low_24h` DECIMAL(20, 8) NOT NULL COMMENT '24小时最低价，8位小数精度，过去24小时内的最低成交价',
    `volume_24h` DECIMAL(20, 8) NOT NULL COMMENT '24小时成交量，8位小数精度，过去24小时内的总交易量',
    `market_cap` DECIMAL(20, 8) NULL COMMENT '市值，8位小数精度，当前价格×流通供应量，null表示未知',
    `circulating_supply` DECIMAL(20, 8) NULL COMMENT '流通供应量，8位小数精度，市场上流通的代币总量，null表示未知',
    `last_updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '价格最后更新时间，自动更新时间戳',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '记录创建时间，自动创建时间戳',
    PRIMARY KEY (`id`),
    UNIQUE KEY `idx_symbol` (`symbol`),
    KEY `idx_last_updated_at` (`last_updated_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='实时价格表，存储各种加密货币的实时市场数据和24小时统计信息';

-- 首页统计数据表
CREATE TABLE `home_statistics` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '首页统计记录ID，主键',
    `daily_yield` DECIMAL(10, 8) NOT NULL DEFAULT 0.00000000 COMMENT '日收益率，8位小数精度，系统整体的每日收益百分比',
    `total_yield` DECIMAL(20, 8) NOT NULL DEFAULT 0.00000000 COMMENT '总收益，8位小数精度，系统历史累计总收益',
    `today_progress` VARCHAR(20) NOT NULL DEFAULT '0%' COMMENT '今日进度，如"85%"，表示当日任务完成进度',
    `active_users` BIGINT UNSIGNED NOT NULL DEFAULT 0 COMMENT '活跃用户数，当前在线或近期活跃的用户数量',
    `total_users` BIGINT UNSIGNED NOT NULL DEFAULT 0 COMMENT '总用户数，系统注册用户总数',
    `total_airdrop` BIGINT UNSIGNED NOT NULL DEFAULT 0 COMMENT '空投总数，历史发放的空投奖励总次数',
    `success_rate` VARCHAR(10) NOT NULL DEFAULT '0.0%' COMMENT '成功率，如"85.6%"，系统整体操作成功率',
    `system_status` ENUM('healthy', 'maintenance', 'degraded', 'unhealthy') NOT NULL DEFAULT 'healthy' COMMENT '系统状态：healthy正常/maintenance维护中/degraded降级/unhealthy异常',
    `recorded_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '统计记录时间，数据采集的时间点',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '记录创建时间，自动创建时间戳',
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '记录最后更新时间，自动更新时间戳',
    PRIMARY KEY (`id`),
    KEY `idx_system_status` (`system_status`),
    KEY `idx_recorded_at` (`recorded_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='首页统计数据表，存储用于首页展示的关键业务指标和系统状态';

-- ================================
-- 活动和优惠相关表
-- ================================

-- 限时优惠套餐表
CREATE TABLE `promotion_packages` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '推广套餐ID，主键，自增整数，唯一标识每个推广套餐',
    `name` VARCHAR(200) NOT NULL COMMENT '套餐名称，推广套餐的显示名称，如"限时特惠套餐"、"新用户专享"等，用于UI展示和用户识别',
    `price` DECIMAL(20, 8) NOT NULL COMMENT '套餐售价，推广套餐的优惠价格，支持8位小数精度，用于用户购买时的实际支付金额',
    `original_price` DECIMAL(20, 8) NULL COMMENT '套餐原价，推广套餐的原始价格，用于显示优惠折扣幅度，null表示无原价对比',
    `currency` VARCHAR(20) NOT NULL DEFAULT 'USDT' COMMENT '计价货币单位，默认USDT，支持多种加密货币计价，如"USDT"、"BTC"、"ETH"等',
    `description` TEXT NOT NULL COMMENT '套餐详细描述，推广套餐的详细说明文字，包含套餐内容、使用规则、有效期等详细信息',
    `profit_percentage` DECIMAL(10, 4) NOT NULL COMMENT '预期收益率百分比，套餐的年化或预期收益率，支持4位小数，如15.5000表示15.5%收益率',
    `duration_days` INT UNSIGNED NOT NULL COMMENT '有效期天数，推广套餐的有效期限，从购买日开始计算的天数，如30、60、90天等',
    `features` JSON NULL COMMENT '套餐特性列表，JSON格式存储套餐的特色功能和服务内容，如["专属客服"、"优先提现"、"手续费减免"]等',
    `start_time` TIMESTAMP NOT NULL COMMENT '推广开始时间，套餐开始生效的时间戳，精确到秒，用于控制推广活动的时间范围',
    `end_time` TIMESTAMP NOT NULL COMMENT '推广结束时间，套餐推广活动的结束时间戳，过期后自动下架，用于限时优惠控制',
    `stock` INT UNSIGNED NOT NULL COMMENT '套餐库存数量，该推广套餐的总库存限制，0表示无限制，用于控制促销数量',
    `sold` INT UNSIGNED NOT NULL DEFAULT 0 COMMENT '已售数量，该推广套餐已经被购买的数量，实时更新，用于库存管理和销售统计',
    `is_available` BOOLEAN NOT NULL DEFAULT TRUE COMMENT '是否可用状态，true表示套餐当前可购买，false表示已售罄或已下架，影响前端显示',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间，记录推广套餐的创建时间戳，由数据库自动生成',
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间，记录套餐信息的最后更新时间戳，每次修改时自动更新',
    PRIMARY KEY (`id`),
    KEY `idx_start_end_time` (`start_time`, `end_time`) COMMENT '开始结束时间复合索引，用于按时效范围查询推广套餐',
    KEY `idx_is_available` (`is_available`) COMMENT '可用状态索引，用于快速筛选当前可购买的套餐',
    KEY `idx_created_at` (`created_at`) COMMENT '创建时间索引，用于按创建时间排序和查询推广套餐'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='限时优惠套餐表';

-- 轮播图表
CREATE TABLE `carousel_items` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '轮播图记录ID，主键',
    `title` VARCHAR(200) NOT NULL COMMENT '轮播图标题，显示在图片上的主标题',
    `subtitle` VARCHAR(500) NULL COMMENT '副标题，显示在主标题下方的补充说明，null表示无副标题',
    `description` TEXT NULL COMMENT '详细描述，轮播图活动的详细说明，null表示无额外描述',
    `image_url` VARCHAR(500) NOT NULL COMMENT '轮播图片URL，上传到云存储的图片访问地址',
    `action_url` VARCHAR(500) NULL COMMENT '点击跳转链接，用户点击轮播图后跳转的URL，null表示无跳转',
    `action_type` ENUM('url', 'page', 'none') NOT NULL DEFAULT 'none' COMMENT '点击行为：url外部链接/page内部页面/none无行为',
    `order` INT UNSIGNED NOT NULL DEFAULT 0 COMMENT '显示顺序，数字越小排序越靠前，用于控制轮播图展示顺序',
    `is_active` BOOLEAN NOT NULL DEFAULT TRUE COMMENT '启用状态，true=显示，false=隐藏，false状态的轮播图不展示',
    `start_time` TIMESTAMP NULL DEFAULT NULL COMMENT '生效开始时间，null表示立即生效',
    `end_time` TIMESTAMP NULL DEFAULT NULL COMMENT '失效结束时间，null表示永久有效，到达此时间后自动隐藏',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '轮播图创建时间，自动创建时间戳',
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '轮播图最后更新时间，自动更新时间戳',
    PRIMARY KEY (`id`),
    KEY `idx_order` (`order`),
    KEY `idx_is_active` (`is_active`),
    KEY `idx_start_end_time` (`start_time`, `end_time`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='轮播图表，管理首页轮播图的内容、显示时间和用户交互行为';

-- ================================
-- 系统配置和日志表
-- ================================

-- 系统配置表
CREATE TABLE `system_configs` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '系统配置记录ID，主键',
    `config_key` VARCHAR(100) NOT NULL UNIQUE COMMENT '配置键名，系统配置的唯一标识符，如"max_login_attempts"',
    `config_value` TEXT NOT NULL COMMENT '配置值，具体的配置参数值，根据config_type进行类型解析',
    `config_type` ENUM('string', 'number', 'boolean', 'json') NOT NULL DEFAULT 'string' COMMENT '配置类型：string字符串/number数字/boolean布尔值/json对象',
    `description` TEXT NULL COMMENT '配置描述，详细说明该配置的用途和含义，null表示无说明',
    `is_encrypted` BOOLEAN NOT NULL DEFAULT FALSE COMMENT '是否加密，true=敏感信息需要加密存储，false=明文存储',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '配置创建时间，自动创建时间戳',
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '配置最后更新时间，自动更新时间戳',
    PRIMARY KEY (`id`),
    UNIQUE KEY `idx_config_key` (`config_key`),
    KEY `idx_config_type` (`config_type`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='系统配置表，存储系统的各种配置参数和运行时设置';

-- 操作日志表
CREATE TABLE `operation_logs` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '操作日志记录ID，主键',
    `user_id` BIGINT UNSIGNED NULL COMMENT '用户ID，外键关联users表，null表示系统操作',
    `action` VARCHAR(100) NOT NULL COMMENT '操作动作，如"user_register"、"login"、"purchase"等',
    `resource_type` VARCHAR(50) NOT NULL COMMENT '资源类型，操作的资源类型，如"user"、"order"、"transaction"等',
    `resource_id` VARCHAR(100) NULL COMMENT '资源ID，操作的具体资源标识，null表示无特定资源',
    `ip_address` VARCHAR(45) NULL COMMENT 'IP地址，用户操作的IP地址，支持IPv4和IPv6格式',
    `user_agent` TEXT NULL COMMENT '用户代理字符串，包含浏览器、操作系统等客户端信息',
    `request_data` JSON NULL COMMENT '请求数据JSON，操作的请求参数，null表示无请求数据',
    `response_data` JSON NULL COMMENT '响应数据JSON，操作返回的数据，null表示无响应数据',
    `status` ENUM('success', 'failed', 'error') NOT NULL COMMENT '操作状态：success成功/failed失败/error错误',
    `error_message` TEXT NULL COMMENT '错误信息，操作失败时的详细错误说明，null表示无错误',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '日志记录时间，操作发生的时间点',
    PRIMARY KEY (`id`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_action` (`action`),
    KEY `idx_resource_type` (`resource_type`),
    KEY `idx_status` (`status`),
    KEY `idx_created_at` (`created_at`),

) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='操作日志表，记录用户和系统的操作行为，用于审计和问题排查';

-- 错误日志表
CREATE TABLE `error_logs` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '错误日志记录ID，主键',
    `user_id` BIGINT UNSIGNED NULL COMMENT '用户ID，外键关联users表，null表示系统错误',
    `error_type` VARCHAR(100) NOT NULL COMMENT '错误类型，如"ValidationError"、"DatabaseError"、"APICallError"等',
    `error_code` VARCHAR(50) NULL COMMENT '错误代码，系统内部错误标识码，null表示无错误代码',
    `error_message` TEXT NOT NULL COMMENT '错误消息，用户友好的错误描述信息',
    `stack_trace` TEXT NULL COMMENT '堆栈跟踪，详细的错误调用栈信息，用于调试，null表示无堆栈信息',
    `request_data` JSON NULL COMMENT '请求数据JSON，发生错误时的请求参数，null表示无请求数据',
    `ip_address` VARCHAR(45) NULL COMMENT 'IP地址，发生错误时的用户IP地址，支持IPv4和IPv6格式',
    `user_agent` TEXT NULL COMMENT '用户代理字符串，发生错误时的客户端环境信息，null表示无代理信息',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '错误记录时间，错误发生的时间点',
    PRIMARY KEY (`id`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_error_type` (`error_type`),
    KEY `idx_error_code` (`error_code`),
    KEY `idx_created_at` (`created_at`),

) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='错误日志表，记录系统运行中发生的各种错误，用于问题诊断和系统监控';

-- ================================
-- 新手福利表
-- ================================

-- 新手福利表
CREATE TABLE `new_user_benefits` (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '新手福利记录ID，主键',
    `user_id` BIGINT UNSIGNED NOT NULL COMMENT '用户ID，外键关联users表，每个用户只有一条记录',
    `amount` DECIMAL(20, 8) NOT NULL COMMENT '福利金额，8位小数精度，新用户可领取的体验金金额',
    `currency` VARCHAR(20) NOT NULL DEFAULT 'USDT' COMMENT '福利货币类型，默认USDT',
    `description` TEXT NOT NULL COMMENT '福利描述，说明新手福利的使用规则和条件',
    `is_claimed` BOOLEAN NOT NULL DEFAULT FALSE COMMENT '领取状态，true=已领取，false=未领取',
    `expire_at` TIMESTAMP NOT NULL COMMENT '过期时间，福利失效的时间点，过期后不可领取',
    `claimed_at` TIMESTAMP NULL DEFAULT NULL COMMENT '领取时间，用户实际领取福利的时间，null表示未领取',
    `bonus_id` VARCHAR(64) NULL COMMENT '福利唯一标识，系统生成的福利编号，null表示未生成',
    `transaction_id` VARCHAR(64) NULL COMMENT '关联交易ID，发放福利时的交易记录ID，null表示未发放',
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '福利记录创建时间，用户注册时自动创建',
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '福利记录最后更新时间，自动更新时间戳',
    PRIMARY KEY (`id`),
    UNIQUE KEY `idx_user_id` (`user_id`),
    KEY `idx_is_claimed` (`is_claimed`),
    KEY `idx_expire_at` (`expire_at`),

) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='新手福利表，管理新用户注册专享体验金的领取和使用';

-- ================================
-- 初始化数据
-- ================================

-- 初始化用户等级数据
INSERT INTO `user_levels` (`id`, `name`, `description`, `required_invites`, `reward_multiplier`, `is_active`) VALUES
(0, 'LV.0', '初始等级', 0, 1.0000, TRUE),
(1, 'LV.1', '基础等级', 5, 1.1000, TRUE),
(2, 'LV.2', '进阶等级', 10, 1.2000, TRUE),
(3, 'LV.3', '专业等级', 20, 1.4000, TRUE),
(4, 'LV.4', '高级等级', 50, 1.6000, TRUE),
(5, 'LV.5', '大师等级', 100, 1.8000, TRUE);

-- 初始化安全问题数据
INSERT INTO `system_configs` (`config_key`, `config_value`, `config_type`, `description`) VALUES
('security_questions', '[{"id": 1, "question": "你的宠物叫什么名字？"}, {"id": 2, "question": "你的出生城市是哪里？"}, {"id": 3, "question": "你的第一所学校名字？"}, {"id": 4, "question": "你最喜欢的颜色？"}, {"id": 5, "question": "你的母亲姓名？"}]', 'json', '安全问题列表');

-- 初始化系统配置
INSERT INTO `system_configs` (`config_key`, `config_value`, `config_type`, `description`) VALUES
('max_login_attempts', '5', 'number', '最大登录尝试次数'),
('account_lock_duration', '1800', 'number', '账户锁定时间（秒）'),
('token_expire_time', '86400', 'number', 'Token过期时间（秒）'),
('min_password_length', '8', 'number', '最小密码长度'),
('withdraw_min_amount', '10.0', 'number', '最小提现金额（USDT）'),
('exchange_fee_rate', '0.005', 'number', '兑换手续费率');

-- 初始化邀请奖励配置
INSERT INTO `invite_reward_configs` (`reward_id`, `title`, `description`, `level`, `required_progress`, `reward_amount`, `reward_type`) VALUES
('level_1', '一级好友奖励', '邀请1位好友注册并完成实名认证', 1, 1, 100.00000000, 'points'),
('level_2', '二级好友奖励', '累计邀请5位好友注册并完成实名认证', 2, 5, 500.00000000, 'points'),
('level_3', '三级好友奖励', '累计邀请10位好友注册并完成实名认证', 3, 10, 1200.00000000, 'points'),
('level_4', '四级好友奖励', '累计邀请25位好友注册并完成实名认证', 4, 25, 3500.00000000, 'points');

-- 初始化轮播图数据
INSERT INTO `carousel_items` (`title`, `subtitle`, `image_url`, `action_type`, `order`, `is_active`) VALUES
('advanced_mining.title', 'advanced_mining.subtitle', 'assets/images/png/carousel-1.png', 'page', 1, TRUE),
('smart_investing.title', 'smart_investing.subtitle', 'assets/images/png/carousel-2.png', 'page', 2, TRUE),
('global_network.title', 'global_network.subtitle', 'assets/images/png/carousel-3.png', 'page', 3, TRUE);

-- 初始化首页统计数据
INSERT INTO `home_statistics` (`daily_yield`, `total_yield`, `today_progress`, `active_users`, `total_users`, `total_airdrop`, `success_rate`, `system_status`) VALUES
(0.01100000, 2108.00000000, '0%', 15420, 256890, 8921, '85.6', 'healthy');

-- 初始化节点统计数据
INSERT INTO `node_statistics` (`computing_power`, `status_message`, `total_nodes`, `active_nodes`, `earnings`, `is_active`, `network_status`, `utilization_rate`, `average_response_time`) VALUES
('2000K', 'Please stay tuned', 1250, 1180, '$2.5K', TRUE, 'healthy', 0.8560, 0.0450);

-- 初始化实时价格数据
INSERT INTO `real_time_prices` (`symbol`, `current_price`, `price_change`, `price_change_percent`, `high_24h`, `low_24h`, `volume_24h`) VALUES
('BTC/USDT', 45200.00000000, 200.00000000, 0.4400, 45500.00000000, 44800.00000000, 1000000.00000000),
('ETH/USDT', 3200.00000000, 50.00000000, 1.5900, 3250.00000000, 3150.00000000, 2000000.00000000);

-- 初始化支持的区块链类型数据（用于资产中心）
INSERT INTO `system_configs` (`config_key`, `config_value`, `config_type`, `description`) VALUES
('supported_blockchains', '[{"code": "TRC20", "name": "TRC20 (TRON)", "fee": 1.0, "minAmount": 10.0, "maxAmount": 100000.0, "icon": "tron", "confirmationTime": "5分钟", "isAvailable": true}, {"code": "ERC20", "name": "ERC20 (Ethereum)", "fee": 5.0, "minAmount": 10.0, "maxAmount": 100000.0, "icon": "ethereum", "confirmationTime": "15分钟", "isAvailable": false}]', 'json', '支持的区块链类型');

-- 初始化支持的货币对数据（用于图表）
INSERT INTO `system_configs` (`config_key`, `config_value`, `config_type`, `description`) VALUES
('supported_currencies', '[{"symbol": "BTC/USDT", "baseCurrency": "BTC", "quoteCurrency": "USDT", "precision": 2, "supportedTimeRanges": ["1m", "5m", "15m", "30m", "1h", "4h", "1d", "1w"], "isActive": true, "minTradeAmount": 0.0001, "maxTradeAmount": 100}, {"symbol": "ETH/USDT", "baseCurrency": "ETH", "quoteCurrency": "USDT", "precision": 2, "supportedTimeRanges": ["1m", "5m", "15m", "30m", "1h", "4h", "1d", "1w"], "isActive": true, "minTradeAmount": 0.001, "maxTradeAmount": 1000}]', 'json', '支持的货币对');

-- 初始化关于我们信息
INSERT INTO `system_configs` (`config_key`, `config_value`, `config_type`, `description`) VALUES
('about_us_info', '{"email": "support@astrai.com", "phone": "400-123-4567", "website": "www.astrai.com", "address": "新加坡科技园", "version": "1.0.0", "versionTag": "Latest", "copyright": "© 2025 Astra Ai. All rights reserved.", "disclaimer": "投资有风险，入市需谨慎", "appDescription": "Astra Ai是一款基于AI技术的算力挖矿应用，为用户提供稳定高效的数字货币挖矿服务。"}', 'json', '关于我们信息');

-- 初始化算力包数据
INSERT INTO `power_packages` (`name`, `task_type`, `required_level`, `earnings_percent`, `amount`, `currency`, `duration_days`, `description`, `features`, `daily_earnings`, `total_earnings`, `is_active`, `sort_order`) VALUES
('基础算力包', 'AI智能计算', 0, 0.5000, 100.00000000, 'USDT', 30, '适合新手入门的基础算力套餐，提供稳定的AI算力挖矿收益', '{"features": ["入门级AI算力", "24小时稳定收益", "手机APP监控", "专业技术支持"]}', 0.50000000, 15.00000000, TRUE, 1),
('进阶算力包', 'AI智能计算', 1, 0.6000, 500.00000000, 'USDT', 30, '适合有一定经验的用户，提供更高算力和收益回报', '{"features": ["进阶AI算力", "更高收益比例", "优先客服支持", "实时数据监控", "风险控制"]}', 3.00000000, 90.00000000, TRUE, 2),
('专业算力包', 'AI智能计算', 2, 0.7500, 1000.00000000, 'USDT', 90, '专业级算力套餐，为专业投资者提供高性能AI算力服务', '{"features": ["专业级AI算力", "专属客户经理", "定制化服务", "高级数据分析", "VIP特权"]}', 7.50000000, 675.00000000, TRUE, 3),
('高级算力包', '数据处理', 3, 0.9000, 5000.00000000, 'USDT', 180, '高级算力套餐，为机构和高净值用户提供顶级算力服务', '{"features": ["顶级AI算力", "1对1专属服务", "定制化解决方案", "投资建议", "优先技术升级", "月度收益报告"]}', 45.00000000, 8100.00000000, TRUE, 4),
('大师算力包', 'AI智能计算', 4, 1.2000, 10000.00000000, 'USDT', 365, '大师级算力套餐，提供业界领先的AI算力和专业投资服务', '{"features": ["旗舰级AI算力", "白金专属服务", "定制投资策略", "现场技术支持", "季度收益分红", "新功能优先体验"]}', 120.00000000, 43800.00000000, TRUE, 5),
('钻石算力包', 'AI智能计算', 5, 1.5000, 50000.00000000, 'USDT', 365, '钻石级算力套餐，专为顶级投资者定制的尊贵算力服务', '{"features": ["钻石级顶级算力", "私人管家服务", "独家算力资源", "定制化开发", "全球技术支持", "年度收益分红", "股权激励机会"]}', 750.00000000, 273750.00000000, TRUE, 6);

-- ================================
-- 创建视图
-- ================================

-- 用户资产统计视图
CREATE VIEW `user_asset_summary` AS
SELECT
    u.id as user_id,
    u.username,
    u.user_level,
    COALESCE(SUM(CASE WHEN ua.currency = 'USDT' THEN ua.balance ELSE 0 END), 0) as usdt_balance,
    COALESCE(SUM(CASE WHEN ua.currency = 'DG' THEN ua.balance ELSE 0 END), 0) as dg_balance,
    COALESCE(SUM(ua.total_earned), 0) as total_earned,
    u.total_assets,
    u.is_kyc_verified,
    u.created_at as registration_date
FROM users u
LEFT JOIN user_assets ua ON u.id = ua.user_id
GROUP BY u.id;

-- 用户邀请统计视图
CREATE VIEW `user_invite_summary` AS
SELECT
    u.id as user_id,
    u.username,
    COUNT(ir.id) as total_invites,
    COUNT(CASE WHEN ir.status = 'registered' THEN 1 END) as registered_invites,
    COUNT(CASE WHEN ir.status = 'completed' THEN 1 END) as completed_invites,
    COALESCE(SUM(ir.total_reward), 0) as total_rewards,
    COUNT(CASE WHEN ir.invite_level = 1 THEN 1 END) as direct_invites,
    COUNT(CASE WHEN ir.invite_level > 1 THEN 1 END) as indirect_invites
FROM users u
LEFT JOIN invite_records ir ON u.id = ir.inviter_id
GROUP BY u.id;

-- 用户收益统计视图
CREATE VIEW `user_earnings_summary` AS
SELECT
    u.id as user_id,
    u.username,
    COUNT(CASE WHEN t.type = 'mining_earning' THEN 1 END) as mining_count,
    COALESCE(SUM(CASE WHEN t.type = 'mining_earning' AND t.status = 'completed' THEN t.amount ELSE 0 END), 0) as mining_earnings,
    COUNT(CASE WHEN t.type = 'referral' THEN 1 END) as referral_count,
    COALESCE(SUM(CASE WHEN t.type = 'referral' AND t.status = 'completed' THEN t.amount ELSE 0 END), 0) as referral_earnings,
    COUNT(CASE WHEN t.type = 'task_earning' THEN 1 END) as task_count,
    COALESCE(SUM(CASE WHEN t.type = 'task_earning' AND t.status = 'completed' THEN t.amount ELSE 0 END), 0) as task_earnings,
    COUNT(CASE WHEN t.type = 'airdrop' THEN 1 END) as airdrop_count,
    COALESCE(SUM(CASE WHEN t.type = 'airdrop' AND t.status = 'completed' THEN t.amount ELSE 0 END), 0) as airdrop_earnings,
    DATE(t.created_at) as earning_date
FROM users u
LEFT JOIN transactions t ON u.id = t.user_id
GROUP BY u.id, DATE(t.created_at);

-- 系统统计视图
CREATE VIEW `system_statistics` AS
SELECT
    (SELECT COUNT(*) FROM users WHERE is_active = TRUE) as active_users,
    (SELECT COUNT(*) FROM users) as total_users,
    (SELECT COUNT(*) FROM users WHERE is_kyc_verified = TRUE) as kyc_verified_users,
    (SELECT COUNT(*) FROM airdrop_participations WHERE status = 'success') as total_airdrop_claims,
    (SELECT COUNT(*) FROM orders WHERE status = 'completed') as total_orders,
    (SELECT COALESCE(SUM(amount), 0) FROM transactions WHERE status = 'completed') as total_transaction_volume,
    (SELECT COUNT(*) FROM chat_conversations WHERE status = 'active') as active_chats,
    (SELECT COUNT(*) FROM withdrawal_requests WHERE status = 'pending') as pending_withdrawals;

-- ================================
-- 创建存储过程
-- ================================

DELIMITER //

-- 创建用户存储过程
CREATE PROCEDURE `CreateUser`(
    IN p_username VARCHAR(50),
    IN p_email VARCHAR(255),
    IN p_password_hash VARCHAR(255),
    IN p_salt VARCHAR(255),
    IN p_invite_code VARCHAR(20),
    IN p_inviter_id BIGINT UNSIGNED
)
BEGIN
    DECLARE v_user_id BIGINT UNSIGNED;

    INSERT INTO users (
        username, email, password_hash, salt,
        invite_code, inviter_id, created_at
    ) VALUES (
        p_username, p_email, p_password_hash, p_salt,
        p_invite_code, p_inviter_id, NOW()
    );

    SET v_user_id = LAST_INSERT_ID();

    -- 为用户创建默认资产账户
    INSERT INTO user_assets (user_id, currency) VALUES
        (v_user_id, 'USDT'),
        (v_user_id, 'DG');

    -- 为用户创建新手福利
    INSERT INTO new_user_benefits (
        user_id, amount, currency, description,
        expire_at, created_at
    ) VALUES (
        v_user_id, 100.00000000, 'USDT',
        '新用户注册专享体验金',
        DATE_ADD(NOW(), INTERVAL 7 DAY), NOW()
    );

    SELECT v_user_id as user_id;
END //

-- 处理空投参与存储过程
CREATE PROCEDURE `ClaimAirdrop`(
    IN p_user_id BIGINT UNSIGNED,
    IN p_airdrop_config_id BIGINT UNSIGNED,
    IN p_current_round INT UNSIGNED,
    OUT p_result BOOLEAN,
    OUT p_dg_amount DECIMAL(20, 8),
    OUT p_message VARCHAR(500)
)
BEGIN
    DECLARE v_participation_count INT DEFAULT 0;
    DECLARE v_airdrop_available BOOLEAN DEFAULT FALSE;
    DECLARE v_success_rate DECIMAL(5, 4) DEFAULT 0;
    DECLARE v_dg_reward DECIMAL(20, 8) DEFAULT 0;

    -- 检查空投活动是否可用
    SELECT COUNT(*) INTO v_participation_count
    FROM airdrop_participations ap
    JOIN airdrop_rounds ar ON ap.airdrop_round_id = ar.id
    WHERE ap.user_id = p_user_id
    AND ap.airdrop_config_id = p_airdrop_config_id
    AND ar.round_number = p_current_round;

    IF v_participation_count > 0 THEN
        SET p_result = FALSE;
        SET p_message = '您已参与过本轮空投';
    ELSE
        -- 模拟空投成功率和随机奖励
        SELECT success_rate INTO v_success_rate
        FROM airdrop_configs
        WHERE id = p_airdrop_config_id;

        SET v_dg_reward = ROUND(RAND() * 200 + 50, 8); -- 50-250 DG随机奖励
        SET p_dg_amount = v_dg_reward;

        IF RAND() <= v_success_rate THEN
            SET p_result = TRUE;
            SET p_message = '恭喜成功抢到空投！';

            -- 记录空投参与记录
            INSERT INTO airdrop_participations (
                user_id, airdrop_config_id, status, dg_amount,
                round, claimed_at, created_at
            ) VALUES (
                p_user_id, p_airdrop_config_id, 'success',
                v_dg_reward, p_current_round, NOW(), NOW()
            );

            -- 增加用户DG余额
            UPDATE user_assets
            SET balance = balance + v_dg_reward
            WHERE user_id = p_user_id AND currency = 'DG';

            -- 记录交易
            INSERT INTO transactions (
                user_id, transaction_id, type, amount,
                status, description, created_at
            ) VALUES (
                p_user_id, CONCAT('airdrop_', UNIX_TIMESTAMP()),
                'airdrop', v_dg_reward, 'completed',
                CONCAT('空投奖励 ', v_dg_reward, ' DG'), NOW()
            );
        ELSE
            SET p_result = FALSE;
            SET p_message = '很遗憾，本次未抢到空投，请下次再试';

            -- 记录空投参与记录
            INSERT INTO airdrop_participations (
                user_id, airdrop_config_id, status,
                round, failure_reason, created_at
            ) VALUES (
                p_user_id, p_airdrop_config_id, 'failed',
                p_current_round, '随机抽取未中签', NOW()
            );
        END IF;
    END IF;
END //

-- 处理订单支付存储过程
CREATE PROCEDURE `ProcessOrderPayment`(
    IN p_order_id BIGINT UNSIGNED,
    IN p_transaction_hash VARCHAR(128),
    OUT p_result BOOLEAN,
    OUT p_message VARCHAR(500)
)
BEGIN
    DECLARE v_user_id BIGINT UNSIGNED;
    DECLARE v_power_package_id BIGINT UNSIGNED;
    DECLARE v_quantity INT UNSIGNED;
    DECLARE v_amount DECIMAL(20, 8);
    DECLARE v_order_status VARCHAR(20);

    -- 获取订单信息
    SELECT user_id, power_package_id, quantity, amount, status
    INTO v_user_id, v_power_package_id, v_quantity, v_amount, v_order_status
    FROM orders
    WHERE id = p_order_id;

    IF v_order_status != 'pending' THEN
        SET p_result = FALSE;
        SET p_message = '订单状态不正确';
    ELSE
        -- 更新订单状态
        UPDATE orders
        SET status = 'paid',
            transaction_hash = p_transaction_hash,
            is_paid = TRUE,
            paid_at = NOW(),
            updated_at = NOW()
        WHERE id = p_order_id;

        -- 创建用户算力记录
        INSERT INTO user_power_records (
            user_id, power_package_id, order_id,
            type, amount, start_time, end_time,
            created_at
        ) VALUES (
            v_user_id, v_power_package_id, p_order_id,
            (SELECT task_type FROM power_packages WHERE id = v_power_package_id),
            v_amount, NOW(), DATE_ADD(NOW(), INTERVAL 30 DAY), NOW()
        );

        -- 记录交易
        INSERT INTO transactions (
            user_id, transaction_id, type, from_currency,
            amount, status, order_id, created_at
        ) VALUES (
            v_user_id, CONCAT('purchase_', UNIX_TIMESTAMP()),
            'purchase', 'USDT', v_amount, 'completed',
            p_order_id, NOW()
        );

        SET p_result = TRUE;
        SET p_message = '订单支付成功';
    END IF;
END //

DELIMITER ;

-- ================================
-- 创建触发器
-- ================================

-- 用户注册后创建邀请记录的触发器
DELIMITER //
CREATE TRIGGER `after_user_registration`
AFTER INSERT ON `users`
FOR EACH ROW
BEGIN
    -- 如果有邀请人，创建邀请记录
    IF NEW.inviter_id IS NOT NULL THEN
        INSERT INTO invite_records (
            inviter_id, invitee_id, invite_code,
            status, invite_level, created_at
        ) VALUES (
            NEW.inviter_id, NEW.id, NEW.invite_code,
            'registered', 1, NOW()
        );

        -- 更新邀请人的邀请奖励进度
        INSERT IGNORE INTO user_invite_rewards (user_id, reward_config_id, current_progress, status)
        SELECT NEW.inviter_id, id, 1, 'progress'
        FROM invite_reward_configs
        WHERE required_progress = 1;

        UPDATE user_invite_rewards uir
        JOIN invite_reward_configs irc ON uir.reward_config_id = irc.id
        SET uir.current_progress = uir.current_progress + 1,
            uir.updated_at = NOW()
        WHERE uir.user_id = NEW.inviter_id
        AND uir.reward_config_id = irc.id
        AND irc.required_progress = 1;
    END IF;
END //
DELIMITER ;

-- 更新算力包库存的触发器
DELIMITER //
CREATE TRIGGER `after_order_creation`
AFTER INSERT ON `orders`
FOR EACH ROW
BEGIN
    -- 如果是优惠套餐订单，减少库存
    IF EXISTS (
        SELECT 1 FROM promotion_packages pp
        WHERE pp.id = NEW.power_package_id
    ) THEN
        UPDATE promotion_packages
        SET sold = sold + NEW.quantity,
            updated_at = NOW()
        WHERE id = NEW.power_package_id;
    END IF;
END //
DELIMITER ;

-- 记录用户操作的触发器
DELIMITER //
CREATE TRIGGER `log_user_operations`
AFTER INSERT ON `users`
FOR EACH ROW
BEGIN
    INSERT INTO operation_logs (
        user_id, action, resource_type, resource_id,
        status, created_at
    ) VALUES (
        NEW.id, 'user_register', 'user', NEW.id,
        'success', NOW()
    );
END //
DELIMITER ;

-- ================================
-- 创建事件（定时任务）
-- ================================

-- 清理过期会话的事件
CREATE EVENT `clean_expired_sessions`
ON SCHEDULE EVERY 1 HOUR
DO
DELETE FROM user_sessions
WHERE expires_at < NOW() OR is_active = FALSE;

-- 清理旧日志的事件
CREATE EVENT `clean_old_logs`
ON SCHEDULE EVERY 1 DAY
DO
BEGIN
    DELETE FROM operation_logs WHERE created_at < DATE_SUB(NOW(), INTERVAL 90 DAY);
    DELETE FROM error_logs WHERE created_at < DATE_SUB(NOW(), INTERVAL 90 DAY);
END;

-- 更新统计数据的事件
CREATE EVENT `update_home_statistics`
ON SCHEDULE EVERY 5 MINUTE
DO
UPDATE home_statistics
SET
    daily_yield = (SELECT ROUND(RAND() * 0.02, 8)),
    total_yield = total_yield + (SELECT ROUND(RAND() * 10, 2)),
    active_users = (SELECT COUNT(*) FROM users WHERE is_active = TRUE),
    success_rate = CONCAT(ROUND(85 + RAND() * 10, 1), '%'),
    updated_at = NOW();

-- 清理过期数据的事件
CREATE EVENT `clean_expired_data`
ON SCHEDULE EVERY 1 DAY
DO
BEGIN
    -- 删除过期的价格数据
    DELETE FROM price_data WHERE created_at < DATE_SUB(NOW(), INTERVAL 1 YEAR);

    -- 删除过期的轮播图
    UPDATE carousel_items
    SET is_active = FALSE
    WHERE end_time < NOW();

    -- 取消过期订单
    UPDATE orders
    SET status = 'expired', updated_at = NOW()
    WHERE status = 'pending'
    AND expired_at < NOW();
END;

-- ================================
-- 数据库版本信息
-- ================================

-- 创建版本表
CREATE TABLE `database_version` (
    `id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
    `version` VARCHAR(20) NOT NULL,
    `description` TEXT NULL,
    `applied_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='数据库版本表';

-- 插入版本信息
INSERT INTO `database_version` (`version`, `description`) VALUES
('1.0.0', 'Coin项目数据库初始版本，包含完整的用户、资产、交易、算力、空投、任务、邀请、聊天等所有功能模块');

-- ================================
-- 数据库性能优化建议
-- ================================

/*
1. 定期维护：
   - 定期执行 OPTIMIZE TABLE 优化表结构
   - 定期执行 ANALYZE TABLE 更新统计信息
   - 定期清理历史数据，保持表的合理大小

2. 索引优化：
   - 根据查询模式调整索引
   - 监控慢查询日志
   - 使用复合索引优化多列查询

3. 分区策略：
   - 对于大表（如交易记录、价格数据）可以考虑按时间分区
   - 提高查询性能和数据管理效率

4. 读写分离：
   - 考虑使用主从复制，读写分离
   - 读操作可以使用从库，写操作使用主库

5. 缓存策略：
   - 对于热点数据使用 Redis 等缓存
   - 减少数据库压力，提高响应速度

6. 备份策略：
   - 定期全量备份
   - 实时增量备份
   - 异地备份保证数据安全

7. 监控告警：
   - 监控数据库性能指标
   - 设置合理的告警阈值
   - 及时发现和解决问题
*/

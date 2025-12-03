# ************************************************************
# Sequel Ace SQL dump
# 版本号： 20095
#
# https://sequel-ace.com/
# https://github.com/Sequel-Ace/Sequel-Ace
#
# 主机: localhost (MySQL 9.3.0)
# 数据库: coin_dgai
# 生成时间: 2025-11-30 11:42:18 +0000
# ************************************************************


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
SET NAMES utf8mb4;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE='NO_AUTO_VALUE_ON_ZERO', SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;


# 转储表 _sqlx_migrations
# ------------------------------------------------------------

CREATE TABLE `_sqlx_migrations` (
  `version` bigint NOT NULL,
  `description` text COLLATE utf8mb4_bin NOT NULL,
  `installed_on` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `success` tinyint(1) NOT NULL,
  `checksum` blob NOT NULL,
  `execution_time` bigint NOT NULL,
  PRIMARY KEY (`version`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin;



# 转储表 airdrop_configs
# ------------------------------------------------------------

CREATE TABLE `airdrop_configs` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '空投活动配置ID，主键',
  `type` varchar(50) COLLATE utf8mb4_bin NOT NULL COMMENT '空投类型：daily每日空投/vip会员空投/newbie新手空投/limited限时空投/public公开奖/premium高级算力空投',
  `title` varchar(200) COLLATE utf8mb4_bin NOT NULL COMMENT '空投活动标题',
  `subtitle` varchar(500) COLLATE utf8mb4_bin DEFAULT NULL COMMENT '空投活动副标题或说明',
  `description` text COLLATE utf8mb4_bin NOT NULL COMMENT '空投活动详细描述，包含规则和奖励说明',
  `activity_start_time` timestamp NOT NULL COMMENT '活动开始时间',
  `activity_end_time` timestamp NOT NULL COMMENT '活动结束时间',
  `total_rounds` int unsigned NOT NULL COMMENT '总轮次数，如999轮表示有999次抢空投机会',
  `round_duration` int unsigned NOT NULL COMMENT '每轮持续时间（秒），如60秒每轮',
  `interval_duration` int unsigned NOT NULL COMMENT '轮次间隔时间（秒），如20秒间隔',
  `participation_type` enum('allUsers','memberLevel','newUsers','powerHolders') COLLATE utf8mb4_bin NOT NULL COMMENT '参与条件：所有用户/指定等级/新用户/算力持有者',
  `required_level` tinyint unsigned DEFAULT NULL COMMENT '参与所需的最低用户等级，仅当participation_type=memberLevel时有效',
  `min_power_amount` decimal(20,8) DEFAULT NULL COMMENT '参与所需的最小算力金额，仅当participation_type=powerHolders时有效',
  `color` varchar(20) COLLATE utf8mb4_bin DEFAULT NULL COMMENT '活动主题色，用于前端显示，十六进制颜色值',
  `status` enum('active','inactive','expired') COLLATE utf8mb4_bin NOT NULL DEFAULT 'active' COMMENT '活动状态：激活/停用/已过期',
  `participant_count` bigint unsigned NOT NULL DEFAULT '0' COMMENT '累计参与人数',
  `success_rate` decimal(5,4) DEFAULT NULL COMMENT '成功率，如0.856表示85.6%的成功率',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '空投配置创建时间，自动创建时间戳',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '空投配置最后更新时间，自动更新时间戳',
  PRIMARY KEY (`id`),
  KEY `idx_type` (`type`),
  KEY `idx_participation_type` (`participation_type`),
  KEY `idx_status` (`status`),
  KEY `idx_activity_time` (`activity_start_time`,`activity_end_time`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='空投活动配置表，定义各种空投活动的规则和参数';



# 转储表 airdrop_participations
# ------------------------------------------------------------

CREATE TABLE `airdrop_participations` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '空投参与记录ID，主键',
  `user_id` bigint unsigned NOT NULL COMMENT '参与用户ID，外键关联users表',
  `airdrop_config_id` bigint unsigned NOT NULL COMMENT '空投活动配置ID，外键关联airdrop_configs表',
  `airdrop_round_id` bigint unsigned DEFAULT NULL COMMENT '空投轮次ID，外键关联airdrop_rounds表，null表示未关联具体轮次',
  `transaction_id` varchar(64) COLLATE utf8mb4_bin DEFAULT NULL COMMENT '关联交易ID，用于追踪空投奖励发放的交易记录',
  `status` enum('success','failed','pending') COLLATE utf8mb4_bin NOT NULL DEFAULT 'pending' COMMENT '参与状态：success成功/failed失败/pending处理中',
  `dg_amount` decimal(20,8) DEFAULT NULL COMMENT '获得的DG代币数量，8位小数精度，成功抢到空投时填写',
  `round` int unsigned DEFAULT NULL COMMENT '参与轮次编号，标识用户参与的是第几轮空投',
  `failure_reason` text COLLATE utf8mb4_bin COMMENT '失败原因，空投抢购失败时的详细说明，如"已参与过本轮""未中签"等',
  `claimed_at` timestamp NULL DEFAULT NULL COMMENT '领取时间，用户成功领取空投奖励的时间，null表示未领取',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '参与记录创建时间，自动创建时间戳',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '参与记录最后更新时间，自动更新时间戳',
  PRIMARY KEY (`id`),
  KEY `idx_user_id` (`user_id`),
  KEY `idx_airdrop_config_id` (`airdrop_config_id`),
  KEY `idx_airdrop_round_id` (`airdrop_round_id`),
  KEY `idx_status` (`status`),
  KEY `idx_claimed_at` (`claimed_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='空投参与记录表，记录用户参与空投活动的详细情况和奖励发放状态';



# 转储表 airdrop_rounds
# ------------------------------------------------------------

CREATE TABLE `airdrop_rounds` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '空投轮次ID，主键',
  `airdrop_config_id` bigint unsigned NOT NULL COMMENT '空投活动配置ID，外键关联airdrop_configs表',
  `round_number` int unsigned NOT NULL COMMENT '轮次编号，从1开始的连续数字，表示当前是第几轮空投',
  `start_time` timestamp NOT NULL COMMENT '轮次开始时间，用户可以开始参与抢空投的时间',
  `end_time` timestamp NOT NULL COMMENT '轮次结束时间，本轮次空投抢购结束时间',
  `total_dg_amount` decimal(20,8) NOT NULL COMMENT '本轮次DG代币总奖励金额，8位小数精度',
  `participant_count` int unsigned NOT NULL DEFAULT '0' COMMENT '参与人数，本轮次参与抢空投的总用户数',
  `success_count` int unsigned NOT NULL DEFAULT '0' COMMENT '成功人数，本轮次成功抢到空投的用户数',
  `status` enum('pending','active','completed') COLLATE utf8mb4_bin NOT NULL DEFAULT 'pending' COMMENT '轮次状态：pending待开始/active进行中/completed已完成',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '轮次创建时间，自动创建时间戳',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '轮次最后更新时间，自动更新时间戳',
  PRIMARY KEY (`id`),
  UNIQUE KEY `idx_config_round` (`airdrop_config_id`,`round_number`),
  KEY `idx_airdrop_config_id` (`airdrop_config_id`),
  KEY `idx_status` (`status`),
  KEY `idx_start_time` (`start_time`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='空投轮次表，记录空投活动的每一轮次详细信息和统计数据';



# 转储表 carousel_items
# ------------------------------------------------------------

CREATE TABLE `carousel_items` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '轮播图记录ID，主键',
  `title` varchar(200) COLLATE utf8mb4_bin NOT NULL COMMENT '轮播图标题，显示在图片上的主标题',
  `subtitle` varchar(500) COLLATE utf8mb4_bin DEFAULT NULL COMMENT '副标题，显示在主标题下方的补充说明，null表示无副标题',
  `description` text COLLATE utf8mb4_bin COMMENT '详细描述，轮播图活动的详细说明，null表示无额外描述',
  `image_url` varchar(500) COLLATE utf8mb4_bin NOT NULL COMMENT '轮播图片URL，上传到云存储的图片访问地址',
  `action_url` varchar(500) COLLATE utf8mb4_bin DEFAULT NULL COMMENT '点击跳转链接，用户点击轮播图后跳转的URL，null表示无跳转',
  `action_type` enum('url','page','none') COLLATE utf8mb4_bin NOT NULL DEFAULT 'none' COMMENT '点击行为：url外部链接/page内部页面/none无行为',
  `order` int unsigned NOT NULL DEFAULT '0' COMMENT '显示顺序，数字越小排序越靠前，用于控制轮播图展示顺序',
  `is_active` tinyint(1) NOT NULL DEFAULT '1' COMMENT '启用状态，true=显示，false=隐藏，false状态的轮播图不展示',
  `start_time` timestamp NULL DEFAULT NULL COMMENT '生效开始时间，null表示立即生效',
  `end_time` timestamp NULL DEFAULT NULL COMMENT '失效结束时间，null表示永久有效，到达此时间后自动隐藏',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '轮播图创建时间，自动创建时间戳',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '轮播图最后更新时间，自动更新时间戳',
  PRIMARY KEY (`id`),
  KEY `idx_order` (`order`),
  KEY `idx_is_active` (`is_active`),
  KEY `idx_start_end_time` (`start_time`,`end_time`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='轮播图表，管理首页轮播图的内容、显示时间和用户交互行为';



# 转储表 chat_conversations
# ------------------------------------------------------------

CREATE TABLE `chat_conversations` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '聊天会话ID，主键',
  `user_id` bigint unsigned NOT NULL COMMENT '用户ID，外键关联users表，发起聊天的用户',
  `support_agent_id` bigint unsigned DEFAULT NULL COMMENT '客服ID，外键关联客服表，null表示未分配客服',
  `title` varchar(200) COLLATE utf8mb4_bin DEFAULT NULL COMMENT '会话标题，可以是问题描述或客服分配信息',
  `status` enum('active','closed','archived') COLLATE utf8mb4_bin NOT NULL DEFAULT 'active' COMMENT '会话状态：活跃/已关闭/已归档',
  `last_message_at` timestamp NULL DEFAULT NULL COMMENT '最后一条消息时间，用于会话排序',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '会话创建时间，自动创建时间戳',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '会话最后更新时间，自动更新时间戳',
  `closed_at` timestamp NULL DEFAULT NULL COMMENT '会话关闭时间，null表示未关闭',
  PRIMARY KEY (`id`),
  KEY `idx_user_id` (`user_id`),
  KEY `idx_support_agent_id` (`support_agent_id`),
  KEY `idx_status` (`status`),
  KEY `idx_last_message_at` (`last_message_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='聊天会话表，管理用户与客服的聊天对话会话';



# 转储表 chat_messages
# ------------------------------------------------------------

CREATE TABLE `chat_messages` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '聊天消息ID，主键',
  `conversation_id` bigint unsigned NOT NULL COMMENT '会话ID，外键关联chat_conversations表',
  `sender_id` bigint unsigned NOT NULL COMMENT '发送者ID，用户或客服的ID',
  `sender_type` enum('user','agent','system') COLLATE utf8mb4_bin NOT NULL COMMENT '发送者类型：用户/客服/系统',
  `message_id` varchar(64) COLLATE utf8mb4_bin NOT NULL COMMENT '消息唯一标识，系统生成的消息ID，唯一索引',
  `content` text COLLATE utf8mb4_bin NOT NULL COMMENT '消息内容，文本消息的正文或文件消息的描述',
  `message_type` enum('text','image','file','system') COLLATE utf8mb4_bin NOT NULL DEFAULT 'text' COMMENT '消息类型：文本/图片/文件/系统消息',
  `status` enum('sending','sent','delivered','read','failed') COLLATE utf8mb4_bin NOT NULL DEFAULT 'sent' COMMENT '消息状态：发送中/已发送/已送达/已读/发送失败',
  `read_at` timestamp NULL DEFAULT NULL COMMENT '消息阅读时间，null表示未读',
  `file_url` varchar(500) COLLATE utf8mb4_bin DEFAULT NULL COMMENT '文件URL，用于图片和文件类型消息',
  `file_name` varchar(255) COLLATE utf8mb4_bin DEFAULT NULL COMMENT '文件名，用于文件类型消息',
  `file_size` bigint unsigned DEFAULT NULL COMMENT '文件大小（字节），用于文件类型消息',
  `metadata` json DEFAULT NULL COMMENT '消息元数据JSON，存储额外的消息信息，如图片尺寸、文件类型等',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '消息创建时间，自动创建时间戳',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '消息最后更新时间，自动更新时间戳',
  PRIMARY KEY (`id`),
  UNIQUE KEY `message_id` (`message_id`),
  UNIQUE KEY `idx_message_id` (`message_id`),
  KEY `idx_conversation_id` (`conversation_id`),
  KEY `idx_sender_id` (`sender_id`),
  KEY `idx_sender_type` (`sender_type`),
  KEY `idx_status` (`status`),
  KEY `idx_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='聊天消息表，存储用户与客服的聊天消息记录';



# 转储表 error_logs
# ------------------------------------------------------------

CREATE TABLE `error_logs` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '错误日志记录ID，主键',
  `user_id` bigint unsigned DEFAULT NULL COMMENT '用户ID，外键关联users表，null表示系统错误',
  `error_type` varchar(100) COLLATE utf8mb4_bin NOT NULL COMMENT '错误类型，如"ValidationError"、"DatabaseError"、"APICallError"等',
  `error_code` varchar(50) COLLATE utf8mb4_bin DEFAULT NULL COMMENT '错误代码，系统内部错误标识码，null表示无错误代码',
  `error_message` text COLLATE utf8mb4_bin NOT NULL COMMENT '错误消息，用户友好的错误描述信息',
  `stack_trace` text COLLATE utf8mb4_bin COMMENT '堆栈跟踪，详细的错误调用栈信息，用于调试，null表示无堆栈信息',
  `request_data` json DEFAULT NULL COMMENT '请求数据JSON，发生错误时的请求参数，null表示无请求数据',
  `ip_address` varchar(45) COLLATE utf8mb4_bin DEFAULT NULL COMMENT 'IP地址，发生错误时的用户IP地址，支持IPv4和IPv6格式',
  `user_agent` text COLLATE utf8mb4_bin COMMENT '用户代理字符串，发生错误时的客户端环境信息，null表示无代理信息',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '错误记录时间，错误发生的时间点',
  PRIMARY KEY (`id`),
  KEY `idx_user_id` (`user_id`),
  KEY `idx_error_type` (`error_type`),
  KEY `idx_error_code` (`error_code`),
  KEY `idx_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='错误日志表，记录系统运行中发生的各种错误，用于问题诊断和系统监控';



# 转储表 home_statistics
# ------------------------------------------------------------

CREATE TABLE `home_statistics` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '首页统计记录ID，主键',
  `daily_yield` decimal(10,8) NOT NULL DEFAULT '0.00000000' COMMENT '日收益率，8位小数精度，系统整体的每日收益百分比',
  `total_yield` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT '总收益，8位小数精度，系统历史累计总收益',
  `today_progress` varchar(20) COLLATE utf8mb4_bin NOT NULL DEFAULT '0%' COMMENT '今日进度，如"85%"，表示当日任务完成进度',
  `active_users` bigint unsigned NOT NULL DEFAULT '0' COMMENT '活跃用户数，当前在线或近期活跃的用户数量',
  `total_users` bigint unsigned NOT NULL DEFAULT '0' COMMENT '总用户数，系统注册用户总数',
  `total_airdrop` bigint unsigned NOT NULL DEFAULT '0' COMMENT '空投总数，历史发放的空投奖励总次数',
  `success_rate` varchar(10) COLLATE utf8mb4_bin NOT NULL DEFAULT '0.0%' COMMENT '成功率，如"85.6%"，系统整体操作成功率',
  `system_status` enum('healthy','maintenance','degraded','unhealthy') COLLATE utf8mb4_bin NOT NULL DEFAULT 'healthy' COMMENT '系统状态：healthy正常/maintenance维护中/degraded降级/unhealthy异常',
  `recorded_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '统计记录时间，数据采集的时间点',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '记录创建时间，自动创建时间戳',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '记录最后更新时间，自动更新时间戳',
  PRIMARY KEY (`id`),
  KEY `idx_system_status` (`system_status`),
  KEY `idx_recorded_at` (`recorded_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='首页统计数据表，存储用于首页展示的关键业务指标和系统状态';



# 转储表 invite_records
# ------------------------------------------------------------

CREATE TABLE `invite_records` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '邀请记录ID，主键',
  `inviter_id` bigint unsigned NOT NULL COMMENT '邀请人用户ID，外键关联users表，发出邀请的用户',
  `invitee_id` bigint unsigned NOT NULL COMMENT '被邀请人用户ID，外键关联users表，接受邀请的用户',
  `invite_code` varchar(20) COLLATE utf8mb4_bin NOT NULL COMMENT '使用的邀请码，与邀请人的invite_code对应',
  `status` enum('pending','registered','completed') COLLATE utf8mb4_bin NOT NULL DEFAULT 'pending' COMMENT '邀请状态：pending待注册/registered已注册/completed已完成（完成KYC或其他条件）',
  `invite_level` tinyint unsigned NOT NULL DEFAULT '1' COMMENT '邀请层级，1表示直接邀请，2表示二级邀请，依此类推',
  `join_date` timestamp NULL DEFAULT NULL COMMENT '被邀请人注册时间，null表示未注册',
  `direct_reward` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT '直接邀请奖励，邀请人从被邀请人获得的直接奖励，8位小数精度',
  `indirect_reward` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT '间接邀请奖励，从下级邀请中获得的间接奖励，8位小数精度',
  `total_reward` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT '总奖励金额，直接奖励+间接奖励的总和，8位小数精度',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '邀请记录创建时间，自动创建时间戳',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '邀请记录最后更新时间，自动更新时间戳',
  PRIMARY KEY (`id`),
  KEY `idx_inviter_id` (`inviter_id`),
  KEY `idx_invitee_id` (`invitee_id`),
  KEY `idx_invite_code` (`invite_code`),
  KEY `idx_status` (`status`),
  KEY `idx_invite_level` (`invite_level`),
  KEY `idx_join_date` (`join_date`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='邀请记录表，记录用户邀请关系、奖励发放和邀请状态跟踪';



# 转储表 invite_reward_configs
# ------------------------------------------------------------

CREATE TABLE `invite_reward_configs` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '邀请奖励配置ID，主键',
  `reward_id` varchar(50) COLLATE utf8mb4_bin NOT NULL COMMENT '奖励唯一标识，用于系统内部引用和追踪',
  `title` varchar(200) COLLATE utf8mb4_bin NOT NULL COMMENT '奖励标题，用户友好的奖励名称，如"邀请1位好友奖励"',
  `description` text COLLATE utf8mb4_bin NOT NULL COMMENT '奖励描述，详细说明获得奖励的条件和奖励内容',
  `level` tinyint unsigned NOT NULL COMMENT '奖励等级，对应邀请等级或里程碑，如1级、2级等',
  `required_progress` int unsigned NOT NULL COMMENT '所需进度值，达到此数值可获得奖励，如邀请人数、任务完成数等',
  `reward_amount` decimal(20,8) NOT NULL COMMENT '奖励数量，发放给用户的奖励金额或积分数量，8位小数精度',
  `reward_type` enum('points','dg','usdt') COLLATE utf8mb4_bin NOT NULL COMMENT '奖励类型：points积分/dg代币/usdt稳定币',
  `is_active` tinyint(1) NOT NULL DEFAULT '1' COMMENT '奖励状态，true=启用，false=停用，停用奖励不可领取',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '奖励配置创建时间，自动创建时间戳',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '奖励配置最后更新时间，自动更新时间戳',
  PRIMARY KEY (`id`),
  UNIQUE KEY `reward_id` (`reward_id`),
  UNIQUE KEY `idx_reward_id` (`reward_id`),
  KEY `idx_level` (`level`),
  KEY `idx_is_active` (`is_active`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='邀请奖励配置表，定义邀请好友活动的奖励规则和发放标准';



# 转储表 kyc_applications
# ------------------------------------------------------------

CREATE TABLE `kyc_applications` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT 'KYC申请记录ID，主键',
  `user_id` bigint unsigned NOT NULL COMMENT '申请用户ID，外键关联users表',
  `submission_id` varchar(64) COLLATE utf8mb4_bin NOT NULL COMMENT '提交唯一标识，系统生成的KYC申请编号，唯一索引',
  `full_name` varchar(100) COLLATE utf8mb4_bin NOT NULL COMMENT '申请人真实姓名，与身份证件姓名一致',
  `id_number` varchar(50) COLLATE utf8mb4_bin NOT NULL COMMENT '身份证号码，用于身份验证的核心信息',
  `id_card_front_url` varchar(500) COLLATE utf8mb4_bin NOT NULL COMMENT '身份证正面照片URL，上传到云存储的访问地址',
  `id_card_back_url` varchar(500) COLLATE utf8mb4_bin NOT NULL COMMENT '身份证背面照片URL，上传到云存储的访问地址',
  `selfie_url` varchar(500) COLLATE utf8mb4_bin DEFAULT NULL COMMENT '手持身份证自拍照片URL，用于活体检测，null表示未提供',
  `status` enum('submitted','pending_review','approved','rejected') COLLATE utf8mb4_bin NOT NULL DEFAULT 'submitted' COMMENT 'KYC状态：submitted已提交/pending_review审核中/approved已通过/rejected已拒绝',
  `rejection_reason` text COLLATE utf8mb4_bin COMMENT '拒绝原因，KYC申请被拒绝时的详细说明，如"照片模糊""信息不符"等',
  `reviewer_id` bigint unsigned DEFAULT NULL COMMENT '审核人员ID，外键关联管理员表，null表示未指定审核人',
  `kyc_level` enum('level_1','level_2','level_3') COLLATE utf8mb4_bin DEFAULT NULL COMMENT 'KYC等级：level_1基础认证/level_2高级认证/level_3企业认证，null表示未确定等级',
  `confidence_score` decimal(5,4) DEFAULT NULL COMMENT 'AI识别置信度分数，0.0000-1.0000，表示身份证识别的可信度，null表示未进行AI识别',
  `submitted_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '申请提交时间，用户提交KYC材料的时间',
  `reviewed_at` timestamp NULL DEFAULT NULL COMMENT '审核完成时间，null表示未审核完成',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '申请记录最后更新时间，自动更新时间戳',
  PRIMARY KEY (`id`),
  UNIQUE KEY `submission_id` (`submission_id`),
  UNIQUE KEY `idx_submission_id` (`submission_id`),
  UNIQUE KEY `idx_user_id` (`user_id`),
  KEY `idx_status` (`status`),
  KEY `idx_kyc_level` (`kyc_level`),
  KEY `idx_submitted_at` (`submitted_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='KYC申请表，记录用户身份认证申请的详细信息和审核状态';



# 转储表 message_configs
# ------------------------------------------------------------

CREATE TABLE `message_configs` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '消息配置ID，主键',
  `type` enum('system','transaction','promotion','announcement') COLLATE utf8mb4_bin NOT NULL COMMENT '消息类型：system系统消息/transaction交易消息/promotion推广消息/announcement公告消息',
  `title` varchar(200) COLLATE utf8mb4_bin NOT NULL COMMENT '消息标题，消息的主要标题或主题',
  `content_template` text COLLATE utf8mb4_bin NOT NULL COMMENT '消息内容模板，支持变量替换的消息正文模板',
  `priority` enum('low','normal','high','urgent') COLLATE utf8mb4_bin NOT NULL DEFAULT 'normal' COMMENT '消息优先级：low低/normal普通/high高/urgent紧急，影响显示顺序和推送方式',
  `is_global` tinyint(1) NOT NULL DEFAULT '1' COMMENT '是否全局消息，true=发送给所有用户，false=指定用户',
  `is_active` tinyint(1) NOT NULL DEFAULT '1' COMMENT '配置状态，true=启用，false=停用，停用后不再使用此模板',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '消息配置创建时间，自动创建时间戳',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '消息配置最后更新时间，自动更新时间戳',
  PRIMARY KEY (`id`),
  KEY `idx_type` (`type`),
  KEY `idx_priority` (`priority`),
  KEY `idx_is_active` (`is_active`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='消息配置表，定义各种类型消息的模板、优先级和发送规则';



# 转储表 new_user_benefits
# ------------------------------------------------------------

CREATE TABLE `new_user_benefits` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '新手福利记录ID，主键',
  `user_id` bigint unsigned NOT NULL COMMENT '用户ID，外键关联users表，每个用户只有一条记录',
  `amount` decimal(20,8) NOT NULL COMMENT '福利金额，8位小数精度，新用户可领取的体验金金额',
  `currency` varchar(20) COLLATE utf8mb4_bin NOT NULL DEFAULT 'USDT' COMMENT '福利货币类型，默认USDT',
  `description` text COLLATE utf8mb4_bin NOT NULL COMMENT '福利描述，说明新手福利的使用规则和条件',
  `is_claimed` tinyint(1) NOT NULL DEFAULT '0' COMMENT '领取状态，true=已领取，false=未领取',
  `expire_at` timestamp NOT NULL COMMENT '过期时间，福利失效的时间点，过期后不可领取',
  `claimed_at` timestamp NULL DEFAULT NULL COMMENT '领取时间，用户实际领取福利的时间，null表示未领取',
  `bonus_id` varchar(64) COLLATE utf8mb4_bin DEFAULT NULL COMMENT '福利唯一标识，系统生成的福利编号，null表示未生成',
  `transaction_id` varchar(64) COLLATE utf8mb4_bin DEFAULT NULL COMMENT '关联交易ID，发放福利时的交易记录ID，null表示未发放',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '福利记录创建时间，用户注册时自动创建',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '福利记录最后更新时间，自动更新时间戳',
  PRIMARY KEY (`id`),
  UNIQUE KEY `idx_user_id` (`user_id`),
  KEY `idx_is_claimed` (`is_claimed`),
  KEY `idx_expire_at` (`expire_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='新手福利表，管理新用户注册专享体验金的领取和使用';



# 转储表 operation_logs
# ------------------------------------------------------------

CREATE TABLE `operation_logs` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '操作日志记录ID，主键',
  `user_id` bigint unsigned DEFAULT NULL COMMENT '用户ID，外键关联users表，null表示系统操作',
  `action` varchar(100) COLLATE utf8mb4_bin NOT NULL COMMENT '操作动作，如"user_register"、"login"、"purchase"等',
  `resource_type` varchar(50) COLLATE utf8mb4_bin NOT NULL COMMENT '资源类型，操作的资源类型，如"user"、"order"、"transaction"等',
  `resource_id` varchar(100) COLLATE utf8mb4_bin DEFAULT NULL COMMENT '资源ID，操作的具体资源标识，null表示无特定资源',
  `ip_address` varchar(45) COLLATE utf8mb4_bin DEFAULT NULL COMMENT 'IP地址，用户操作的IP地址，支持IPv4和IPv6格式',
  `user_agent` text COLLATE utf8mb4_bin COMMENT '用户代理字符串，包含浏览器、操作系统等客户端信息',
  `request_data` json DEFAULT NULL COMMENT '请求数据JSON，操作的请求参数，null表示无请求数据',
  `response_data` json DEFAULT NULL COMMENT '响应数据JSON，操作返回的数据，null表示无响应数据',
  `status` enum('success','failed','error') COLLATE utf8mb4_bin NOT NULL COMMENT '操作状态：success成功/failed失败/error错误',
  `error_message` text COLLATE utf8mb4_bin COMMENT '错误信息，操作失败时的详细错误说明，null表示无错误',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '日志记录时间，操作发生的时间点',
  PRIMARY KEY (`id`),
  KEY `idx_user_id` (`user_id`),
  KEY `idx_action` (`action`),
  KEY `idx_resource_type` (`resource_type`),
  KEY `idx_status` (`status`),
  KEY `idx_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='操作日志表，记录用户和系统的操作行为，用于审计和问题排查';



# 转储表 orders
# ------------------------------------------------------------

CREATE TABLE `orders` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '订单记录ID，主键',
  `order_id` varchar(64) COLLATE utf8mb4_bin NOT NULL COMMENT '订单唯一标识，系统生成的订单ID，用于API查询和验证',
  `user_id` bigint unsigned NOT NULL COMMENT '下单用户ID，外键关联users表',
  `power_package_id` bigint unsigned NOT NULL COMMENT '算力包产品ID，外键关联power_packages表',
  `quantity` int unsigned NOT NULL DEFAULT '1' COMMENT '购买数量，默认为1份，支持批量购买',
  `amount` decimal(20,8) NOT NULL COMMENT '订单总金额，8位小数精度，amount = quantity × 算力包单价',
  `asset_pay` decimal(20,8) unsigned NOT NULL,
  `coin_pay` decimal(20,8) unsigned NOT NULL,
  `blockchain_type` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL COMMENT '区块链网络类型，如TRC20、ERC20、BEP20等，用于区分不同网络支付',
  `blockchain_address` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL COMMENT '支付接收地址，用户需要向此地址转账支付订单',
  `transaction_hash` varchar(128) COLLATE utf8mb4_bin DEFAULT NULL COMMENT '区块链交易哈希，用于验证支付完成状态，支付成功后填写',
  `status` tinyint(1) NOT NULL COMMENT '0 pending待支付/1 paid已支付/2 cancelled已取消/3 已升级',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '订单创建时间，自动创建时间戳',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '订单最后更新时间，自动更新时间戳',
  PRIMARY KEY (`id`),
  UNIQUE KEY `order_id` (`order_id`),
  UNIQUE KEY `idx_order_id` (`order_id`),
  KEY `idx_user_id` (`user_id`),
  KEY `idx_power_package_id` (`power_package_id`),
  KEY `idx_status` (`status`),
  KEY `idx_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='订单表，记录用户购买算力包的订单信息，包括支付状态和订单生命周期管理';



# 转储表 password_reset_tokens
# ------------------------------------------------------------

CREATE TABLE `password_reset_tokens` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `username` varchar(50) COLLATE utf8mb4_bin NOT NULL,
  `token_hash` varchar(500) COLLATE utf8mb4_bin NOT NULL,
  `expires_at` datetime NOT NULL,
  `ip_address` varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL,
  `is_used` tinyint(1) NOT NULL DEFAULT '0',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin;



# 转储表 power_packages
# ------------------------------------------------------------

CREATE TABLE `power_packages` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '算力包配置ID，主键',
  `title` json NOT NULL COMMENT '算力包名称，如"基础算力包"、"高级算力包"等',
  `lv` smallint unsigned NOT NULL DEFAULT '0' COMMENT '购买所需的最低用户等级，0-5对应LV.0-LV.5',
  `daily_yield_percentage` decimal(10,4) NOT NULL COMMENT '收益率百分比，表示每日收益的百分比，如0.5表示日收益率0.5%',
  `amount` decimal(20,8) NOT NULL COMMENT '算力包价格，8位小数精度，用户需要支付的费用',
  `description` json NOT NULL COMMENT '算力包详细描述，包含服务内容和收益说明，帮助用户了解产品',
  `status` tinyint(1) NOT NULL COMMENT '算力包状态，true=在售，false=停售',
  `sort_order` int unsigned NOT NULL DEFAULT '0' COMMENT '排序字段，用于前端显示顺序',
  `is_upgrade` tinyint(1) NOT NULL DEFAULT '0' COMMENT '0 不可升级， 1 可升级',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '算力包创建时间，自动创建时间戳',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '算力包最后更新时间，自动更新时间戳',
  PRIMARY KEY (`id`),
  KEY `idx_sort_order` (`sort_order`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='算力包配置表，定义可购买的算力产品规格和价格';



# 转储表 price_data
# ------------------------------------------------------------

CREATE TABLE `price_data` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '价格数据记录ID，主键',
  `symbol` varchar(20) COLLATE utf8mb4_bin NOT NULL COMMENT '交易对符号，如BTC/USDT、ETH/USDT等',
  `timestamp` bigint unsigned NOT NULL COMMENT '时间戳，Unix时间戳格式，表示该K线数据的时间点',
  `open_price` decimal(20,8) NOT NULL COMMENT '开盘价格，8位小数精度，该时间段的起始价格',
  `high_price` decimal(20,8) NOT NULL COMMENT '最高价格，8位小数精度，该时间段内的最高成交价',
  `low_price` decimal(20,8) NOT NULL COMMENT '最低价格，8位小数精度，该时间段内的最低成交价',
  `close_price` decimal(20,8) NOT NULL COMMENT '收盘价格，8位小数精度，该时间段的结束价格',
  `volume` decimal(20,8) NOT NULL COMMENT '成交量，8位小数精度，该时间段内的交易数量',
  `amount` decimal(20,8) NOT NULL COMMENT '成交额，8位小数精度，该时间段内的交易总金额',
  `interval` enum('1m','5m','15m','30m','1h','4h','1d','1w') COLLATE utf8mb4_bin NOT NULL COMMENT '时间间隔：1m分钟/5m五分钟/15m十五分钟/30m半小时/1h小时/4h四小时/1d天/1w周',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '数据创建时间，自动创建时间戳',
  PRIMARY KEY (`id`),
  UNIQUE KEY `idx_symbol_timestamp_interval` (`symbol`,`timestamp`,`interval`),
  KEY `idx_symbol` (`symbol`),
  KEY `idx_timestamp` (`timestamp`),
  KEY `idx_interval` (`interval`),
  KEY `idx_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='价格数据表，存储各种加密货币的K线价格数据，用于图表展示和技术分析';



# 转储表 promotion_packages
# ------------------------------------------------------------

CREATE TABLE `promotion_packages` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '推广套餐ID，主键，自增整数，唯一标识每个推广套餐',
  `name` varchar(200) COLLATE utf8mb4_bin NOT NULL COMMENT '套餐名称，推广套餐的显示名称，如"限时特惠套餐"、"新用户专享"等，用于UI展示和用户识别',
  `price` decimal(20,8) NOT NULL COMMENT '套餐售价，推广套餐的优惠价格，支持8位小数精度，用于用户购买时的实际支付金额',
  `original_price` decimal(20,8) DEFAULT NULL COMMENT '套餐原价，推广套餐的原始价格，用于显示优惠折扣幅度，null表示无原价对比',
  `currency` varchar(20) COLLATE utf8mb4_bin NOT NULL DEFAULT 'USDT' COMMENT '计价货币单位，默认USDT，支持多种加密货币计价，如"USDT"、"BTC"、"ETH"等',
  `description` text COLLATE utf8mb4_bin NOT NULL COMMENT '套餐详细描述，推广套餐的详细说明文字，包含套餐内容、使用规则、有效期等详细信息',
  `profit_percentage` decimal(10,4) NOT NULL COMMENT '预期收益率百分比，套餐的年化或预期收益率，支持4位小数，如15.5000表示15.5%收益率',
  `duration_days` int unsigned NOT NULL COMMENT '有效期天数，推广套餐的有效期限，从购买日开始计算的天数，如30、60、90天等',
  `features` json DEFAULT NULL COMMENT '套餐特性列表，JSON格式存储套餐的特色功能和服务内容，如["专属客服"、"优先提现"、"手续费减免"]等',
  `start_time` timestamp NOT NULL COMMENT '推广开始时间，套餐开始生效的时间戳，精确到秒，用于控制推广活动的时间范围',
  `end_time` timestamp NOT NULL COMMENT '推广结束时间，套餐推广活动的结束时间戳，过期后自动下架，用于限时优惠控制',
  `stock` int unsigned NOT NULL COMMENT '套餐库存数量，该推广套餐的总库存限制，0表示无限制，用于控制促销数量',
  `sold` int unsigned NOT NULL DEFAULT '0' COMMENT '已售数量，该推广套餐已经被购买的数量，实时更新，用于库存管理和销售统计',
  `is_available` tinyint(1) NOT NULL DEFAULT '1' COMMENT '是否可用状态，true表示套餐当前可购买，false表示已售罄或已下架，影响前端显示',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间，记录推广套餐的创建时间戳，由数据库自动生成',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间，记录套餐信息的最后更新时间戳，每次修改时自动更新',
  PRIMARY KEY (`id`),
  KEY `idx_start_end_time` (`start_time`,`end_time`) COMMENT '开始结束时间复合索引，用于按时效范围查询推广套餐',
  KEY `idx_is_available` (`is_available`) COMMENT '可用状态索引，用于快速筛选当前可购买的套餐',
  KEY `idx_created_at` (`created_at`) COMMENT '创建时间索引，用于按创建时间排序和查询推广套餐'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='限时优惠套餐表';



# 转储表 real_time_prices
# ------------------------------------------------------------

CREATE TABLE `real_time_prices` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '实时价格记录ID，主键',
  `symbol` varchar(20) COLLATE utf8mb4_bin NOT NULL COMMENT '交易对符号，如BTC/USDT、ETH/USDT等，唯一索引',
  `current_price` decimal(20,8) NOT NULL COMMENT '当前价格，8位小数精度，最新的市场价格',
  `price_change` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT '价格变动，8位小数精度，相比24小时前的绝对价格变化',
  `price_change_percent` decimal(10,4) NOT NULL DEFAULT '0.0000' COMMENT '价格变动百分比，4位小数精度，相比24小时前的相对价格变化',
  `high_24h` decimal(20,8) NOT NULL COMMENT '24小时最高价，8位小数精度，过去24小时内的最高成交价',
  `low_24h` decimal(20,8) NOT NULL COMMENT '24小时最低价，8位小数精度，过去24小时内的最低成交价',
  `volume_24h` decimal(20,8) NOT NULL COMMENT '24小时成交量，8位小数精度，过去24小时内的总交易量',
  `market_cap` decimal(20,8) DEFAULT NULL COMMENT '市值，8位小数精度，当前价格×流通供应量，null表示未知',
  `circulating_supply` decimal(20,8) DEFAULT NULL COMMENT '流通供应量，8位小数精度，市场上流通的代币总量，null表示未知',
  `last_updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '价格最后更新时间，自动更新时间戳',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '记录创建时间，自动创建时间戳',
  PRIMARY KEY (`id`),
  UNIQUE KEY `symbol` (`symbol`),
  UNIQUE KEY `idx_symbol` (`symbol`),
  KEY `idx_last_updated_at` (`last_updated_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='实时价格表，存储各种加密货币的实时市场数据和24小时统计信息';



# 转储表 security_questions
# ------------------------------------------------------------

CREATE TABLE `security_questions` (
  `id` int unsigned NOT NULL AUTO_INCREMENT,
  `question` varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL,
  `is_active` tinyint(1) NOT NULL,
  `sort_order` smallint NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin;



# 转储表 system_configs
# ------------------------------------------------------------

CREATE TABLE `system_configs` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '系统配置记录ID，主键',
  `config_key` varchar(200) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL COMMENT '配置键名，系统配置的唯一标识符，如"max_login_attempts"',
  `config_value` text COLLATE utf8mb4_bin NOT NULL COMMENT '配置值，具体的配置参数值，根据config_type进行类型解析',
  `description` text COLLATE utf8mb4_bin COMMENT '配置描述，详细说明该配置的用途和含义，null表示无说明',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '配置创建时间，自动创建时间戳',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '配置最后更新时间，自动更新时间戳',
  PRIMARY KEY (`id`),
  UNIQUE KEY `config_key` (`config_key`),
  UNIQUE KEY `idx_config_key` (`config_key`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='系统配置表，存储系统的各种配置参数和运行时设置';



# 转储表 transactions
# ------------------------------------------------------------

CREATE TABLE `transactions` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '交易记录ID，主键',
  `user_id` bigint unsigned NOT NULL COMMENT '用户ID，外键关联users表',
  `transaction_id` varchar(64) COLLATE utf8mb4_bin NOT NULL COMMENT '交易唯一标识，系统生成的交易ID，唯一索引',
  `types` enum('withdraw','exchange','purchase','cancel_purchase','airdrop','referral','mining_earning','welcome') CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL COMMENT '交易类型：提现/兑换/购买/撤消购买/空投/邀请/挖矿收益/新手福利',
  `from_currency` varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL DEFAULT '' COMMENT '源货币代码，兑换交易的转出货币，如BTC、USDT等',
  `to_currency` varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL DEFAULT '' COMMENT '目标货币代码，兑换交易的转入货币，如ETH、DG等',
  `amount` decimal(20,8) NOT NULL COMMENT '交易金额，8位小数精度，交易的主要数值',
  `fee` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT '交易手续费，8位小数精度，平台收取的服务费用',
  `exchange_rate` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT '兑换汇率，用于币种兑换交易，表示两个货币之间的兑换比例',
  `blockchain_type` varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL DEFAULT 'TRC20' COMMENT '区块链类型，如TRC20、ERC20等，用于链上交易',
  `from_address` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL DEFAULT '' COMMENT '转出地址，充值或提现的区块链钱包地址',
  `to_address` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL DEFAULT '' COMMENT '转入地址，充值或提现的区块链钱包地址',
  `description` varchar(2000) CHARACTER SET utf8mb4 COLLATE utf8mb4_bin NOT NULL DEFAULT '' COMMENT '交易描述，用户友好的交易说明',
  `metadata` json DEFAULT NULL COMMENT '交易元数据JSON，存储额外的交易相关信息',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '交易创建时间，自动创建时间戳',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '交易最后更新时间，自动更新时间戳',
  `completed_at` timestamp NULL DEFAULT NULL COMMENT '交易完成时间，null表示未完成',
  `status` enum('pending','processing','completed','failed','cancelled') COLLATE utf8mb4_bin NOT NULL DEFAULT 'pending' COMMENT '''交易状态：pending待处理/processing处理中/completed已完成/failed失败/cancelled已取消'',',
  PRIMARY KEY (`id`),
  UNIQUE KEY `transaction_id` (`transaction_id`),
  UNIQUE KEY `idx_transaction_id` (`transaction_id`),
  KEY `idx_user_id` (`user_id`),
  KEY `idx_type` (`types`),
  KEY `idx_currency` (`from_currency`,`to_currency`),
  KEY `idx_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='交易记录表';



# 转储表 user_assets
# ------------------------------------------------------------

CREATE TABLE `user_assets` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '资产记录ID，主键',
  `user_id` bigint unsigned NOT NULL COMMENT '用户ID，外键关联users表',
  `currency` varchar(20) COLLATE utf8mb4_bin NOT NULL COMMENT '货币类型代码，如USDT、DG、BTC、ETH等',
  `balance` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT '可用余额，8位小数精度，用户可以自由支配的金额',
  `frozen_balance` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT '冻结余额，8位小数精度，如提现中、订单待支付等冻结的金额',
  `total_earned` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT '总收益金额，8位小数精度，历史累计获得的总收益',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '资产账户创建时间，自动创建时间戳',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '资产最后更新时间，自动更新时间戳',
  PRIMARY KEY (`id`),
  UNIQUE KEY `idx_user_currency` (`user_id`,`currency`),
  KEY `idx_user_id` (`user_id`),
  KEY `idx_currency` (`currency`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='用户资产表，存储用户各种数字货币的余额和收益信息';



# 转储表 user_invite_rewards
# ------------------------------------------------------------

CREATE TABLE `user_invite_rewards` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '用户奖励进度记录ID，主键',
  `user_id` bigint unsigned NOT NULL COMMENT '用户ID，外键关联users表，追踪邀请进度的用户',
  `reward_config_id` bigint unsigned NOT NULL COMMENT '奖励配置ID，外键关联invite_reward_configs表',
  `current_progress` int unsigned NOT NULL DEFAULT '0' COMMENT '当前进度值，用户在当前奖励目标下的实际进度数值',
  `status` enum('pending','progress','completed','claimed') COLLATE utf8mb4_bin NOT NULL DEFAULT 'pending' COMMENT '进度状态：pending未开始/progress进行中/completed已完成/claimed已领取',
  `claimed_at` timestamp NULL DEFAULT NULL COMMENT '奖励领取时间，null表示未领取',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '进度记录创建时间，自动创建时间戳',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '进度记录最后更新时间，自动更新时间戳',
  PRIMARY KEY (`id`),
  UNIQUE KEY `idx_user_reward` (`user_id`,`reward_config_id`),
  KEY `idx_user_id` (`user_id`),
  KEY `idx_reward_config_id` (`reward_config_id`),
  KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='用户邀请奖励进度表，追踪用户完成邀请奖励目标的进度和状态';



# 转储表 user_levels
# ------------------------------------------------------------

CREATE TABLE `user_levels` (
  `id` tinyint unsigned NOT NULL AUTO_INCREMENT COMMENT '用户等级ID，主键，0-5对应LV.0-LV.5',
  `name` varchar(50) COLLATE utf8mb4_bin NOT NULL COMMENT '等级名称，如"LV.1"、"LV.2"等',
  `description` text COLLATE utf8mb4_bin COMMENT '等级描述，说明该等级的特权和要求',
  `required_invites` int unsigned NOT NULL DEFAULT '0' COMMENT '升级所需邀请人数，邀请好友达到此数量可升级',
  `reward_multiplier` decimal(10,4) NOT NULL DEFAULT '1.0000' COMMENT '收益倍数，该等级用户的收益乘数，如1.2000表示20%加成',
  `min_power` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT '最低算力要求，达到此算力金额才能维持或升级到该等级',
  `icon_url` varchar(500) COLLATE utf8mb4_bin DEFAULT NULL COMMENT '等级图标URL地址，用于前端显示等级徽章',
  `is_active` tinyint(1) NOT NULL DEFAULT '1' COMMENT '等级状态，true=启用，false=停用，停用等级不可升级',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '等级创建时间，自动创建时间戳',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '等级最后更新时间，自动更新时间戳',
  PRIMARY KEY (`id`),
  KEY `idx_required_invites` (`required_invites`),
  KEY `idx_is_active` (`is_active`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='用户等级表，定义用户等级体系、升级条件和权益奖励';



# 转储表 user_messages
# ------------------------------------------------------------

CREATE TABLE `user_messages` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '用户消息记录ID，主键',
  `user_id` bigint unsigned NOT NULL COMMENT '接收消息的用户ID，外键关联users表',
  `message_config_id` bigint unsigned DEFAULT NULL COMMENT '消息配置ID，外键关联message_configs表，null表示自定义消息',
  `title` varchar(200) COLLATE utf8mb4_bin NOT NULL COMMENT '消息标题，显示给用户的消息主题',
  `content` text COLLATE utf8mb4_bin NOT NULL COMMENT '消息内容，完整的消息正文内容',
  `type` enum('system','transaction','promotion','announcement') COLLATE utf8mb4_bin NOT NULL COMMENT '消息类型：system系统消息/transaction交易消息/promotion推广消息/announcement公告消息',
  `priority` enum('low','normal','high','urgent') COLLATE utf8mb4_bin NOT NULL DEFAULT 'normal' COMMENT '消息优先级：low低/normal普通/high高/urgent紧急',
  `is_read` tinyint(1) NOT NULL DEFAULT '0' COMMENT '阅读状态，true=已读，false=未读',
  `read_at` timestamp NULL DEFAULT NULL COMMENT '阅读时间，用户首次阅读消息的时间，null表示未读',
  `actions` json DEFAULT NULL COMMENT '操作按钮JSON，包含消息相关的操作按钮，如"查看详情""确认"等',
  `metadata` json DEFAULT NULL COMMENT '消息元数据JSON，存储额外的消息相关信息，如链接、图片等',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '消息创建时间，自动创建时间戳',
  PRIMARY KEY (`id`),
  KEY `idx_user_id` (`user_id`),
  KEY `idx_message_config_id` (`message_config_id`),
  KEY `idx_type` (`type`),
  KEY `idx_priority` (`priority`),
  KEY `idx_is_read` (`is_read`),
  KEY `idx_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='用户消息表，存储发送给用户的各种消息和通知';



# 转储表 user_power
# ------------------------------------------------------------

CREATE TABLE `user_power` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '算力记录ID，主键',
  `user_id` bigint unsigned NOT NULL COMMENT '用户ID，外键关联users表',
  `power_package_id` bigint unsigned NOT NULL COMMENT '算力包ID，外键关联power_packages表',
  `lv` smallint NOT NULL DEFAULT '1' COMMENT '算力等级',
  `daily_yield_percentage` decimal(10,4) NOT NULL COMMENT '收益率百分比，表示每日收益的百分比，如0.5表示日收益率0.5%',
  `order_id` varchar(50) COLLATE utf8mb4_bin NOT NULL COMMENT '关联的订单ID，外键关联orders表，null表示非订单购买',
  `types` smallint NOT NULL COMMENT '1 赠送or0 购买',
  `amount` decimal(20,8) NOT NULL COMMENT '算力包价格，8位小数精度',
  `start_time` timestamp NULL DEFAULT NULL COMMENT '算力开始生效时间',
  `status` smallint NOT NULL COMMENT '0 no-pay 1 active, 2 cancelled, 3 upgrade',
  `earnings` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT '累计收益金额，8位小数精度',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '算力记录创建时间，自动创建时间戳',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '算力记录最后更新时间，自动更新时间戳',
  PRIMARY KEY (`id`),
  KEY `idx_user_id` (`user_id`),
  KEY `idx_power_package_id` (`power_package_id`),
  KEY `idx_order_id` (`order_id`),
  KEY `idx_start_time` (`start_time`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='用户算力记录表，记录用户购买的算力包使用情况和收益';



# 转储表 user_power_record
# ------------------------------------------------------------

CREATE TABLE `user_power_record` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `user_id` bigint unsigned NOT NULL,
  `power_package_id` bigint unsigned NOT NULL,
  `user_power_id` bigint NOT NULL DEFAULT '0',
  `lv` tinyint NOT NULL DEFAULT '1',
  `daily_yield_percentage` decimal(10,4) NOT NULL,
  `close_price` decimal(20,8) NOT NULL,
  `package_amount` decimal(20,8) NOT NULL,
  `amount` decimal(20,8) NOT NULL,
  `created_at` date NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `date` (`created_at`,`user_id`,`user_power_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='用户算力记录表，记录用户购买的算力包使用情况和收益';



# 转储表 user_security_questions
# ------------------------------------------------------------

CREATE TABLE `user_security_questions` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '安全问题记录ID，主键',
  `user_id` bigint unsigned NOT NULL COMMENT '用户ID，外键关联users表',
  `question_id` smallint unsigned NOT NULL COMMENT '安全问题ID，对应系统预定义的问题列表',
  `answer_hash` varchar(255) COLLATE utf8mb4_bin NOT NULL COMMENT '问题答案的哈希值，使用bcrypt加密',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '安全问题设置时间，自动创建时间戳',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '安全问题最后更新时间，自动更新时间戳',
  PRIMARY KEY (`id`),
  UNIQUE KEY `idx_user_question` (`user_id`,`question_id`),
  KEY `idx_user_id` (`user_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='用户安全问题表，存储用户设置的密保问题和答案';



# 转储表 user_sessions
# ------------------------------------------------------------

CREATE TABLE `user_sessions` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '会话记录ID，主键',
  `user_id` bigint unsigned NOT NULL COMMENT '用户ID，外键关联users表',
  `token` varchar(512) COLLATE utf8mb4_bin NOT NULL COMMENT 'JWT会话令牌，用于API认证，唯一索引',
  `device_info` json DEFAULT NULL COMMENT '设备信息JSON，包含设备类型、操作系统、设备型号等',
  `ip_address` varchar(45) COLLATE utf8mb4_bin DEFAULT NULL COMMENT '用户登录IP地址，支持IPv4和IPv6格式',
  `user_agent` text COLLATE utf8mb4_bin COMMENT '用户代理字符串，包含浏览器和客户端信息',
  `is_active` tinyint(1) NOT NULL DEFAULT '1' COMMENT '会话激活状态，true=有效，false=已失效',
  `expires_at` timestamp NOT NULL COMMENT '会话过期时间，超过此时间会话自动失效',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '会话创建时间，自动创建时间戳',
  `last_used_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '会话最后使用时间，用于延长会话有效期',
  PRIMARY KEY (`id`),
  UNIQUE KEY `token` (`token`),
  UNIQUE KEY `idx_token` (`token`),
  KEY `idx_user_id` (`user_id`),
  KEY `idx_expires_at` (`expires_at`),
  KEY `idx_is_active` (`is_active`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='用户会话表，管理用户登录状态和令牌';



# 转储表 user_tasks
# ------------------------------------------------------------

CREATE TABLE `user_tasks` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '用户任务记录ID，主键',
  `user_id` bigint unsigned NOT NULL COMMENT '用户ID，外键关联users表',
  `task_config_id` bigint unsigned NOT NULL COMMENT '任务配置ID，外键关联task_configs表',
  `status` enum('available','running','completed','failed','cancelled') COLLATE utf8mb4_bin NOT NULL DEFAULT 'available' COMMENT '任务状态：available可用/running进行中/completed已完成/failed失败/cancelled已取消',
  `start_time` timestamp NULL DEFAULT NULL COMMENT '任务开始时间，null表示未开始',
  `end_time` timestamp NULL DEFAULT NULL COMMENT '任务结束时间，null表示未结束',
  `estimated_completion_time` timestamp NULL DEFAULT NULL COMMENT '预计完成时间，根据任务难度和历史数据估算',
  `is_accelerating` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否使用加速，true=使用积分加速，false=正常速度',
  `acceleration_multiplier` decimal(10,4) NOT NULL DEFAULT '1.0000' COMMENT '加速倍数，使用加速后的速度倍率，如1.5000表示1.5倍速',
  `points_used` bigint unsigned NOT NULL DEFAULT '0' COMMENT '已使用积分数量，用于加速任务的积分消耗',
  `earnings_rate` decimal(20,8) DEFAULT NULL COMMENT '收益率，用户完成任务的实时收益速率，8位小数精度',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '任务记录创建时间，自动创建时间戳',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '任务记录最后更新时间，自动更新时间戳',
  PRIMARY KEY (`id`),
  UNIQUE KEY `idx_user_task` (`user_id`,`task_config_id`),
  KEY `idx_user_id` (`user_id`),
  KEY `idx_task_config_id` (`task_config_id`),
  KEY `idx_status` (`status`),
  KEY `idx_start_time` (`start_time`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='用户任务记录表，记录用户接取和执行AI训练任务的详细过程和状态';



# 转储表 users
# ------------------------------------------------------------

CREATE TABLE `users` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '用户唯一标识ID，主键',
  `username` varchar(50) COLLATE utf8mb4_bin NOT NULL COMMENT '用户名，唯一索引，用于登录',
  `password_hash` varchar(255) COLLATE utf8mb4_bin NOT NULL COMMENT '用户密码的哈希值，使用bcrypt加密',
  `user_level` tinyint unsigned NOT NULL DEFAULT '0' COMMENT '用户等级，0-5级对应LV.0-LV.5',
  `upgrade_progress` int NOT NULL DEFAULT '0' COMMENT '升级进度',
  `invite_code` varchar(20) COLLATE utf8mb4_bin NOT NULL COMMENT '用户邀请码，用于邀请好友注册',
  `parent_inviter_id` bigint unsigned NOT NULL DEFAULT '0' COMMENT '邀请人的用户ID的上级，外键关联users表',
  `inviter_id` bigint unsigned NOT NULL COMMENT '邀请人的用户ID，外键关联users表',
  `total_assets` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT '用户总资产价值（USDT计价），8位小数精度',
  `dg_amount` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT '用户DG代币余额，8位小数精度',
  `is_kyc_verified` tinyint(1) NOT NULL DEFAULT '0' COMMENT 'KYC认证状态，true=已认证，false=未认证',
  `has_security_questions` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否设置密保问题，true=已设置，false=未设置',
  `is_active` tinyint(1) NOT NULL DEFAULT '1' COMMENT '账户激活状态，true=正常，false=禁用',
  `is_locked` tinyint(1) NOT NULL DEFAULT '0' COMMENT '账户锁定状态，true=锁定，false=正常',
  `login_attempts` tinyint unsigned NOT NULL DEFAULT '0' COMMENT '登录失败次数，超过阈值将锁定账户',
  `locked_until` timestamp NULL DEFAULT NULL COMMENT '账户锁定到期时间，null表示未锁定或永久锁定',
  `qr_code_url` varchar(500) COLLATE utf8mb4_bin DEFAULT NULL COMMENT '用户二维码图片URL地址，用于邀请和收款',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '用户注册时间，自动创建时间戳',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '用户信息最后更新时间，自动更新时间戳',
  `last_login_at` timestamp NULL DEFAULT NULL COMMENT '用户最后登录时间，null表示从未登录',
  PRIMARY KEY (`id`),
  UNIQUE KEY `username` (`username`),
  UNIQUE KEY `invite_code` (`invite_code`),
  UNIQUE KEY `idx_username` (`username`),
  UNIQUE KEY `idx_invite_code` (`invite_code`),
  KEY `idx_inviter_id` (`inviter_id`),
  KEY `idx_user_level` (`user_level`),
  KEY `idx_is_active` (`is_active`),
  KEY `idx_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='用户基本信息表';



# 转储表 withdrawal_requests
# ------------------------------------------------------------

CREATE TABLE `withdrawal_requests` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '提现记录ID，主键',
  `user_id` bigint unsigned NOT NULL COMMENT '提现用户ID，外键关联users表',
  `withdrawal_id` varchar(64) COLLATE utf8mb4_bin NOT NULL COMMENT '提现唯一标识，系统生成的提现单号，唯一索引',
  `amount` decimal(20,8) NOT NULL COMMENT '提现金额，8位小数精度，用户申请提现的具体金额',
  `currency` varchar(20) COLLATE utf8mb4_bin NOT NULL COMMENT '提现货币类型，如USDT、DG、BTC、ETH等',
  `blockchain_type` varchar(20) COLLATE utf8mb4_bin NOT NULL COMMENT '区块链网络类型，如TRC20、ERC20、BEP20等，用于区分不同网络提现',
  `destination_address` varchar(255) COLLATE utf8mb4_bin NOT NULL COMMENT '目标接收地址，用户提供的数字货币钱包地址',
  `status` enum('pending','processing','completed','failed','cancelled') COLLATE utf8mb4_bin NOT NULL DEFAULT 'pending' COMMENT '提现状态：pending待审核/processing处理中/completed已完成/failed失败/cancelled已取消',
  `fee` decimal(20,8) NOT NULL DEFAULT '0.00000000' COMMENT '提现手续费，8位小数精度，实际到账金额 = amount - fee',
  `transaction_hash` varchar(128) COLLATE utf8mb4_bin DEFAULT NULL COMMENT '区块链交易哈希，用于查询和验证链上交易状态',
  `confirmations` int unsigned DEFAULT NULL COMMENT '区块链确认数，表示交易被区块确认的次数',
  `reviewer_id` bigint unsigned DEFAULT NULL COMMENT '审核人员ID，外键关联管理员表，null表示未指定审核人',
  `rejection_reason` text COLLATE utf8mb4_bin COMMENT '拒绝原因，提现申请被拒绝时的详细说明',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '提现申请创建时间，自动创建时间戳',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '提现记录最后更新时间，自动更新时间戳',
  `processed_at` timestamp NULL DEFAULT NULL COMMENT '提现处理完成时间，null表示未处理',
  PRIMARY KEY (`id`),
  UNIQUE KEY `withdrawal_id` (`withdrawal_id`),
  UNIQUE KEY `idx_withdrawal_id` (`withdrawal_id`),
  KEY `idx_user_id` (`user_id`),
  KEY `idx_status` (`status`),
  KEY `idx_currency` (`currency`),
  KEY `idx_created_at` (`created_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='提现记录表，记录用户数字货币提现申请、审核和处理的全过程';




/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;
/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;

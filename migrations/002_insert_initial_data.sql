-- 插入初始安全问题数据
INSERT INTO security_questions (id, question, is_active, sort_order) VALUES
(1, '你的宠物叫什么名字？', TRUE, 1),
(2, '你的出生城市是哪里？', TRUE, 2),
(3, '你的母亲姓名是？', TRUE, 3),
(4, '你最喜欢的颜色是？', TRUE, 4),
(5, '你的第一所学校名称是？', TRUE, 5);

-- 插入算力等级数据
INSERT INTO power_levels (id, name, description, required_invites, reward_multiplier, is_active) VALUES
(0, 'LV.0', '初始等级', 0, 1.000, TRUE),
(1, 'LV.1', '基础等级', 5, 1.100, TRUE),
(2, 'LV.2', '进阶等级', 10, 1.250, TRUE),
(3, 'LV.3', '专业等级', 15, 1.400, TRUE),
(4, 'LV.4', '高级等级', 20, 1.600, TRUE),
(5, 'LV.5', '大师等级', 25, 1.800, TRUE),
(6, 'LV.6', '专家等级', 30, 2.000, TRUE),
(7, 'LV.7', '传奇等级', 35, 2.200, TRUE),
(8, 'LV.8', '神话等级', 40, 2.500, TRUE),
(9, 'LV.9', '至尊等级', 50, 3.000, TRUE);

-- 插入算力包数据
INSERT INTO power_packages (name, description, task_type, required_level, earnings_percent, amount, currency, duration_days, is_active, sort_order) VALUES
('AI智能计算', '为您提供稳定高效的AI算力服务', 'AI智能计算', 1, 5.8, 150.00, 'USDT', 30, TRUE, 1),
('深度学习训练', '基于深度学习模型的算力服务', '深度学习训练', 2, 6.2, 200.00, 'USDT', 30, TRUE, 2),
('区块链验证', '参与区块链网络验证获得收益', '区块链验证', 3, 4.5, 100.00, 'USDT', 30, TRUE, 3),
('数据挖掘分析', '大数据挖掘和分析服务', '数据挖掘分析', 1, 5.2, 120.00, 'USDT', 30, TRUE, 4),
('云计算服务', '提供弹性云计算资源', '云计算服务', 2, 4.8, 180.00, 'USDT', 30, TRUE, 5);

-- 插入任务数据 (与算力包类似)
INSERT INTO tasks (task_type, required_level, earnings_percent, amount, currency, description, completion_time, difficulty, is_active, sort_order) VALUES
('AI智能计算', 1, 5.8, 150.00, 'USDT', '为您提供稳定高效的AI算力服务', 7200, 'easy', TRUE, 1),
('深度学习训练', 2, 6.2, 200.00, 'USDT', '基于深度学习模型的算力服务', 10800, 'medium', TRUE, 2),
('区块链验证', 3, 4.5, 100.00, 'USDT', '参与区块链网络验证获得收益', 3600, 'easy', TRUE, 3),
('数据挖掘分析', 1, 5.2, 120.00, 'USDT', '大数据挖掘和分析服务', 5400, 'easy', TRUE, 4),
('云计算服务', 2, 4.8, 180.00, 'USDT', '提供弹性云计算资源', 9000, 'medium', TRUE, 5);

-- 插入邀请奖励数据
INSERT INTO invite_rewards (id, title, description, level, reward_amount, reward_type, required_progress, is_active) VALUES
('level_1', '一级好友奖励', '邀请1位好友注册并完成实名认证', 1, 100.00, 'DG', 1, TRUE),
('level_2', '二级好友奖励', '累计邀请5位好友注册并完成实名认证', 2, 500.00, 'DG', 5, TRUE),
('level_3', '三级好友奖励', '累计邀请10位好友注册并完成实名认证', 3, 1200.00, 'DG', 10, TRUE),
('level_4', '四级好友奖励', '累计邀请20位好友注册并完成实名认证', 4, 2500.00, 'DG', 20, TRUE),
('level_5', '五级好友奖励', '累计邀请50位好友注册并完成实名认证', 5, 8000.00, 'DG', 50, TRUE);

-- 插入支持的区块链类型数据
INSERT INTO supported_blockchains (code, name, fee, min_amount, max_amount, icon, confirmation_time, is_available) VALUES
('TRC20', 'TRC20 (TRON)', 1.0, 10.0, 100000.0, 'tron', '5分钟', TRUE),
('ERC20', 'ERC20 (Ethereum)', 5.0, 10.0, 100000.0, 'ethereum', '15分钟', FALSE),
('BEP20', 'BEP20 (BSC)', 0.5, 10.0, 100000.0, 'binance', '3分钟', TRUE),
('POLYGON', 'Polygon', 0.1, 10.0, 100000.0, 'polygon', '2分钟', TRUE);

-- 插入交易对数据
INSERT INTO trading_pairs (symbol, base_currency, quote_currency, precision_amount, precision_price, is_active, min_trade_amount, max_trade_amount) VALUES
('BTC/USDT', 'BTC', 'USDT', 8, 2, TRUE, 0.0001, 100),
('ETH/USDT', 'ETH', 'USDT', 6, 2, TRUE, 0.001, 1000),
('DG/USDT', 'DG', 'USDT', 2, 4, TRUE, 1.0, 1000000),
('BNB/USDT', 'BNB', 'USDT', 4, 2, TRUE, 0.01, 10000);

-- 插入价格信息数据
INSERT INTO price_info (symbol, current_price, price_change, price_change_percent, high_24h, low_24h, volume_24h, market_cap, circulating_supply) VALUES
('BTC/USDT', 45200.00, 200.00, 0.44, 45500.00, 44800.00, 1000000.00, 856000000000.00, 18900000.00),
('ETH/USDT', 2450.00, -50.00, -2.00, 2520.00, 2430.00, 5000000.00, 294000000000.00, 120000000.00),
('DG/USDT', 0.10, 0.005, 5.26, 0.098, 0.092, 10000000.00, 10000000.00, 100000000.00),
('BNB/USDT', 320.00, 10.00, 3.23, 325.00, 310.00, 2000000.00, 48000000000.00, 150000000.00);

-- 插入轮播图数据
INSERT INTO carousel_items (image_url, title, subtitle, description, action_url, sort_order, is_active) VALUES
('https://api.example.com/images/carousel-1.png', 'advanced_mining.title', 'advanced_mining.subtitle', '先进的AI挖矿技术，为您提供稳定的收益', '/promotion/advanced-mining', 1, TRUE),
('https://api.example.com/images/carousel-2.png', 'smart_investing.title', 'smart_investing.subtitle', '智能投资策略，让您的资产增值', '/promotion/smart-investing', 2, TRUE),
('https://api.example.com/images/carousel-3.png', 'secure_trading.title', 'secure_trading.subtitle', '安全可靠的交易平台，保障您的资产安全', '/security/features', 3, TRUE);

-- 插入限时限惠套餐数据
INSERT INTO promotion_packages (name, price, original_price, currency, description, profit_percentage, duration_days, features, start_time, end_time, stock, sold, is_available) VALUES
('高级算力套餐', 2999.00, 3999.00, 'USDT', '送100T算力', 15.0, 30, '["100T AI算力", "15% 收益率", "专属客服", "优先支持"]', '2025-11-23 00:00:00', '2025-11-30 23:59:59', 100, 75, TRUE),
('尊享算力套餐', 5999.00, 7999.00, 'USDT', '送200T算力', 18.0, 30, '["200T AI算力", "18% 收益率", "VIP专属服务", "优先支持", "生日福利"]', '2025-11-23 00:00:00', '2025-11-30 23:59:59', 50, 30, TRUE);

-- 插入空投活动数据
INSERT INTO airdrop_activities (type, title, subtitle, description, activity_start_time, activity_end_time, total_rounds, round_duration, interval_duration, participation_type, color, status) VALUES
('daily', '每日空投', '所有人可参与', '每天定时抢空投，获得随机DG奖励', '2025-11-23 00:00:00', '2025-11-23 23:59:59', 999, 60, 20, '{"runtimeType":"allUsers"}', '0xFF4ECDC4', 'active'),
('vip', '会员专属空投', 'VIP会员专享', 'VIP会员专属空投活动，奖励更丰厚', '2025-11-23 00:00:00', '2025-11-30 23:59:59', 100, 300, 60, '{"runtimeType":"memberLevel","requiredLevel":3}', '0xFFD700', 'active'),
('special', '限时特惠空投', '限时活动', '限时特惠空投活动，数量有限，先到先得', '2025-11-23 10:00:00', '2025-11-23 22:00:00', 50, 1800, 300, '{"runtimeType":"allUsers"}', '0xFFFF00', 'active');

-- 插入统计数据
INSERT INTO statistics (daily_yield, total_yield, today_progress, active_users, total_users, total_airdrop, success_rate, system_status) VALUES
(0.011, 2108.0, '0%', 15420, 256890, 8921, '85.6', 'healthy');

-- 插入节点统计数据
INSERT INTO node_stats (computing_power, status_message, total_nodes, active_nodes, earnings, is_active, network_status, utilization_rate, average_response_time, throughput) VALUES
('2000K', 'Please stay tuned', 1250, 1180, '$2.5K', TRUE, 'healthy', 0.856, 0.045, 1250.5);

-- 插入关于我们信息
INSERT INTO about_us (email, phone, website, address, version, version_tag, copyright, disclaimer, app_description, team_info) VALUES
('support@astrai.com', '400-123-4567', 'www.astrai.com', '新加坡科技园', '1.0.0', 'Latest', '© 2025 Astra Ai. All rights reserved.', '投资有风险，入市需谨慎', 'Astra Ai是一款基于AI技术的算力挖矿应用，为用户提供稳定高效的数字货币挖矿服务。', '[{"name":"技术团队","description":"来自全球顶级科技公司的AI专家"},{"name":"运营团队","description":"拥有丰富数字货币运营经验的专业团队"}]');

-- 插入K线数据示例 (BTC/USDT 1分钟线)
INSERT INTO kline_data (symbol, interval_type, timestamp, open_price, high_price, low_price, close_price, volume, amount) VALUES
('BTC/USDT', '1m', 1640995200, 45000.00, 45500.00, 44800.00, 45200.00, 1000000.00, 45200000000.00),
('BTC/USDT', '1m', 1640995260, 45200.00, 45300.00, 45100.00, 45250.00, 800000.00, 36160000000.00),
('BTC/USDT', '1m', 1640995320, 45250.00, 45400.00, 45200.00, 45350.00, 900000.00, 40770000000.00);
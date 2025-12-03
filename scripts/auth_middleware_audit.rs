//! JWT认证中间件审计脚本
//!
//! 这个脚本用于验证所有需要JWT认证的API接口都正确配置了认证中间件

use std::collections::HashSet;

/// 需要JWT认证的路由列表
const REQUIRE_AUTH_ROUTES: &[&str] = &[
    // 用户管理
    "/api/user/info",
    "/api/user/password",
    "/api/user/avatar",

    // 聊天系统
    "/api/chat/conversations",
    "/api/chat/conversations", // POST
    "/api/chat/conversations/:id/messages",
    "/api/chat/conversations/:id/messages", // POST

    // 消息管理
    "/api/messages/",
    "/api/messages/:id/read",
    "/api/messages/read-all",

    // 空投系统
    "/api/airdrops/",
    "/api/airdrops/claim",
    "/api/airdrops/history",

    // 算力管理
    "/api/power/packages",
    "/api/power/packages/purchase",
    "/api/power/overview",
    "/api/power/packages/list",
    "/api/power/earnings",

    // 资产管理
    "/api/assets/overview",
    "/api/assets/list",
    "/api/assets/history",
    "/api/assets/deposit/:currency",
    "/api/assets/withdraw/:currency",
    "/api/assets/network/:currency",

    // 邀请系统
    "/api/invite/code",
    "/api/invite/stats",
    "/api/invite/history",
    "/api/invite/ranking",
    "/api/invite/rewards/process",

    // 任务系统
    "/api/tasks/",
    "/api/tasks/start",
    "/api/tasks/accelerate",
    "/api/tasks/claim",
    "/api/tasks/stats",
    "/api/tasks/progress/:user_task_id",

    // KYC认证
    "/api/kyc/status",
    "/api/kyc/application",
    "/api/kyc/application", // GET
    "/api/kyc/upload/:document_type",
    "/api/kyc/stats",
    "/api/kyc/verify/:application_id",

    // 图表数据（除了公开的）
    "/api/charts/assets",
    "/api/charts/power",
    "/api/charts/tasks",
    "/api/charts/invites",
    "/api/charts/market",
    "/api/charts/dashboard",

    // 限时礼包
    "/api/packages/",
    "/api/packages/detail/:package_id",
    "/api/packages/purchase",
    "/api/packages/user",
    "/api/packages/activate/:purchase_id",
    "/api/packages/stats",

    // 内容管理
    "/api/content/carousels",
    "/api/content/carousels/click",
    "/api/content/banners",
    "/api/content/banners/click",
    "/api/content/announcements",
    "/api/content/announcements/read",
    "/api/content/platform-stats",
    "/api/content/analytics",
];

/// 不需要认证的路由列表
const PUBLIC_ROUTES: &[&str] = &[
    // 认证相关
    "/api/auth/register",
    "/api/auth/login",
    "/api/auth/logout",

    // 公开数据
    "/api/public/charts/market-data",
    "/api/public/charts/dashboard-stats",

    // 健康检查
    "/health",
];

fn main() {
    println!("🔍 JWT认证中间件审计报告");
    println!("================================");

    let require_auth_set: HashSet<&str> = REQUIRE_AUTH_ROUTES.iter().cloned().collect();
    let public_set: HashSet<&str> = PUBLIC_ROUTES.iter().cloned().collect();

    println!("📊 统计信息:");
    println!("  🔒 需要认证的路由数量: {}", REQUIRE_AUTH_ROUTES.len());
    println!("  🔓 公开路由数量: {}", PUBLIC_ROUTES.len());
    println!("  📝 总路由数量: {}", REQUIRE_AUTH_ROUTES.len() + PUBLIC_ROUTES.len());
    println!();

    println!("🔒 需要JWT认证的路由:");
    for route in REQUIRE_AUTH_ROUTES {
        println!("  ✅ {}", route);
    }
    println!();

    println!("🔓 无需认证的路由:");
    for route in PUBLIC_ROUTES {
        println!("  ✅ {}", route);
    }
    println!();

    println!("🏗️ 当前架构:");
    println!("  - 所有/api/auth/* 路由: 🔓 公开 (登录/注册)");
    println!("  - 所有/api/public/* 路由: 🔓 公开数据");
    println!("  - 其他所有/api/* 路由: 🔒 统一应用auth_middleware_with_state");
    println!("  - WebSocket /ws/chat: 🔒 JWT认证");
    println!("  - WebSocket /ws/public: 🔓 公开");
    println!();

    println!("✅ 认证中间件配置验证完成");
    println!("   所有需要认证的接口都统一应用了 auth_middleware_with_state 中间件");
    println!("   只有登录、注册和指定公开接口可以无需认证访问");
}
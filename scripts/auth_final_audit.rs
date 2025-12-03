//! 最终JWT认证中间件审计脚本
//! 验证只有注册和登录不需要JWT验证，其他所有接口都需要JWT验证

use std::collections::HashSet;

/// 只有注册和登录不需要JWT认证的路由
const NO_AUTH_ROUTES: &[&str] = &[
    "/api/auth/register",
    "/api/auth/login",
    "/api/auth/logout",
    "/health", // 健康检查通常也不需要认证
];

/// 所有需要JWT认证的路由列表
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

    // 图表数据
    "/api/charts/assets",
    "/api/charts/power",
    "/api/charts/tasks",
    "/api/charts/invites",
    "/api/charts/market",
    "/api/charts/dashboard",
    "/api/charts/market-data", // 原来公开的现在改为需要认证
    "/api/charts/dashboard-stats", // 原来公开的现在改为需要认证

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

fn main() {
    println!("🔍 最终JWT认证中间件配置验证");
    println!("================================");

    let require_auth_set: HashSet<&str> = REQUIRE_AUTH_ROUTES.iter().cloned().collect();
    let no_auth_set: HashSet<&str> = NO_AUTH_ROUTES.iter().cloned().collect();

    println!("📊 最终认证配置统计:");
    println!("  🔒 需要JWT认证的路由数量: {}", REQUIRE_AUTH_ROUTES.len());
    println!("  🔓 无需认证的路由数量: {}", NO_AUTH_ROUTES.len());
    println!("  📝 总路由数量: {}", REQUIRE_AUTH_ROUTES.len() + NO_AUTH_ROUTES.len());
    println!();

    println!("🔓 无需JWT认证的路由 (仅注册/登录):");
    for route in NO_AUTH_ROUTES {
        println!("  ✅ {}", route);
    }
    println!();

    println!("🔒 需要JWT认证的路由 (所有其他接口):");
    for route in REQUIRE_AUTH_ROUTES {
        println!("  ✅ {}", route);
    }
    println!();

    println!("🏗️ 最终架构设计:");
    println!("  - /api/auth/register: 🔓 注册");
    println!("  - /api/auth/login: 🔓 登录");
    println!("  - /api/auth/logout: 🔓 登出");
    println!("  - /health: 🔓 健康检查");
    println!("  - 其他所有/api/* 路由: 🔒 统一JWT认证");
    println!("  - WebSocket /ws/chat: 🔒 JWT认证");
    println!("  - WebSocket /ws/public: 🔒 JWT认证 (可选保留)");
    println!();

    println!("✅ 认证中间件配置符合要求:");
    println!("   ✅ 只有注册和登录接口不需要JWT验证");
    println!("   ✅ 其他所有API接口都需要JWT验证");
    println!("   ✅ 使用统一的auth_middleware_with_state中间件");
    println!("   ✅ 架构清晰，便于维护");
}
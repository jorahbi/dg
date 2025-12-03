use snowflake::SnowflakeIdGenerator;
use std::sync::{Mutex, OnceLock};

// 全局单例（线程安全）
static ID_GENERATOR: OnceLock<Mutex<SnowflakeIdGenerator>> = OnceLock::new();

fn get_generator() -> &'static Mutex<SnowflakeIdGenerator> {
    ID_GENERATOR.get_or_init(|| {
        // machine_id: 0~31, node_id: 0~31，根据你的机器数改
        Mutex::new(SnowflakeIdGenerator::new(1, 1))
    })
}

// 生成订单号：20231225 + 19位 snowflake（共27位）
pub fn generate_no(prefix: &str) -> String {
    let mut gen = get_generator().lock().unwrap();
    let id = gen.real_time_generate(); // 严格趋势递增

    let today = chrono::Utc::now().format("%Y%m%d").to_string();
    format!("{}{today}{:019}", prefix, id) // 示例：20251129123456789012345
}

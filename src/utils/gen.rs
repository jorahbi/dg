use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

// 自定义 Snowflake ID 生成器
#[derive(Debug)]
struct SimpleSnowflakeIdGenerator {
    machine_id: u64,
    node_id: u64,
    last_timestamp: u64,
    sequence: u64,
}

impl SimpleSnowflakeIdGenerator {
    fn new(machine_id: u64, node_id: u64) -> Self {
        Self {
            machine_id,
            node_id,
            last_timestamp: 0,
            sequence: 0,
        }
    }

    fn generate(&mut self) -> u64 {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        if timestamp == self.last_timestamp {
            self.sequence = (self.sequence + 1) & 0xFFF; // 12-bit sequence
            if self.sequence == 0 {
                // Wait for next millisecond
                while SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64
                    <= timestamp
                {}
            }
        } else {
            self.sequence = 0;
        }

        self.last_timestamp = timestamp;

        // Snowflake format: 41-bit timestamp + 5-bit node_id + 5-bit machine_id + 12-bit sequence
        ((timestamp & 0x1FFFFFFFFFF) << 22)
            | ((self.node_id & 0x1F) << 17)
            | ((self.machine_id & 0x1F) << 12)
            | (self.sequence & 0xFFF)
    }
}

// 全局单例（线程安全）
static ID_GENERATOR: OnceLock<Mutex<SimpleSnowflakeIdGenerator>> = OnceLock::new();

fn get_generator() -> &'static Mutex<SimpleSnowflakeIdGenerator> {
    ID_GENERATOR.get_or_init(|| {
        // machine_id: 0~31, node_id: 0~31，根据你的机器数改
        Mutex::new(SimpleSnowflakeIdGenerator::new(1, 1))
    })
}

// 生成订单号：20231225 + 19位 snowflake（共27位）
pub fn generate_no(prefix: &str) -> String {
    let mut gen = get_generator().lock().unwrap();
    let id = gen.generate(); // 严格趋势递增

    let today = chrono::Utc::now().format("%Y%m%d").to_string();
    format!("{}{today}{:019}", prefix, id) // 示例：20251129123456789012345
}

use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, UtcOffset};
use strum::{Display, EnumString};
/// 北京时间偏移（东八区），time 0.3.44 推荐写法
const BEIJING_OFFSET: UtcOffset = {
    match UtcOffset::from_hms(8, 0, 0) {
        Ok(offset) => offset,
        Err(_) => panic!("Invalid timezone offset: +08:00 is always valid"),
    }
};

#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display)]
pub enum TimeZone {
    #[strum(to_string = "Asia/Shanghai")]
    Beijing,
}

impl TimeZone {
    pub fn get_time(&self) -> OffsetDateTime {
        match self.to_string().as_str() {
            "Asia/Shanghai" =>  OffsetDateTime::now_utc().to_offset(BEIJING_OFFSET),
            _ => OffsetDateTime::now_utc()
        }
    }
}


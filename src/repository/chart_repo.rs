use crate::{error::Result, state::AppState};

pub struct ChartRepo;

impl ChartRepo {
    pub async fn simple_method(_state: &AppState) -> Result<()> {
        // 暂时空实现，避免SQLX编译错误
        // TODO: 实现真正的数据库操作
        Ok(())
    }
}

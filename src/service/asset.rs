use crate::{error::Result, state::AppState};

pub struct AssetService;

impl AssetService {
    pub fn new(_state: &AppState) -> Self {
        Self
    }
    pub async fn get_recharge_records(&self, _user_id: i64) -> Result<()> {
        Ok(())
    }

    pub async fn create_withdrawal(&self, _user_id: i64) -> Result<()> {
        Ok(())
    }
}
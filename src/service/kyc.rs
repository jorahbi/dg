use crate::{error::Result, state::AppState};

pub struct KycService;

impl KycService {
    pub fn new() -> Self {
        Self
    }
    pub async fn get_kyc_status(_state: &AppState) -> Result<()> {
        Ok(())
    }
}

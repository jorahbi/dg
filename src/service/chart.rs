use crate::{error::Result, state::AppState};

pub struct ChartService;

impl ChartService {
    pub fn new() -> Self {
        Self
    }
    pub async fn get_chart_data(_state: &AppState) -> Result<()> {
        Ok(())
    }
}

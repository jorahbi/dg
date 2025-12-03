use crate::{error::Result, state::AppState};

pub struct ContentService;

impl ContentService {
    pub fn new() -> Self {
        Self
    }
    pub async fn get_contents(_state: &AppState) -> Result<()> {
        Ok(())
    }
}

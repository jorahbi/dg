use crate::{error::Result, state::AppState};

pub struct MessageService;

impl MessageService {
    pub fn new() -> Self {
        Self
    }
    pub async fn get_messages(_state: &AppState) -> Result<()> {
        Ok(())
    }
}

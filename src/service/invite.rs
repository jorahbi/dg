use crate::{error::Result, state::AppState};

pub struct InviteService;

impl InviteService {
    pub fn new(_state: &AppState) -> Self {
        Self
    }
    pub async fn get_invite_rewards(&self, _user_id: i64) -> Result<()> {
        Ok(())
    }

    pub async fn get_invite_code(&self, _user_id: i64) -> Result<()> {
        Ok(())
    }
}

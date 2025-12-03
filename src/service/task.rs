use crate::{error::Result, state::AppState};

pub struct TaskService;

impl TaskService {
    pub fn new(_state: &AppState) -> Self {
        Self
    }

    pub async fn get_tasks(&self) -> Result<()> {
        Ok(())
    }

    pub async fn start_task(&self, _task_id: i64) -> Result<()> {
        Ok(())
    }
}
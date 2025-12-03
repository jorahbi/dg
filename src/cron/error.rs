use thiserror::Error;

#[derive(Error, Debug)]
pub enum CronError {
    #[error("Scheduler error: {0}")]
    SchedulerError(String),

    #[error("Task execution error: {0}")]
    TaskError(String),

    #[error("Time parse error: {0}")]
    TimeParseError(String),
}

impl From<Box<dyn std::error::Error + Send + Sync>> for CronError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        CronError::SchedulerError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, CronError>;
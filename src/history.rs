use chrono::{DateTime, Local};

#[derive(Clone)]
pub struct TaskSummary {
    pub task_name: String,
    pub filename: String,
    pub total_events: usize,
    pub timestamp: DateTime<Local>,
}

impl TaskSummary {
    pub fn display_string(&self) -> String {
        format!(
            "[{}] {} — {} events → {}",
            self.timestamp.format("%Y-%m-%d %H:%M:%S"),
            self.task_name,
            self.total_events,
            self.filename
        )
    }
}
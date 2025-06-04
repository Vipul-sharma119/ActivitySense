use chrono::Local;

#[derive(Debug, Clone)]
pub struct ActionEvent {
    pub timestamp: String,
    pub event_type: String,
    pub details: String,
}

impl ActionEvent {
    pub fn new(event_type: &str, details: &str) -> Self {
        Self {
            timestamp: chrono::Local::now().to_rfc3339(),
            event_type: event_type.to_string(),
            details: details.to_string(),
        }
    }
}

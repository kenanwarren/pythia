use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct Event {
    pub id: Uuid,
    pub r#type: String,
    pub level: String,
    pub timestamp: DateTime<Utc>,
    pub source: String,
    pub payload: String,
    pub trigger: String,
    pub category: String,
    pub account_id: Uuid,
}

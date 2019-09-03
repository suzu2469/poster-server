use chrono::{DateTime, TimeZone, Utc};

pub struct Todo {
    pub id: String,
    pub name: String,
    pub is_done: bool,
    pub created_at: DateTime<Utc>,
}
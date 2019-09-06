use chrono::{DateTime, Utc};

pub struct Todo {
    pub id: i32,
    pub name: String,
    pub is_done: bool,
    pub created_at: DateTime<Utc>,
}

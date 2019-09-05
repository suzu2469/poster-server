use crate::domain::entity::todo::Todo;
use crate::domain::repository::todo::TodoRepository;
use chrono::Utc;

#[derive(Copy, Clone)]
pub struct MockDatastore {}

impl TodoRepository for MockDatastore {
    fn list(&self) -> Vec<Todo> {
        vec![
            Todo {
                id: "1".to_string(),
                name: "Task1".to_string(),
                is_done: false,
                created_at: Utc::now(),
            },
            Todo {
                id: "2".to_string(),
                name: "Task2".to_string(),
                is_done: true,
                created_at: Utc::now(),
            },
        ]
    }
}

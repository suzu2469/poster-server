use crate::domain::entity::todo::Todo;

pub trait TodoRepository {
    fn list(&self) -> Vec<Todo>;
}

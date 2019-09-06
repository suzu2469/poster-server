use crate::domain::entity::todo::Todo;
use crate::shared::DBConnection;

pub trait TodoRepository {
    fn list(&self, conn: &DBConnection) -> Vec<Todo>;
    fn create(&self, conn: &DBConnection, dto: &TodoCreateDTO) -> Result<(), ()>;
}

pub struct TodoCreateDTO {
    pub name: String,
    pub is_done: bool,
}

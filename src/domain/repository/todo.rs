use crate::domain::entity::todo::Todo;
use crate::shared::DBConnection;

pub trait TodoRepository {
    fn list(&self, conn: &DBConnection) -> Vec<Todo>;
    fn create(&self, conn: &DBConnection, dto: &TodoCreateDTO) -> Result<(), ()>;
    fn update(&self, conn: &DBConnection, id: i32, dto: &TodoUpdateDTO) -> Result<(), ()>;
}

pub struct TodoCreateDTO {
    pub name: String,
    pub is_done: bool,
}

pub struct TodoUpdateDTO {
    pub name: String,
    pub is_done: bool,
}

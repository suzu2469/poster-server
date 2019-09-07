use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;

use crate::application::controller::todo::{TodoController, TodoCreateInput, TodoUpdateInput};
use crate::domain::repository::todo::TodoRepository;
use crate::shared::DBConnection;


#[derive(Copy, Clone)]
pub struct TodoHandler<T: TodoRepository> {
    pub todo_controller: TodoController<T>,
}

impl<T: TodoRepository> TodoHandler<T> {
    pub fn list(&self, pool: &web::Data<DBConnection>) -> Result<HttpResponse> {
        self.todo_controller.list(pool.get_ref())
    }
    pub fn create(
        &self,
        pool: &web::Data<DBConnection>,
        data: &web::Json<TodoCreateInput>,
    ) -> Result<HttpResponse> {
        self.todo_controller.create(pool, data)
    }
    pub fn update(
        &self,
        pool: &web::Data<DBConnection>,
        data: &web::Json<TodoUpdateInput>,
        path: &web::Path<TodoUpdatePath>,
    ) -> Result<HttpResponse> {
        self.todo_controller.update(pool, path.id, data)
    }
}

#[derive(Deserialize)]
pub struct TodoUpdatePath {
    id: i32,
}

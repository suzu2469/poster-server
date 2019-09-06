use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;

use crate::application::controller::todo::{TodoController, TodoCreateInput};
use crate::domain::repository::todo::TodoRepository;
use crate::shared::DBConnection;
use actix_web::web::BytesMut;

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
}

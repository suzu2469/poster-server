use actix_web::{HttpResponse, Result};
use serde::Deserialize;

use crate::application::usecase::todo::TodoUsecase;
use crate::domain::repository::todo::{TodoCreateDTO, TodoRepository};
use crate::shared::DBConnection;

#[derive(Copy, Clone)]
pub struct TodoController<T: TodoRepository> {
    pub todo_usecase: TodoUsecase<T>,
}

impl<T: TodoRepository> TodoController<T> {
    pub fn list(&self, conn: &DBConnection) -> Result<HttpResponse> {
        self.todo_usecase.list(conn)
    }
    pub fn create(&self, conn: &DBConnection, input: &TodoCreateInput) -> Result<HttpResponse> {
        let dto = TodoCreateDTO {
            name: input.name.clone(),
            is_done: input.is_done,
        };
        self.todo_usecase.create(conn, &dto)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TodoCreateInput {
    pub name: String,
    pub is_done: bool,
}

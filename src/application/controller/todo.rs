use actix_web::{HttpResponse, Result};

use crate::application::usecase::todo::TodoUsecase;
use crate::domain::repository::todo::TodoRepository;

#[derive(Copy, Clone)]
pub struct TodoController<T: TodoRepository> {
    pub todo_usecase: TodoUsecase<T>,
}

impl<T: TodoRepository> TodoController<T> {
    pub fn list(&self) -> Result<HttpResponse> {
        self.todo_usecase.list()
    }
}

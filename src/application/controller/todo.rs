use actix_web::{HttpResponse, Result};

use crate::application::usecase::todo::TodoUsecase;

pub struct TodoController {
    todo_usecase: TodoUsecase,
}

impl TodoController {
    pub fn list(&self) -> Result<HttpResponse> {
        self.todo_usecase.list()
    }
}

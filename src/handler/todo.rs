use actix_web::{HttpRequest, HttpResponse, Result};

use crate::application::controller::todo::TodoController;
use crate::domain::repository::todo::TodoRepository;

#[derive(Copy, Clone)]
pub struct TodoHandler<T: TodoRepository> {
    pub todo_controller: TodoController<T>,
}

impl<T: TodoRepository> TodoHandler<T> {
    pub fn list(&self, _r: HttpRequest) -> Result<HttpResponse> {
        self.todo_controller.list()
    }
}

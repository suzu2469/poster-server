use actix_web::{HttpResponse, Result};

use crate::application::presenter::todo::TodoPresenter;
use crate::domain::repository::todo::TodoRepository;

#[derive(Copy, Clone)]
pub struct TodoUsecase<T: TodoRepository> {
    pub todo_repository: T,
    pub todo_presenter: TodoPresenter,
}

impl<T: TodoRepository> TodoUsecase<T> {
    pub fn list(&self) -> Result<HttpResponse> {
        let todos = self.todo_repository.list();
        self.todo_presenter.list(&todos)
    }
}

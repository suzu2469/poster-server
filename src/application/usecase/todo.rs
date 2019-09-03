use actix_web::{HttpResponse, Result};

use crate::application::presenter::todo::TodoPresenter;
use crate::domain::repository::todo::TodoRepository;

pub struct TodoUsecase {
    todo_repository: dyn TodoRepository,
    todo_presenter: TodoPresenter,
}

impl TodoUsecase {
    pub fn list(&self) -> Result<HttpResponse> {
        let todos = self.todo_repository.list();
        self.todo_presenter.list(&todos)
    }
}

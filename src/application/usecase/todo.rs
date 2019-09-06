use actix_web::{HttpResponse, Result};

use crate::application::presenter::todo::TodoPresenter;
use crate::domain::repository::todo::{TodoCreateDTO, TodoRepository};
use crate::shared::DBConnection;

#[derive(Copy, Clone)]
pub struct TodoUsecase<T: TodoRepository> {
    pub todo_repository: T,
    pub todo_presenter: TodoPresenter,
}

impl<T: TodoRepository> TodoUsecase<T> {
    pub fn list(&self, conn: &DBConnection) -> Result<HttpResponse> {
        let todos = self.todo_repository.list(conn);
        self.todo_presenter.list(&todos)
    }

    pub fn create(&self, conn: &DBConnection, dto: &TodoCreateDTO) -> Result<HttpResponse> {
        let res = self.todo_repository.create(conn, dto);
        match res {
            Ok(()) => Ok(self.todo_presenter.ok(None)),
            Err(()) => Ok(self.todo_presenter.ng(None)),
        }
    }
}

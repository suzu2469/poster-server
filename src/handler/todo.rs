use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;

use crate::application::controller::todo::{TodoController, TodoCreateInput, TodoUpdateInput};
use crate::application::presenter::todo::TodoPresenter;
use crate::application::usecase::todo::TodoUsecase;
use crate::infrastructure::postgres::adapter::todo::PgAdapter;
use crate::shared::DBConnection;

fn todo_controller_factory() -> TodoController<PgAdapter> {
    let todo_presenter = TodoPresenter {};
    let todo_repository = PgAdapter {};
    let todo_usecase = TodoUsecase {
        todo_presenter,
        todo_repository,
    };
    TodoController { todo_usecase }
}

pub fn list(pool: web::Data<DBConnection>) -> Result<HttpResponse> {
    let todo_controller = todo_controller_factory();
    todo_controller.list(&pool)
}
pub fn create(
    pool: web::Data<DBConnection>,
    data: web::Json<TodoCreateInput>,
) -> Result<HttpResponse> {
    let todo_controller = todo_controller_factory();
    todo_controller.create(&pool, &data)
}
pub fn update(
    pool: web::Data<DBConnection>,
    data: web::Json<TodoUpdateInput>,
    path: web::Path<TodoUpdatePath>,
) -> Result<HttpResponse> {
    let todo_controller = todo_controller_factory();
    todo_controller.update(&pool, path.id, &data)
}
pub fn delete(
    pool: web::Data<DBConnection>,
    path: web::Path<TodoDeletePath>,
) -> Result<HttpResponse> {
    let todo_controller = todo_controller_factory();
    todo_controller.delete(&pool, path.id)
}

#[derive(Deserialize)]
pub struct TodoUpdatePath {
    id: i32,
}

#[derive(Deserialize)]
pub struct TodoDeletePath {
    id: i32,
}

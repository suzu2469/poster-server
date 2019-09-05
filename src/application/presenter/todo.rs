use actix_web::{HttpResponse, Result};

use crate::domain::entity::todo::Todo;
use serde::Serialize;

#[derive(Copy, Clone)]
pub struct TodoPresenter {}

impl TodoPresenter {
    pub fn list(&self, todos: &Vec<Todo>) -> Result<HttpResponse> {
        let data = TodoListResponse(
            todos
                .iter()
                .map(|t| TodoListResponseItem {
                    id: t.id.clone(),
                    name: t.name.clone(),
                    is_done: t.is_done.clone(),
                })
                .collect(),
        );
        Ok(HttpResponse::Ok().json(data))
    }
}

#[derive(Serialize)]
struct TodoListResponseItem {
    id: String,
    name: String,
    is_done: bool,
}

#[derive(Serialize)]
struct TodoListResponse(Vec<TodoListResponseItem>);

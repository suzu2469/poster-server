use actix_web::{HttpResponse, Result};

use crate::domain::entity::todo::Todo;
use serde::Serialize;

pub struct TodoPresenter {
    http_response: HttpResponse,
}

impl TodoPresenter {
    pub fn list(&self, todos: &Vec<Todo>) -> Result<HttpResponse> {
        let data: TodoListResponse = todos
            .iter()
            .map(|t| TodoListResponseItem {
                id: t.id,
                name: t.name,
                is_done: t.is_done,
            })
            .collect();
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

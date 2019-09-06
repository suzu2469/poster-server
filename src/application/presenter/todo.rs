use actix_web::{HttpResponse, Result};

use crate::domain::entity::todo::Todo;
use actix_web::http::StatusCode;
use serde::Serialize;

#[derive(Copy, Clone)]
pub struct TodoPresenter {}

impl TodoPresenter {
    pub fn list(&self, todos: &Vec<Todo>) -> Result<HttpResponse> {
        let data = TodoListResponse(
            todos
                .iter()
                .map(|t| TodoListResponseItem {
                    id: t.id,
                    name: t.name.clone(),
                    is_done: t.is_done.clone(),
                })
                .collect(),
        );
        Ok(HttpResponse::Ok().json(data))
    }

    pub fn ok(&self, payload: Option<String>) -> HttpResponse {
        match payload {
            Some(s) => HttpResponse::Ok().json(TodoOkResponse { message: s }),
            None => HttpResponse::Ok()
                .status(StatusCode::from_u16(204).unwrap())
                .finish(),
        }
    }

    pub fn ng(&self, payload: Option<String>) -> HttpResponse {
        HttpResponse::InternalServerError().json(TodoNgResponse {
            message: "Internal Server Error".to_string(),
            payload,
        })
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TodoListResponseItem {
    id: i32,
    name: String,
    is_done: bool,
}

#[derive(Serialize)]
struct TodoListResponse(Vec<TodoListResponseItem>);

#[derive(Serialize)]
struct TodoOkResponse {
    message: String,
}

#[derive(Serialize)]
struct TodoNgResponse {
    message: String,
    payload: Option<String>,
}

use actix_web::{get, post, put, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

use crate::application::controller::todo::TodoController;

pub struct TodoHandler {
    todo_controller: TodoController,
}

impl TodoHandler {
    #[get("/todos")]
    pub fn list(&self) -> Result<HttpResponse> {
        self.todo_controller.list()
    }
}

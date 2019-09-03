#[macro_use]
extern crate actix_web;
extern crate serde_derive;

pub mod application;
mod domain;
mod handler;
mod infrastructure;

use actix_web::{web, App, HttpServer, Responder};

#[get("/")]
fn index() -> impl Responder {
    "Hello World"
}

fn main() -> std::io::Result<()> {
    let todo_presenter = application::presenter::todo::TodoPresenter {
        json: web::HttpResponse
    }
    let todo_usecase = domain::usecase::todo::TodoUsecase {

    }
    let todo_controller = application::controller::todo::TodoController {
        todoUsecase:
    }
    let todoHandler = handler::todo::TodoHandler {
        todo_controller:
    }

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(handler::todo::list)
            .service(handler::todo::create)
            .service(handler::todo::update)
    })
    .bind("127.0.0.1:8080")?
    .run()
}

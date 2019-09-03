#[macro_use]
extern crate actix_web;
extern crate serde_derive;

mod handler;

use actix_web::{App, HttpServer, Responder};

#[get("/")]
fn index() -> impl Responder {
    "Hello World"
}

fn main() -> std::io::Result<()> {
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

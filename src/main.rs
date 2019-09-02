#[macro_use]
extern crate actix_web;
extern crate serde_derive;

mod controller;

use actix_web::{App, HttpServer, Responder};

#[get("/")]
fn index() -> impl Responder {
    "Hello World"
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(controller::todo::list)
            .service(controller::todo::create)
            .service(controller::todo::update)
    })
    .bind("127.0.0.1:8080")?
    .run()
}

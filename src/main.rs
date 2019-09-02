#[macro_use]
extern crate actix_web;

use crate::controller;
use actix_web::{web, App, HttpServer, Responder};

#[get("/")]
fn index() -> impl Responder {
    "Hello World"
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(controller::todo::list))
        .bind("127.0.0.1:8080")?
        .run()
}

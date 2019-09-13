extern crate structopt;
#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;
extern crate serde_derive;
extern crate structopt_derive;

mod application;
mod domain;
mod handler;
mod infrastructure;
mod schema;

use crate::handler::todo as TodoHandler;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer, Responder};
use infrastructure::postgres::connection;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "poster-server")]
struct Options {
    #[structopt(short = "p", long = "port", default_value = "3000")]
    port: u16,
}

#[get("/")]
fn index() -> impl Responder {
    "Hello World"
}

pub mod shared {
    use crate::infrastructure::postgres::connection;
    pub type DBConnection = connection::ConnectionPool;
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let pool: shared::DBConnection = connection::create_connection();

    let options = Options::from_args();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![
                        header::ACCEPT,
                        header::AUTHORIZATION,
                        header::CONTENT_TYPE,
                    ])
                    .max_age(3600),
            )
            .wrap(Logger::new("%a %{User-Agent}i %s $T"))
            .service(index)
            .route("/todos", web::get().to(TodoHandler::list))
            .route("/todos", web::post().to(TodoHandler::create))
            .route("/todos/{id}", web::put().to(TodoHandler::update))
            .route("/todos/{id}", web::delete().to(TodoHandler::delete))
    })
    .bind(format!("0.0.0.0:{}", options.port))?
    .run()
}

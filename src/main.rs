extern crate structopt;
#[macro_use]
extern crate actix_web;
extern crate serde_derive;
extern crate structopt_derive;

pub mod application;
mod domain;
mod handler;
mod infrastructure;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer, Responder};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "poster-server")]
struct Options {
    #[structopt(short = "p", long = "port", default_value = "3000")]
    port: u16
}

#[get("/")]
fn index() -> impl Responder {
    "Hello World"
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let options = Options::from_args();

    let todo_presenter = application::presenter::todo::TodoPresenter {};
    let todo_repository = infrastructure::mock::todo::MockDatastore {};
    let todo_usecase = application::usecase::todo::TodoUsecase {
        todo_presenter,
        todo_repository,
    };
    let todo_controller = application::controller::todo::TodoController { todo_usecase };
    let todo_handler = handler::todo::TodoHandler { todo_controller };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i %s $T"))
            .service(index)
            .route("/todos", web::get().to(move |r| todo_handler.list(r)))
    })
    .bind(format!("127.0.0.1:{}", options.port))?
    .run()
}

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

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer, Responder};
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

    let todo_presenter = application::presenter::todo::TodoPresenter {};
    let todo_repository = infrastructure::postgres::adapter::todo::PgAdapter {};
    let todo_usecase = application::usecase::todo::TodoUsecase {
        todo_presenter,
        todo_repository,
    };
    let todo_controller = application::controller::todo::TodoController { todo_usecase };
    let todo_handler = handler::todo::TodoHandler { todo_controller };

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i %s $T"))
            .service(index)
            .route(
                "/todos",
                web::get()
                    .to(move |pool: web::Data<shared::DBConnection>| todo_handler.list(&pool))

            ).route("/todos", web::post()
                    .to(move |pool: web::Data<shared::DBConnection>, data: web::Json<application::controller::todo::TodoCreateInput>| todo_handler.create(&pool, &data)),
            )
    })
    .bind(format!("0.0.0.0:{}", options.port))?
    .run()
}

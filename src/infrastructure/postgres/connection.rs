use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;
use std::env;

pub type ConnectionPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_connection() -> ConnectionPool {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Pool::builder()
        .build(ConnectionManager::<PgConnection>::new(url))
        .expect("Connection failed to create")
}

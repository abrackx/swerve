#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
extern crate env_logger;
#[macro_use]
extern crate log;

use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

mod schema;
mod models;
mod auth;
mod config;
mod handlers;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool: Pool = config::migrate_and_config_db(&database_url);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .configure(handlers::config_services)
    })
        .bind("127.0.0.1:8888")?
        .run()
        .await
}
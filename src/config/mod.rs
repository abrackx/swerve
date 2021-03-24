use sqlx::{Error, PgPool};
use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use actix_web::dev::HttpResponseBuilder;
use serde::Serialize;

pub type Pool = PgPool;

pub async fn migrate_and_config_db(url: &str) -> Result<Pool, Error> {
    info!("Migrating database...");
    let migrations: Migrator = sqlx::migrate!();
    let pool = PgPoolOptions::new()
        .connect(url)
        .await?;
    migrations.run(&pool).await?;
    Ok(pool)
}

//I don't think this does what I want at all
#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    data: T,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn build(&self, status: StatusCode) -> HttpResponse {
        HttpResponse::build(status).json(&self.data)
    }
}
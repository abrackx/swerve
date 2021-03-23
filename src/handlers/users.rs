use actix_web::{error, Error, get, HttpResponse};
use actix_web::web::{Data, Path};

use crate::config::Pool;
use crate::models::user::User;
use crate::errors::ApiError;

#[get("/users")]
pub async fn get_users(db: Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(db.get_ref())
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|e| error::ErrorInternalServerError(e))?)
}

#[get("/users/{uid}")]
pub async fn get_users_by_id(db: Data<Pool>, uid: Path<i32>) -> Result<HttpResponse, ApiError> {
    Ok(sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(&uid.into_inner())
        .fetch_one(db.get_ref())
        .await
        .map(|user| HttpResponse::Ok().json(user))?)
}
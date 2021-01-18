use actix_web::{Error, get, HttpResponse, post, ResponseError, web};
use actix_web::client::HttpError;

use crate::{
    models::user::User
};
use crate::config::Pool;
use crate::models::project::Project;

#[get("/users")]
pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(db.get_ref())
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

#[get("/users/{uid}")]
pub async fn get_users_by_id(db: web::Data<Pool>, uid: web::Path<i32>) -> Result<HttpResponse, Error> {
    Ok(sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", uid.into_inner())
        .fetch_one(db.get_ref())
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|e| {
            error!("{:?}", e);
            HttpResponse::InternalServerError()
        })?)
}

pub fn config_services(cfg: &mut web::ServiceConfig) {
    info!("Configuring services for {:?}...", std::thread::current().id());
    cfg.service(web::scope("/api")
        .service(get_users)
        .service(get_users_by_id)
    );
}
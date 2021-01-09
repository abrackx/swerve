use std::vec::Vec;

use actix_web::{Error, get, HttpResponse, put, web};
use diesel::dsl::insert_into;

use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

use super::models::User;
use super::Pool;
use super::schema::users::dsl::*;

#[get("/users")]
pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_users(db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

#[get("/users/{id}")]
pub async fn get_user_by_id(
    db: web::Data<Pool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_user_by_id(db, user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

#[put("/users")]
pub async fn upsert_user(
    db: web::Data<Pool>,
    item: web::Json<User>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db_upsert_user(db, item.into_inner()))
        .await
        .map(|user| HttpResponse::Accepted().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn db_get_user_by_id(pool: web::Data<Pool>, user_id: i32) -> Result<User, diesel::result::Error> {
    users.find(Some(user_id)).get_result::<User>(&pool.get().unwrap())
}

fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let items = users.load::<User>(&pool.get().unwrap())?;
    Ok(items)
}

fn db_upsert_user(pool: web::Data<Pool>, user: User) -> Result<Vec<User>, diesel::result::Error> {
    let result = insert_into(users)
        .values(&user)
        .on_conflict(id)
        .do_update()
        .set(&user)
        .get_results(&pool.get().unwrap())?;
    Ok(result)
}

pub fn config_services(cfg: &mut web::ServiceConfig) {
    info!("Configuring services...");
    cfg.service(web::scope("/api")
        .service(get_users)
        .service(get_user_by_id)
        .service(upsert_user)
    );
}
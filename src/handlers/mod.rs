use std::vec::Vec;

use actix_web::{Error, get, HttpResponse, post, web};
use diesel::dsl::insert_into;

use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

use super::Pool;
use super::schema::*;
use diesel::pg::expression::array_comparison::any;
use diesel::ExpressionMethods;
use diesel::prelude::*;
use crate::{
    models::user::User
};
//use crate::models::user::UserDTO;
use crate::models::project::Project;
use crate::models::relations::UserProject;
use crate::schema::users::all_columns;

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
        web::block(move || db_get_projects_by_user_id(db, user_id.into_inner()))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|e| {
                error!("{:?}", e);
                HttpResponse::InternalServerError()
            })?,
    )
}

// #[post("/users")]
// pub async fn add_user(
//     db: web::Data<Pool>,
//     user: web::Json<UserDTO>,
// ) -> Result<HttpResponse, Error> {
//     Ok(
//         web::block(move || db_add_user(db, user.into_inner()))
//             .await
//             .map(|user| HttpResponse::Ok().json(user))
//             .map_err(|_| HttpResponse::InternalServerError())?,
//     )
// }

// fn db_add_user(pool: web::Data<Pool>, user: UserDTO) -> Result<(),()> {
//     diesel::insert_into(users::table).values(&user).execute(&pool.get().unwrap());
//     Ok(())
// }

fn db_get_user_by_id(pool: web::Data<Pool>, user_id: i32) -> Result<User, diesel::result::Error> {
    users::table.find(user_id).get_result::<User>(&pool.get().unwrap())
}

fn db_get_projects_by_user_id(pool: web::Data<Pool>, user_id: i32) -> Result<Project, diesel::result::Error> {
    let conn = &pool.get().unwrap();
    let u: User = users::table.find(user_id).get_result(conn)?;
    let x = UserProject::belonging_to(&u).select(user_project::id);
    project::table
        .filter(project::id.eq(any(x)))
        .get_result::<Project>(conn)
}

fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let items = users::table.load::<User>(&pool.get().unwrap())?;
    Ok(items)
}

pub fn config_services(cfg: &mut web::ServiceConfig) {
    info!("Configuring services for {:?}...", std::thread::current().id());
    cfg.service(web::scope("/api")
        .service(get_users)
        .service(get_user_by_id)
        //.service(add_user)
    );
}
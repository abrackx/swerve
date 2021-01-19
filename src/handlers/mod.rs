use actix_web::{Error, error, get, HttpResponse, post, web};
use actix_web::body::Body;
use actix_web::client::HttpError;
use actix_web::middleware::errhandlers::ErrorHandlerResponse;

use actix_multipart::Multipart;
use crate::{
    models::user::User
};
use crate::config::Pool;
use crate::models::project::Project;
use futures::{TryStreamExt, StreamExt};
use std::io::Write;

#[get("/users")]
pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(db.get_ref())
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|e| error::ErrorInternalServerError(e))?)
}

#[get("/users/{uid}")]
pub async fn get_users_by_id(db: web::Data<Pool>, uid: web::Path<i32>) -> Result<HttpResponse, Error> {
    Ok(sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(&uid.into_inner())
        .fetch_one(db.get_ref())
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|e| error::ErrorInternalServerError(e))?)
}

#[get("/projects/users/{uid}")]
pub async fn get_projects_by_uid(db: web::Data<Pool>, uid: web::Path<i32>) -> Result<HttpResponse, Error> {
    Ok(sqlx::query_as::<_, Project>("
        SELECT projects.*
        FROM projects
        JOIN user_projects ON projects.id = user_projects.project_id
        WHERE user_projects.user_id = $1
        ").bind(&uid.into_inner())
        .fetch_all(db.get_ref())
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|e| error::ErrorInternalServerError(e))?)
}

#[get("/projects/{project_id}")]
pub async fn get_projects_by_id(db: web::Data<Pool>, project_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    Ok(sqlx::query_as::<_, Project>("
        SELECT projects.*
        FROM projects
        WHERE id = $1
        ").bind(&project_id.into_inner())
        .fetch_all(db.get_ref())
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|e| error::ErrorInternalServerError(e))?)
}

#[post("/projects/{project_id}")]
//this doesn't work as intended /shrug
async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("./tmp/{}", sanitize_filename::sanitize(&filename));

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();
        info!("{:?}",f);
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }
    Ok(HttpResponse::Ok().into())
}


pub fn init(cfg: &mut web::ServiceConfig) {
    info!("Configuring services for {:?}...", std::thread::current().id());
    cfg.service(web::scope("/api")
        .service(get_users)
        .service(get_users_by_id)
        .service(get_projects_by_uid)
        .service(get_projects_by_id)
        .service(save_file)
    );
}
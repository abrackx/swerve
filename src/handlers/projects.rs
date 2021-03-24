use actix_web::{error, Error, get, HttpResponse};
use actix_web::web::{Data, Path};

use crate::config::Pool;
use crate::errors::ApiError;
use crate::models::project::{get_project_by_id, get_project_by_uid, Project};

#[get("/projects/users/{uid}")]
pub async fn get_projects_by_uid(db: Data<Pool>, uid: Path<i32>) -> Result<HttpResponse, ApiError> {
    Ok(get_project_by_uid(db.get_ref(), uid.into_inner())
        .await
        .map(|project| HttpResponse::Ok().json(project))?)
}

#[get("/projects/{project_id}")]
pub async fn get_projects_by_id(db: Data<Pool>, project_id: Path<i32>) -> Result<HttpResponse, ApiError> {
    Ok(get_project_by_id(db.get_ref(), project_id.into_inner())
        .await
        .map(|project| HttpResponse::Ok().json(project))?)
}
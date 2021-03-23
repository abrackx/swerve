use actix_web::{error, Error, get, HttpResponse};
use actix_web::web::{Data, Path};
use anyhow::Result;

use crate::config::Pool;
use crate::models::project::{get_project_by_uid, Project};

#[get("/projects/users/{uid}")]
pub async fn get_projects_by_uid(db: Data<Pool>, uid: Path<i32>) -> Result<HttpResponse, Error>{
    Ok(get_project_by_uid(db, uid.into_inner())
        .await
        .map(|project| HttpResponse::Ok().json(project))
        .map_err(|e| error::ErrorInternalServerError(e))?)
}

#[get("/projects/{project_id}")]
pub async fn get_projects_by_id(db: Data<Pool>, project_id: Path<i32>) -> Result<HttpResponse, Error> {
    Ok(sqlx::query_as::<_, Project>("
            SELECT projects.*
            FROM projects
            WHERE id = $1
        ").bind(&project_id.into_inner())
        .fetch_all(db.get_ref())
        .await
        .map(|project| HttpResponse::Ok().json(project))
        .map_err(|e| error::ErrorInternalServerError(e))?)
}
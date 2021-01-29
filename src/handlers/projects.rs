use actix_web::{error, Error, get, HttpResponse};
use actix_web::web::{Data, Path};

use crate::config::Pool;
use crate::models::project::Project;

#[get("/projects/users/{uid}")]
pub async fn get_projects_by_uid(db: Data<Pool>, uid: Path<i32>) -> Result<HttpResponse, Error> {
    Ok(sqlx::query_as::<_, Project>("
            SELECT projects.*
            FROM projects
            JOIN user_projects ON projects.id = user_projects.project_id
            WHERE user_projects.user_id = $1
        ").bind(&uid.into_inner())
        .fetch_all(db.get_ref())
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
use actix_web::web::Data;
use anyhow::Result;
use chrono::NaiveDateTime;

use crate::config::Pool;

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

pub async fn get_project_by_uid(db: Data<Pool>, uid: i32) -> Result<Vec<Project>> {
    let res = sqlx::query_as::<_, Project>("
            SELECT projects.*
            FROM projects
            JOIN user_projects ON projects.id = user_projects.project_id
            WHERE user_projects.user_id = $1
    ")
        .bind(uid)
        .fetch_all(db.get_ref())
        .await?;
    Ok(res)
}
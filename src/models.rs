use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset, Associations, Identifiable)]
#[table_name = "users"]
pub struct User {
    pub id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset, Associations, Identifiable)]
#[table_name = "project"]
pub struct Project {
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset, Associations, Identifiable)]
#[table_name = "tag"]
pub struct Tag {
    pub id: Option<i32>,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset, Associations, Identifiable)]
#[table_name = "file"]
pub struct File {
    pub id: Option<i32>,
    pub name: String,
    pub sort: i32,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset, Associations, Identifiable)]
#[table_name = "image"]
pub struct Image {
    pub id: Option<i32>,
    pub name: String,
    pub sort: i32,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset, Associations, Identifiable)]
#[table_name = "project_file"]
#[belongs_to(Project)]
#[belongs_to(File)]
pub struct ProjectFile {
    pub id: Option<i32>,
    pub project_id: Option<i32>,
    pub file_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset, Associations, Identifiable)]
#[table_name = "project_image"]
#[belongs_to(Project)]
#[belongs_to(Image)]
pub struct ProjectImage {
    pub id: Option<i32>,
    pub project_id: Option<i32>,
    pub image_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset, Associations, Identifiable)]
#[table_name = "project_tag"]
#[belongs_to(Project)]
#[belongs_to(Tag)]
pub struct ProjectTag {
    pub id: Option<i32>,
    pub project_id: Option<i32>,
    pub tag_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset, Associations, Identifiable)]
#[table_name = "user_project"]
#[belongs_to(User)]
#[belongs_to(Project)]
pub struct UserProject {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub project_id: Option<i32>,
}
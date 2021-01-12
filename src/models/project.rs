use crate::schema::project::{self, dsl::*};

#[derive(Debug, Serialize, Deserialize)]
#[derive(Identifiable, Queryable, Associations)]
#[table_name = "project"]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: chrono::NaiveDateTime,
    pub modified_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(Insertable, AsChangeset)]
#[table_name = "project"]
pub struct ProjectDTO {
    pub name: String,
    pub description: String,
    pub created_at: chrono::NaiveDateTime,
    pub modified_at: chrono::NaiveDateTime,
}
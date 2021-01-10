use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "users"]
pub struct User {
    pub id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "project"]
pub struct Project {
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "tag"]
pub struct Tag {
    pub id: Option<i32>,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "file"]
pub struct File {
    pub id: Option<i32>,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}
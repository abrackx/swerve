use crate::schema::users::{self, dsl::*};

#[derive(Debug, Serialize, Deserialize)]
#[derive(Queryable, Identifiable, Associations)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(Queryable, Insertable, AsChangeset)]
#[table_name = "users"]
pub struct UserDTO {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}
use crate::schema::user_project::{self, dsl::*};

#[derive(Identifiable, Queryable, Associations)]
#[table_name = "user_project"]
#[belongs_to(crate::models::user::User)]
#[belongs_to(crate::models::project::Project)]
pub struct UserProject {
    pub id: i32,
    pub user_id: i32,
    pub project_id: i32,
}
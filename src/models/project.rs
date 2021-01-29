use chrono::NaiveDateTime;

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}
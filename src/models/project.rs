#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: chrono::NaiveDateTime,
    pub modified_at: chrono::NaiveDateTime,
}
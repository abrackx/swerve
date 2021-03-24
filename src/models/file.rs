use chrono::NaiveDateTime;

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct File {
    pub id: i32,
    pub name: String,
    pub sort: i32,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

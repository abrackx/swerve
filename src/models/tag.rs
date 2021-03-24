use chrono::NaiveDateTime;

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
}

pub mod user;
pub mod project;

//

//
// #[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset, Associations, Identifiable)]
// #[table_name = "tag"]
// pub struct Tag {
//     pub id: Option<i32>,
//     pub name: String,
//     pub created_at: NaiveDateTime,
// }
//
// #[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset, Associations, Identifiable)]
// #[table_name = "file"]
// pub struct File {
//     pub id: Option<i32>,
//     pub name: String,
//     pub sort: i32,
//     pub created_at: NaiveDateTime,
//     pub modified_at: NaiveDateTime,
// }
//
// #[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset, Associations, Identifiable)]
// #[table_name = "image"]
// pub struct Image {
//     pub id: Option<i32>,
//     pub name: String,
//     pub sort: i32,
//     pub created_at: NaiveDateTime,
//     pub modified_at: NaiveDateTime,
// }
//
// #[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset, Associations, Identifiable)]
// #[table_name = "project_file"]
// #[belongs_to(Project)]
// #[belongs_to(File)]
// pub struct ProjectFile {
//     pub id: Option<i32>,
//     pub project_id: i32,
//     pub file_id: i32,
// }
//
// #[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset, Associations, Identifiable)]
// #[table_name = "project_image"]
// #[belongs_to(Project)]
// #[belongs_to(Image)]
// pub struct ProjectImage {
//     pub id: Option<i32>,
//     pub project_id: Option<i32>,
//     pub image_id: Option<i32>,
// }
//
// #[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset, Associations, Identifiable)]
// #[table_name = "project_tag"]
// #[belongs_to(Project)]
// #[belongs_to(Tag)]
// pub struct ProjectTag {
//     pub id: Option<i32>,
//     pub project_id: Option<i32>,
//     pub tag_id: Option<i32>,
// }
//

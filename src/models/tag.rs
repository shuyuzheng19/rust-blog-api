use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// #[derive(Debug, serde::Serialize, serde::Deserialize)]
// pub struct Tag {
//     pub id: i32,
//     pub name: String,
//     pub create_at: NaiveDateTime,
//     pub update_at: NaiveDateTime,
//     pub deleted_at: Option<NaiveDateTime>,
// }

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TagVo {
    pub id: i64,
    pub name: String,
}

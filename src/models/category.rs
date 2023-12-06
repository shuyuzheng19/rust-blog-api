use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
use sqlx::types::chrono::{DateTime, Local};

use crate::common::date_format;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CategoryVo {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, FromRow, Deserialize)]
pub struct OtherAdminVo {
    pub id: i64,
    pub name: String,
    #[serde(with = "date_format", rename = "createAt")]
    pub create_at: DateTime<Local>,
    #[serde(with = "date_format", rename = "updateAt")]
    pub update_at: DateTime<Local>,
}

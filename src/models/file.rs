use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Row};
use sqlx::postgres::PgRow;
use sqlx::types::chrono::{DateTime, Local};

use crate::common::{date_format, get_size_str};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct FileInfo {
    pub id: i64,
    pub user_id: i64,
    pub old_name: String,
    pub new_name: String,
    #[serde(with = "date_format")]
    pub create_at: DateTime<Local>,
    pub size: i64,
    pub suffix: String,
    pub absolute_path: String,
    pub is_public: bool,
    pub md5: String,
    pub url: String,
}

impl FileInfo {
    pub fn new() -> Self {
        return Self {
            id: 0,
            user_id: 0,
            old_name: "".to_string(),
            new_name: "".to_string(),
            create_at: Default::default(),
            size: 0,
            suffix: "".to_string(),
            absolute_path: "".to_string(),
            is_public: false,
            md5: "".to_string(),
            url: "".to_string(),
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileVo {
    pub id: i64,
    pub name: String,
    #[serde(with = "date_format", rename = "dateStr")]
    pub date_str: DateTime<Local>,
    pub suffix: String,
    #[serde(rename = "sizeStr")]
    pub size_str: String,
    pub md5: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileCheckRequest {
    pub name: String,
    pub md5: String,
}

impl<'c> FromRow<'c, PgRow> for FileVo {
    fn from_row(row: &'c PgRow) -> Result<Self, Error> {
        let size: i64 = row.get("size");
        let size_str = get_size_str(size as f64);

        Ok(FileVo {
            id: row.get("id"),
            name: row.get("old_name"),
            date_str: row.get("create_at"),
            suffix: row.get("suffix"),
            size_str,
            md5: row.get("md5"),
            url: row.get("url"),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct FileAdminVo {
    pub id: i64,
    pub name: String,
    pub size: i64,
    pub uid: i64,
    pub url: String,
    pub md5: String,
    #[serde(with = "date_format", rename = "createAt")]
    pub create_at: DateTime<Local>,
    pub public: bool,
}

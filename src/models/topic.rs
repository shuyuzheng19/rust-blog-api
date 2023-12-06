use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Row};
use sqlx::postgres::PgRow;
use sqlx::types::chrono::{DateTime, Local};

use crate::common::date_format;
use crate::models::user::SimpleUserVo;

#[derive(Serialize, Deserialize, Debug)]
pub struct TopicVo {
    pub id: i64,
    pub name: String,
    pub description: String,
    #[serde(rename = "cover")]
    pub cover_image: String,
    pub user: SimpleUserVo,
    #[serde(rename = "timeStamp")]
    pub time_stamp: i64,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct SimpleTopicVo {
    pub id: i64,
    pub name: String,
}

impl<'c> FromRow<'c, PgRow> for TopicVo {
    fn from_row(row: &'c PgRow) -> Result<Self, Error> {
        if row.is_empty() {
            return Err(Error::RowNotFound);
        }
        let date: DateTime<Local> = row.get("create_at");
        Ok(TopicVo {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            cover_image: row.get("cover_image"),
            user: SimpleUserVo {
                id: row.get("u_id"),
                nick_name: row.get("u_nick_name"),
            },
            time_stamp: date.timestamp_millis(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserSimpleTopicVo {
    pub id: i64,
    pub name: String,
    pub description: String,
    #[serde(rename = "cover")]
    pub cover_image: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopicRequest {
    pub id: Option<i64>,
    pub name: String,
    pub desc: String,
    pub cover: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminTopicVo {
    pub id: i64,
    pub name: String,
    pub description: String,
    #[serde(rename = "createAt", with = "date_format")]
    pub create_at: DateTime<Local>,
    #[serde(rename = "updateAt", with = "date_format")]
    pub update_at: DateTime<Local>,
    #[serde(rename = "coverImage")]
    pub cover_image: String,
    pub user: SimpleUserVo,
}

impl<'c> FromRow<'c, PgRow> for AdminTopicVo {
    fn from_row(row: &'c PgRow) -> Result<Self, Error> {
        if row.is_empty() {
            return Err(Error::RowNotFound);
        }
        Ok(AdminTopicVo {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            create_at: row.get("create_at"),
            update_at: row.get("update_at"),
            cover_image: row.get("cover_image"),
            user: SimpleUserVo {
                id: row.get("u_id"),
                nick_name: row.get("u_nick_name"),
            },
        })
    }
}

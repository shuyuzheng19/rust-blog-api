use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Row};
use sqlx::postgres::PgRow;
use sqlx::types::chrono::{DateTime, Local};

use crate::common::date_format;
use crate::models::category::CategoryVo;
use crate::models::tag::TagVo;
use crate::models::topic::SimpleTopicVo;
use crate::models::user::SimpleUserVo;

#[derive(Debug)]
pub struct Blog {
    pub id: i64,
    pub description: String,
    pub title: String,
    pub cover_image: String,
    pub source_url: Option<String>,
    pub content: String,
    pub eye_count: i64,
    pub like_count: i64,
    pub markdown: bool,
    pub create_at: DateTime<Local>,
    pub update_at: DateTime<Local>,
    pub category_id: Option<i32>,
    pub user_id: i32,
    pub topic_id: Option<i32>,
    pub deleted_at: Option<DateTime<Local>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlogVo {
    pub id: i64,
    pub title: String,
    pub desc: String,
    #[serde(rename = "coverImage")]
    pub cover_image: String,
    #[serde(rename = "timeStamp")]
    pub time_stamp: i64,
    pub category: CategoryVo,
    pub user: SimpleUserVo,
}

impl<'c> FromRow<'c, PgRow> for BlogVo {
    fn from_row(row: &'c PgRow) -> Result<Self, Error> {
        if row.is_empty() {
            return Err(Error::RowNotFound);
        }
        let date: DateTime<Local> = row.get("create_at");
        Ok(BlogVo {
            id: row.get("id"),
            title: row.get("title"),
            desc: row.get("description"),
            cover_image: row.get("cover_image"),
            time_stamp: date.timestamp_millis(),
            category: CategoryVo {
                id: row.get("c_id"),
                name: row.get("c_name"),
            },
            user: SimpleUserVo {
                id: row.get("u_id"),
                nick_name: row.get("u_nick_name"),
            },
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopicBlogVo {
    pub id: i64,
    pub title: String,
    pub desc: String,
    #[serde(rename = "coverImage")]
    pub cover_image: String,
    #[serde(rename = "timeStamp")]
    pub time_stamp: i64,
    pub user: SimpleUserVo,
}

impl<'c> FromRow<'c, PgRow> for TopicBlogVo {
    fn from_row(row: &'c PgRow) -> Result<Self, Error> {
        if row.is_empty() {
            return Err(Error::RowNotFound);
        }
        let date: DateTime<Local> = row.get("create_at");
        Ok(TopicBlogVo {
            id: row.get("id"),
            title: row.get("title"),
            desc: row.get("description"),
            cover_image: row.get("cover_image"),
            time_stamp: date.timestamp_millis(),
            user: SimpleUserVo {
                id: row.get("u_id"),
                nick_name: row.get("u_nick_name"),
            },
        })
    }
}

#[derive(Debug, Serialize, FromRow, Deserialize)]
pub struct SimpleBlogVo {
    pub id: i64,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RecommendBlogVo {
    pub id: i64,
    pub title: String,
    #[serde(rename = "coverImage")]
    pub cover_image: String,
}

#[derive(Debug, Serialize, FromRow, Deserialize)]
pub struct SearchBlogVo {
    pub id: i64,
    pub description: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlogContentVo {
    pub id: i64,
    pub description: String,
    pub title: String,
    #[serde(rename = "coverImage")]
    cover_image: String,
    #[serde(rename = "source_url")]
    pub source_url: String,
    pub content: String,
    #[serde(rename = "eyeCount")]
    pub eye_count: i64,
    #[serde(rename = "likeCount")]
    pub like_count: i64,
    pub category: Option<CategoryVo>,
    pub topic: Option<SimpleTopicVo>,
    pub tags: Vec<TagVo>,
    pub user: SimpleUserVo,
    #[serde(with = "date_format", rename = "createTime")]
    pub create_time: DateTime<Local>,
    #[serde(with = "date_format", rename = "updateTime")]
    pub update_time: DateTime<Local>,
}

impl<'c> FromRow<'c, PgRow> for BlogContentVo {
    fn from_row(row: &'c PgRow) -> Result<Self, Error> {
        if row.is_empty() {
            return Err(Error::RowNotFound);
        }

        let mut category: Option<CategoryVo> = None;

        let mut topic: Option<SimpleTopicVo> = None;

        let cid: Option<i64> = row.get("c_id");

        let tid: Option<i64> = row.get("t_id");

        if cid != None {
            category = Some(CategoryVo {
                id: cid.unwrap(),
                name: row.get("c_name"),
            })
        }

        if tid != None {
            topic = Some(SimpleTopicVo {
                id: tid.unwrap(),
                name: row.get("t_name"),
            })
        }

        Ok(BlogContentVo {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            cover_image: row.get("cover_image"),
            source_url: row.get("source_url"),
            content: row.get("content"),
            eye_count: row.get("eye_count"),
            create_time: row.get("create_at"),
            category,
            topic,
            tags: vec![],
            user: SimpleUserVo {
                id: row.get("u_id"),
                nick_name: row.get("u_nick_name"),
            },
            like_count: 0,
            update_time: row.get("update_at"),
        })
    }
}

#[derive(Debug, Serialize, FromRow)]
pub struct ArchiveBlogVo {
    pub id: i64,
    pub title: String,
    #[serde(rename = "desc")]
    pub description: String,
    #[serde(with = "date_format")]
    pub create: DateTime<Local>,
}

#[derive(Serialize, Debug)]
pub struct BlogAdminVo {
    pub id: i64,
    pub title: String,
    pub description: String,
    #[serde(rename = "coverImage")]
    pub cover_image: String,
    #[serde(rename = "eyeCount")]
    pub eye_count: i64,
    #[serde(rename = "likeCount")]
    pub like_count: i64,
    pub category: Option<CategoryVo>,
    pub topic: Option<SimpleTopicVo>,
    #[serde(with = "date_format", rename = "createAt")]
    pub create_at: DateTime<Local>,
    pub original: bool,
    pub user: SimpleUserVo,
}

impl<'c> FromRow<'c, PgRow> for BlogAdminVo {
    fn from_row(row: &'c PgRow) -> Result<Self, Error> {
        let mut category: Option<CategoryVo> = None;

        let mut topic: Option<SimpleTopicVo> = None;

        let c_id: Option<i64> = row.get("c_id");

        let t_id: Option<i64> = row.get("t_id");

        if let Some(cid) = c_id {
            category = Some(CategoryVo {
                id: cid,
                name: row.get("c_name"),
            })
        } else if let Some(tid) = t_id {
            topic = Some(SimpleTopicVo {
                id: tid,
                name: row.get("t_name"),
            })
        }

        let is_source: Option<String> = row.get("source_url");

        return Ok(Self {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            cover_image: row.get("cover_image"),
            eye_count: row.get("eye_count"),
            like_count: row.get("like_count"),
            category: category,
            topic: topic,
            create_at: row.get("create_at"),
            original: is_source == None,
            user: SimpleUserVo {
                id: row.get("u_id"),
                nick_name: row.get("u_nick_name"),
            },
        });
    }
}

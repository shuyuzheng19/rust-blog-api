use std::time::Duration;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::chrono::{DateTime, Local};

use crate::common::constants::default_page;
use crate::common::date_format::time_stamp_to_date;
use crate::common::is_image_url;

fn default_sort() -> Sort {
    return Sort::CREATE;
}

fn default_cid() -> i64 {
    -1
}
#[derive(Deserialize, Debug)]
pub struct BlogFindRequest {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_cid")]
    pub cid: i64,
    #[serde(default = "default_sort")]
    pub sort: Sort,
}

#[derive(Deserialize, Debug)]
pub enum Sort {
    CREATE,
    UPDATE,
    EYE,
    LIKE,
    BACK,
}

impl Sort {
    pub fn to_order_by_string(&self, prefix: String) -> String {
        return match self {
            Sort::CREATE => String::from(prefix + "create_at desc"),
            Sort::UPDATE => String::from(prefix + "update_at desc"),
            Sort::EYE => String::from(prefix + "eye_count desc"),
            Sort::LIKE => String::from(prefix + "like_count desc"),
            Sort::BACK => String::from(prefix + "create_at asc"),
            _ => String::from(prefix + "create_at desc"),
        };
    }
}

pub struct ArchiveRange(pub i64, pub DateTime<Local>, pub DateTime<Local>);

fn default_start_date() -> i64 {
    8
}

fn default_end_date() -> i64 {
    8
}

#[derive(Deserialize, Debug)]
pub struct ArchiveRangeRequest {
    #[serde(default = "default_page")]
    page: i64,
    start: i64,
    end: i64,
}

impl ArchiveRangeRequest {
    pub fn get_range_request(&self) -> ArchiveRange {
        let start_date = time_stamp_to_date(self.start);

        let end_date = time_stamp_to_date(self.end)+Duration::from_secs(60*60*24);

        return ArchiveRange(self.page, start_date, end_date);
    }
}

#[derive(Deserialize, Debug)]
pub struct GetUserBlogRequest {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_sort")]
    pub sort: Sort,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct SearchQueryRequest {
    #[serde(default = "default_page")]
    pub page: i64,
    pub keyword: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct BlogRequest {
    pub id: Option<i64>,
    pub title: String,
    pub description: String,
    pub content: String,
    #[serde(rename = "sourceUrl")]
    pub source_url: String,
    #[serde(rename = "coverImage")]
    pub cover_image: String,
    #[sqlx(skip)]
    pub tags: Vec<i64>,
    pub topic: Option<i64>,
    pub category: Option<i64>,
}

impl BlogRequest {
    pub fn check(&self) -> Option<String> {
        let title_len = self.title.chars().count();

        let desc_len = self.description.chars().count();

        if title_len < 1 || title_len > 50 {
            return Some(String::from("博客标题不能小于1个字符并且不能大于50个字符"));
        } else if desc_len < 1 || desc_len > 200 {
            return Some(String::from("博客简介不能小于1个字符并且不能大于200个字符"));
        } else if self.content.is_empty() {
            return Some(String::from("博客内容不能为空"));
        } else if !is_image_url(&self.cover_image) {
            return Some(String::from("这不是一个图片链接"));
        }

        if self.topic == None && self.category == None {
            return Some(String::from("分类和专题要选择一个"));
        }

        return None;
    }
}

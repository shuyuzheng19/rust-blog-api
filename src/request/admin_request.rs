use serde::Deserialize;

use crate::common::constants::default_page;
use crate::request::blog_request::Sort;

#[derive(Deserialize, Debug)]
pub struct AdminBlogFilter {
    #[serde(default = "default_page")]
    pub page: i64,
    pub sort: Sort,
    pub category: Option<i64>,
    pub start: Option<i64>,
    pub end: Option<i64>,
    pub original: Option<bool>,
    pub topic: Option<i64>,
    pub keyword: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct OtherAdminFilter {
    #[serde(default = "default_page")]
    pub page: i64,
    pub sort: FileSort,
    pub deleted: bool,
    pub start: Option<i64>,
    pub end: Option<i64>,
    pub keyword: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct UpdatePublicRequest {
    pub is_pub: bool,
    pub id: i64,
}

#[derive(Deserialize, Debug)]
pub struct UpdateRole {
    pub role: String,
    pub username: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateGpt {
    pub t:String,
    pub keyword: String,
}

#[derive(Deserialize, Debug)]
pub enum CookieType {
    MUSIC,
    BILI,
    XHS,
}

#[derive(Deserialize, Debug)]
pub struct UpdateCookie {
    pub t: CookieType,
    pub cookie: String,
}

#[derive(Deserialize, Debug)]
pub enum FileSort {
    CREATE,
    BACK,
    SIZE,
    SBACK,
}

impl FileSort {
    pub fn to_order_by_string(&self, prefix: String) -> String {
        return match self {
            FileSort::CREATE => String::from(prefix + "create_at desc"),
            FileSort::BACK => String::from(prefix + "create_at asc"),
            FileSort::SIZE => String::from(prefix + "size desc"),
            FileSort::SBACK => String::from(prefix + "size asc"),
            _ => String::from(prefix + "create_at desc"),
        };
    }
}

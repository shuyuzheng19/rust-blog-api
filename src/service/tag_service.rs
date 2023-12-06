use std::sync::Arc;

use r2d2_redis::redis::Commands;
use sqlx::{Pool, Postgres};

use crate::common::constants::TAG_RANDOM_LIST_COUNT;
use crate::common::redis_keys::{RANDOM_TAG_KEY, TAG_MAP_KEY};
use crate::conf::redis_config::get_pool_connection;
use crate::models::blogs::BlogVo;
use crate::models::tag::TagVo;
use crate::repository::tag_repository::TagRepository;
use crate::response::page_info::PageInfo;

pub struct TagService(Arc<TagRepository>);

impl TagService {
    pub fn new(db_conn: Pool<Postgres>) -> TagService {
        let tag_repository = TagRepository::new(db_conn);
        TagService(Arc::new(tag_repository))
    }

    // 获取标签列表
    pub async fn get_category_list(&self) -> Vec<TagVo> {
        // 从数据库中获取标签列表
        self.0.get_tag_list().await
    }

    // 添加标签
    pub async fn add_tag(&self, name: &String) -> Option<TagVo> {
        // 添加标签到数据库
        return self.0.add_tag(name).await;
    }

    // 获取随机标签
    pub async fn get_random_tag(&self) -> Vec<TagVo> {
        let result = get_random_tag();
        return if result.is_ok() {
            result.unwrap()
        } else {
            // 从数据库中获取标签列表
            let tag_list = self.0.get_tag_list().await;
            for tag in &tag_list {
                let json_str = serde_json::to_string(tag).unwrap();
                get_pool_connection()
                    .sadd::<String, String, usize>(RANDOM_TAG_KEY.to_owned(), json_str)
                    .unwrap();
            }
            let result = get_random_tag();
            if result.is_ok() {
                result.unwrap()
            } else {
                vec![]
            }
        };
    }

    // 通过标签ID获取标签信息
    pub async fn get_tag_by_id(&self, tid: i64) -> Option<TagVo> {
        let result = get_pool_connection().hget::<String, i64, String>(TAG_MAP_KEY.to_owned(), tid);
        return if let Ok(r) = result {
            let redis_tag = serde_json::from_str(&r).unwrap();
            Some(redis_tag)
        } else {
            // 从数据库中获取标签信息
            let result = self.0.get_tag_by_id(tid).await;
            if let Some(tag) = &result {
                let json_str = serde_json::to_string(tag).unwrap();
                let _ = get_pool_connection()
                    .hset::<String, i64, String, String>(TAG_MAP_KEY.to_owned(), tid, json_str)
                    .is_err();
            }
            result
        };
    }

    // 获取带有标签的博客列表
    pub async fn get_tag_blog_list(&self, page: i64, tid: i64) -> PageInfo<BlogVo> {
        // 从数据库中获取带有标签的博客列表
        self.0.get_tag_blogs(page, tid).await
    }
}

fn get_random_tag() -> Result<Vec<TagVo>, bool> {
    let result = get_pool_connection()
        .srandmember_multiple::<String, Vec<String>>(
            RANDOM_TAG_KEY.to_owned(),
            TAG_RANDOM_LIST_COUNT,
        )
        .unwrap();
    if result.is_empty() {
        return Err(true);
    }
    let mut list: Vec<TagVo> = Vec::new();
    for str in result {
        list.push(serde_json::from_str(&str).unwrap());
    }
    return Ok(list);
}

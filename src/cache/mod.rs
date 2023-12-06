use r2d2_redis::redis::Commands;
use serde::{Deserialize, Serialize};

use crate::common::redis_keys::{
    BLOG_LIST_PAGE_INFO_KEY, CATEGORY_LIST_KEY, FIRST_PAGE_TOPIC_KEY, RANDOM_TAG_KEY, TAG_MAP_KEY,
    TOPIC_MAP_KEY, USER_INFO_KEY,
};
use crate::conf::redis_config::get_pool_connection;

pub mod blog_cache;
pub(crate) mod user_cache;

// 清除页面信息的 Redis 键
pub fn clear_page_info_keys() {
    let keys: Vec<String> = get_pool_connection()
        .keys(BLOG_LIST_PAGE_INFO_KEY.to_owned() + "*")
        .unwrap();
    for key in keys {
        let _ = get_pool_connection().del::<String, i64>(key);
    }
}

pub fn clear_category_info_keys() {
    get_pool_connection()
        .del::<&str, i64>(CATEGORY_LIST_KEY)
        .expect("删除分类列表失败");
}

pub fn clear_tag_info_key() {
    get_pool_connection()
        .del::<&str, i64>(RANDOM_TAG_KEY)
        .expect("删除标签列表失败");
    get_pool_connection()
        .del::<&str, i64>(TAG_MAP_KEY)
        .expect("删除标签Map列表失败");
}

pub fn clear_topic_info_key() {
    get_pool_connection()
        .del::<&str, i64>(FIRST_PAGE_TOPIC_KEY)
        .expect("删除第一页专题失败");
    get_pool_connection()
        .del::<&str, i64>(TOPIC_MAP_KEY)
        .expect("删除专题MAP失败");
}

pub fn clear_user_info(username: &String) {
    get_pool_connection()
        .del::<String, i64>(USER_INFO_KEY.to_owned() + username)
        .unwrap();
}

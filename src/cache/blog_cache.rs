use std::collections::HashMap;

use r2d2_redis::redis::{Commands, RedisError, RedisResult};

use crate::common::redis_keys::{
    BLOG_MAP_KEY, EYE_COUNT_MAP, HOT_BLOG_KEY, HOT_BLOG_KEY_EXPIRE, LATEST_BLOG_KEY,
    LATEST_BLOG_KEY_EXPIRE, RECOMMEND_BLOG_KEY, SAVE_BLOG_MAP,
};
use crate::conf::redis_config::get_pool_connection;
use crate::models::blogs::{BlogContentVo, RecommendBlogVo, SimpleBlogVo};

// 博客缓存结构
pub struct BlogCache {}

impl BlogCache {
    pub fn new() -> BlogCache {
        return BlogCache {};
    }

    // 设置热门博客到 Redis
    pub fn set_hot_blog(&self, blogs: Option<&Vec<SimpleBlogVo>>) -> bool {
        return get_pool_connection()
            .set_ex::<&str, String, String>(
                HOT_BLOG_KEY,
                serde_json::to_string(&blogs).unwrap(),
                HOT_BLOG_KEY_EXPIRE,
            )
            .is_ok();
    }

    // 从 Redis 获取热门博客
    pub fn get_hot_blog(&self) -> Option<Vec<SimpleBlogVo>> {
        let result: RedisResult<String> = get_pool_connection().get(HOT_BLOG_KEY);
        return match result {
            Ok(r) => Some(serde_json::from_str(&r).unwrap()),
            Err(e) => None,
        };
    }

    // 设置最新博客到 Redis
    pub fn set_latest_blog(&self, blogs: Option<&Vec<SimpleBlogVo>>) -> bool {
        return get_pool_connection()
            .set_ex::<&str, String, String>(
                LATEST_BLOG_KEY,
                serde_json::to_string(&blogs).unwrap(),
                LATEST_BLOG_KEY_EXPIRE,
            )
            .is_ok();
    }

    // 从 Redis 获取最新博客
    pub fn get_latest_blog(&self) -> Option<Vec<SimpleBlogVo>> {
        let result: RedisResult<String> = get_pool_connection().get(LATEST_BLOG_KEY);
        return match result {
            Ok(r) => Some(serde_json::from_str(&r).unwrap()),
            Err(e) => None,
        };
    }

    // 设置博客信息到 Redis
    pub fn set_blog_info(&self, id: i64, blog: &Option<BlogContentVo>) -> bool {
        return get_pool_connection()
            .hset::<&str, i64, String, String>(
                BLOG_MAP_KEY,
                id,
                serde_json::to_string(blog).unwrap(),
            )
            .is_ok();
    }

    // 根据 ID 从 Redis 删除博客信息
    pub fn delete_blog_info_by_id(&self, id: i64) -> bool {
        return get_pool_connection()
            .hdel::<&str, i64, String>(BLOG_MAP_KEY, id)
            .is_ok();
    }

    // 增加博客的浏览次数
    pub fn increase_in_view(&self, default_count: i64, id: i64) -> i64 {
        let flag: bool = get_pool_connection().hexists(EYE_COUNT_MAP, id).unwrap();

        let count: i64 = if flag {
            get_pool_connection()
                .hincr::<&str, i64, i64, i64>(EYE_COUNT_MAP, id, 1)
                .unwrap()
        } else {
            default_count + 1
        };

        if !flag {
            get_pool_connection()
                .hset::<&str, i64, i64, i64>(EYE_COUNT_MAP, id, count)
                .unwrap();
        }

        return count;
    }

    // 从 Redis 获取博客信息
    pub fn get_blog_info(&self, id: i64) -> Result<Option<BlogContentVo>, RedisError> {
        let result: RedisResult<String> = get_pool_connection().hget(BLOG_MAP_KEY, id);
        return match result {
            Ok(r) => {
                let result = serde_json::from_str(&r);
                Ok(match result {
                    Ok(r) => Some(r),
                    Err(e) => None,
                })
            }
            Err(e) => Err(e),
        };
    }

    // 从 Redis 获取推荐博客
    pub fn get_recommend_blog(&self) -> Vec<RecommendBlogVo> {
        let result = get_pool_connection().get::<&str, String>(RECOMMEND_BLOG_KEY);
        return match result {
            Ok(r) => serde_json::from_str(&r).unwrap(),
            Err(e) => vec![],
        };
    }

    // 设置推荐博客到 Redis
    pub fn set_recommend_blog(&self, vec: &Vec<RecommendBlogVo>) -> bool {
        let vec_json = serde_json::to_string(vec).unwrap();
        return get_pool_connection()
            .set::<&str, String, String>(RECOMMEND_BLOG_KEY, vec_json)
            .is_ok();
    }

    // 设置保存或编辑的博客到 Redis
    pub fn set_save_edit_blog(&self, u_id: i64, content: &String) -> bool {
        return get_pool_connection()
            .hset::<&str, i64, &String, String>(SAVE_BLOG_MAP, u_id, content)
            .is_err();
    }

    // 从 Redis 获取保存或编辑的博客
    pub fn get_save_edit_blog(&self, u_id: i64) -> Option<String> {
        let result = get_pool_connection().hget::<&str, i64, String>(SAVE_BLOG_MAP, u_id);
        if let Ok(r) = result {
            return Some(r);
        } else {
            return None;
        }
    }

    pub fn get_blog_map_values(&self) -> HashMap<i64, i64> {
        let result = get_pool_connection()
            .hgetall::<&str, HashMap<i64, i64>>(EYE_COUNT_MAP)
            .unwrap();
        return result;
    }
}

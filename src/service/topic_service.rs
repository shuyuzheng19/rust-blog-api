use std::sync::Arc;

use r2d2_redis::redis::Commands;
use sqlx::{Pool, Postgres};

use crate::common::is_image_url;
use crate::common::redis_keys::{FIRST_PAGE_TOPIC_EXPIRE, FIRST_PAGE_TOPIC_KEY, TOPIC_MAP_KEY};
use crate::conf::redis_config::get_pool_connection;
use crate::error::custom_error::{E, Status};
use crate::models::blogs::{SimpleBlogVo, TopicBlogVo};
use crate::models::topic::{SimpleTopicVo, TopicRequest, TopicVo, UserSimpleTopicVo};
use crate::repository::topic_repository::TopicRepository;
use crate::response::page_info::PageInfo;

pub struct TopicService(Arc<TopicRepository>);

impl TopicService {
    pub fn new(db_conn: Pool<Postgres>) -> TopicService {
        let topic_repository = TopicRepository::new(db_conn);
        TopicService(Arc::new(topic_repository))
    }

    pub async fn get_topic_list_by_page(&self, page: i64) -> PageInfo<TopicVo> {
        if page == 1 {
            if let Ok(r) =
                get_pool_connection().get::<String, String>(FIRST_PAGE_TOPIC_KEY.to_owned())
            {
                let redis_page_info: PageInfo<TopicVo> = serde_json::from_str(&r).unwrap();
                redis_page_info
            } else {
                // 从数据库中获取主题列表信息
                let result: PageInfo<TopicVo> = self.0.get_topic_by_page(page).await;
                let json_str = serde_json::to_string(&result).unwrap();
                get_pool_connection()
                    .set_ex::<String, String, String>(
                        FIRST_PAGE_TOPIC_KEY.to_owned(),
                        json_str,
                        FIRST_PAGE_TOPIC_EXPIRE,
                    )
                    .unwrap();
                result
            }
        } else {
            // 从数据库中获取主题列表信息
            self.0.get_topic_by_page(page).await
        }
    }

    pub async fn get_topic_all_blogs(&self, tid: i64) -> Vec<SimpleBlogVo> {
        return self.0.get_all_topic_blogs(tid).await;
    }

    pub async fn get_topic_blog_list(&self, page: i64, tid: i64) -> PageInfo<TopicBlogVo> {
        // 从数据库中获取主题博客列表信息
        self.0.get_topic_blogs(page, tid).await
    }

    pub async fn get_user_topics(&self, u_id: i64) -> Vec<UserSimpleTopicVo> {
        // 从数据库中获取用户的主题列表
        self.0.get_user_topics(u_id).await
    }

    pub async fn get_topic_by_id(&self, tid: i64) -> Option<SimpleTopicVo> {
        let result =
            get_pool_connection().hget::<String, i64, String>(TOPIC_MAP_KEY.to_owned(), tid);
        return if let Ok(r) = result {
            let redis_topic = serde_json::from_str(&r).unwrap();
            redis_topic
        } else {
            // 从数据库中获取主题信息
            let result = self.0.get_topic_by_id(tid).await;
            let json_str = serde_json::to_string(&result).unwrap();
            let _ = get_pool_connection()
                .hset::<String, i64, String, String>(TOPIC_MAP_KEY.to_owned(), tid, json_str)
                .is_err();
            result
        };
    }

    pub async fn get_all_topics(&self) -> Vec<SimpleTopicVo> {
        // 从数据库中获取所有主题列表
        self.0.get_all_simple_topic().await
    }

    pub async fn add_topic(&self, uid: i64, topic: TopicRequest) -> Option<E> {
        if !is_image_url(&topic.cover) {
            return Some(E::error(
                Status::CHECK_DATA_ERROR,
                String::from("这不是一个正确的图片链接"),
            ));
        }
        let result = self.0.add_topic(uid, topic).await;

        if let Some(e) = result {
            return Some(E::error(Status::ADD_ERROR, String::from("添加专题失败")));
        } else {
            get_pool_connection()
                .del::<&str, String>(FIRST_PAGE_TOPIC_KEY)
                .unwrap_or_default();
            return None;
        }
    }
}

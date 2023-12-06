use log::error;
use sqlx::{Pool, Postgres, Row};

use crate::common::constants::{BLOG_PAGE_SIZE, TOPIC_PAGE_COUNT};
use crate::error::custom_error::{E, Status};
use crate::models::blogs::{SimpleBlogVo, TopicBlogVo};
use crate::models::topic::{SimpleTopicVo, TopicRequest, TopicVo, UserSimpleTopicVo};
use crate::response::page_info::PageInfo;

pub struct TopicRepository {
    pool: Pool<Postgres>,
}

impl TopicRepository {
    pub fn new(db_pool: Pool<Postgres>) -> TopicRepository {
        return TopicRepository { pool: db_pool };
    }

    /// 获取用户的话题列表。
    pub async fn get_user_topics(&self, u_id: i64) -> Vec<UserSimpleTopicVo> {
        let sql = "SELECT id, name, description, cover_image FROM topics WHERE deleted_at IS NULL AND user_id = $1";
        let result = sqlx::query_as::<_, UserSimpleTopicVo>(sql)
            .bind(&u_id)
            .fetch_all(&self.pool)
            .await;
        return match result {
            Ok(r) => r,
            Err(e) => vec![],
        };
    }

    /// 获取话题相关的博客列表。
    pub async fn get_topic_blogs(&self, page: i64, t_id: i64) -> PageInfo<TopicBlogVo> {
        // 获取博客总数
        let count_sql = format!(
            "SELECT {} FROM blogs b
            JOIN users u ON b.user_id = u.id
            WHERE b.deleted_at IS NULL AND b.topic_id = $1",
            "count(b.id)"
        );

        let count_result = sqlx::query(&count_sql)
            .bind(&t_id)
            .fetch_one(&self.pool)
            .await;

        let total = match count_result {
            Ok(r) => r.get("count"),
            Err(err) => 0,
        };

        let mut result: PageInfo<TopicBlogVo> = PageInfo {
            page,
            size: BLOG_PAGE_SIZE,
            data: vec![],
            total: 0,
        };

        if total == 0 {
            return result;
        }

        let offset = (page - 1) * BLOG_PAGE_SIZE;

        // 获取博客列表
        let select_sql = format!(
            "SELECT {} FROM blogs b
            JOIN users u ON b.user_id = u.id
            WHERE b.deleted_at IS NULL AND b.topic_id = $1
            ORDER BY create_at ASC OFFSET $2 LIMIT $3",
            "b.id, b.title, b.description, b.cover_image, b.create_at, u.id AS u_id, u.nick_name AS u_nick_name"
        );

        let select_result = sqlx::query_as::<_, TopicBlogVo>(&select_sql)
            .bind(&t_id)
            .bind(&offset)
            .bind(BLOG_PAGE_SIZE)
            .bind(&TOPIC_PAGE_COUNT)
            .fetch_all(&self.pool)
            .await;

        return match select_result {
            Ok(r) => {
                result.total = total;
                result.data = r;
                result
            }
            Err(err) => {
                error!("{}", err); // 记录错误
                result
            }
        };
    }

    /// 获取话题的所有博客
    pub async fn get_all_topic_blogs(&self, t_id: i64) -> Vec<SimpleBlogVo> {
        let sql = "select id,title from blogs where deleted_at is null and topic_id = $1 order by create_at asc";

        let result = sqlx::query_as::<_, SimpleBlogVo>(sql)
            .bind(&t_id)
            .fetch_all(&self.pool)
            .await;

        return match result {
            Ok(r) => r,
            Err(err) => {
                error!("{}", err); // 记录错误
                vec![]
            }
        };
    }

    /// 通过话题ID获取话题信息。
    pub async fn get_topic_by_id(&self, t_id: i64) -> Option<SimpleTopicVo> {
        let sql = "SELECT id, name FROM topics WHERE deleted_at IS NULL AND id = $1";
        let result = sqlx::query_as::<_, SimpleTopicVo>(sql)
            .bind(&t_id)
            .fetch_one(&self.pool)
            .await;
        return match result {
            Ok(r) => Some(r),
            Err(e) => None,
        };
    }

    /// 获取所有话题的简要信息。
    pub async fn get_all_simple_topic(&self) -> Vec<SimpleTopicVo> {
        let sql = "SELECT id, name FROM topics WHERE deleted_at IS NULL";
        let result = sqlx::query_as::<_, SimpleTopicVo>(sql)
            .fetch_all(&self.pool)
            .await;
        return match result {
            Ok(r) => r,
            Err(e) => vec![],
        };
    }

    /// 获取话题列表分页。
    pub async fn get_topic_by_page(&self, page: i64) -> PageInfo<TopicVo> {
        // 获取话题总数
        let count_sql = format!(
            "SELECT {} FROM topics t
            JOIN public.users u ON u.id = t.user_id
            WHERE t.deleted_at IS NULL",
            "count(t.id)"
        );

        let count_result = sqlx::query(&count_sql).fetch_one(&self.pool).await;

        let total = match count_result {
            Ok(r) => r.get("count"),
            Err(err) => 0,
        };

        let mut result: PageInfo<TopicVo> = PageInfo {
            page,
            size: TOPIC_PAGE_COUNT,
            data: vec![],
            total: 0,
        };

        if total == 0 {
            return result;
        }

        let offset = (page - 1) * TOPIC_PAGE_COUNT;

        // 获取话题列表
        let select_sql = format!(
            "SELECT {} FROM topics t
            JOIN public.users u ON u.id = t.user_id
            WHERE t.deleted_at IS NULL
            ORDER BY create_at DESC OFFSET $1 LIMIT $2",
            "t.id, t.name, t.description, t.cover_image, t.create_at, u.id AS u_id, u.username AS u_username, u.nick_name AS u_nick_name"
        );

        let select_result = sqlx::query_as::<_, TopicVo>(&select_sql)
            .bind(&offset)
            .bind(&TOPIC_PAGE_COUNT)
            .fetch_all(&self.pool)
            .await;

        return match select_result {
            Ok(r) => {
                result.total = total;
                result.data = r;
                result
            }
            Err(err) => result,
        };
    }

    pub async fn add_topic(&self, uid: i64, topic: TopicRequest) -> Option<E> {
        let sql = "insert into topics(name,description,cover_image,create_at,update_at,user_id) values ($1,$2,$3,now(),now(),$4)";

        let result = sqlx::query(sql)
            .bind(&topic.name)
            .bind(&topic.desc)
            .bind(&topic.cover)
            .bind(&uid)
            .execute(&self.pool)
            .await;

        if let Err(e) = result {
            return Some(E::error(Status::ADD_ERROR, String::from("添加专题失败")));
        } else {
            return None;
        }
    }
}

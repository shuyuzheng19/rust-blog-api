use log::error;
use sqlx::{Pool, Postgres, Row};

use crate::common::constants::BLOG_PAGE_SIZE;
use crate::models::blogs::BlogVo;
use crate::models::tag::TagVo;
use crate::response::page_info::PageInfo;

pub struct TagRepository {
    pool: Pool<Postgres>,
}

impl TagRepository {
    pub fn new(db_pool: Pool<Postgres>) -> TagRepository {
        TagRepository { pool: db_pool }
    }

    /// 添加标签到数据库。
    pub async fn add_tag(&self, name: &String) -> Option<TagVo> {
        let sql =
            "INSERT INTO tags (name, create_at, update_at) VALUES ($1, now(), now()) returning id";
        let result = sqlx::query(sql).bind(name).fetch_one(&self.pool).await;
        return match result {
            Ok(r) => {
                let id: i64 = r.get("id");
                if id > 0 {
                    Some(TagVo {
                        id: id,
                        name: name.to_owned(),
                    })
                } else {
                    None
                }
            }
            Err(e) => {
                error!("{}", e); // 记录错误
                None
            }
        };
    }

    /// 获取标签列表。
    pub async fn get_tag_list(&self) -> Vec<TagVo> {
        let sql = "SELECT id, name FROM tags WHERE deleted_at IS NULL";
        let result = sqlx::query_as::<_, TagVo>(sql).fetch_all(&self.pool).await;
        return match result {
            Ok(r) => r,
            Err(e) => {
                error!("{}", e); // 记录错误
                vec![]
            }
        };
    }

    /// 通过标签ID获取标签信息。
    pub async fn get_tag_by_id(&self, t_id: i64) -> Option<TagVo> {
        let sql = "SELECT id, name FROM tags WHERE deleted_at IS NULL AND id = $1";
        let result = sqlx::query_as::<_, TagVo>(sql)
            .bind(&t_id)
            .fetch_one(&self.pool)
            .await;
        return match result {
            Ok(r) => Some(r),
            Err(e) => None,
        };
    }

    /// 获取标签相关的博客列表。
    pub async fn get_tag_blogs(&self, page: i64, t_id: i64) -> PageInfo<BlogVo> {
        // 获取博客总数
        let count_sql = format!(
            "SELECT {} FROM blogs b
            JOIN categories c ON c.id = b.category_id
            JOIN users u ON b.user_id = u.id
            JOIN blogs_tags bg ON bg.blog_id = b.id
            WHERE b.deleted_at IS NULL AND bg.tag_id = $1",
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

        let mut result: PageInfo<BlogVo> = PageInfo {
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
            JOIN categories c ON c.id = b.category_id
            JOIN users u ON b.user_id = u.id
            JOIN blogs_tags bg ON bg.blog_id = b.id
            WHERE b.deleted_at IS NULL AND bg.tag_id = $1
            ORDER BY create_at DESC OFFSET $2 LIMIT $3",
            "b.id, b.title, b.description, b.cover_image, b.create_at, u.id AS u_id, u.nick_name AS u_nick_name, c.id AS c_id, c.name AS c_name"
        );

        let select_result = sqlx::query_as::<_, BlogVo>(&select_sql)
            .bind(&t_id)
            .bind(&offset)
            .bind(BLOG_PAGE_SIZE)
            .bind(&BLOG_PAGE_SIZE)
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
}

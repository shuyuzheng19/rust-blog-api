use log::error;
use sqlx::{Executor, Pool, Postgres, QueryBuilder, Row};

use crate::common::constants::{
    ARCHIVE_BLOG_PAGE_SIZE, BLOG_PAGE_SIZE, LATEST_BLOG_PAGE_SIZE, USER_TOP_BLOG_PAGE_SIZE,
};
use crate::error::custom_error::{E, Status};
use crate::models::blogs::{
    ArchiveBlogVo, BlogContentVo, BlogVo, RecommendBlogVo, SearchBlogVo, SimpleBlogVo,
};
use crate::models::tag::TagVo;
use crate::request::blog_request::{
    ArchiveRange, BlogFindRequest, BlogRequest, GetUserBlogRequest,
};
use crate::response::page_info::PageInfo;

pub struct BlogRepository {
    pool: Pool<Postgres>,
}

impl BlogRepository {
    pub fn new(db_pool: Pool<Postgres>) -> BlogRepository {
        BlogRepository { pool: db_pool }
    }

    pub async fn find_page_category_list(&self, r: &BlogFindRequest) -> PageInfo<BlogVo> {
        let mut count_sql = format!(
            "SELECT {} FROM blogs b
            JOIN categories c ON b.category_id = c.id
            JOIN users u on b.user_id = u.id
            WHERE b.deleted_at is null",
            "count(b.id)"
        );

        let mut select_sql = format!(
            "SELECT {} FROM blogs b
            JOIN categories c ON b.category_id = c.id
            JOIN users u on b.user_id = u.id
            WHERE b.deleted_at is null",
            "b.id, b.title, b.description, b.cover_image,b.create_at, c.id AS c_id, c.name AS c_name,u.id as u_id,u.nick_name as u_nick_name"
        );

        let sort = &r.sort;

        let mut select_build: QueryBuilder<Postgres> = QueryBuilder::new(&select_sql.to_string());

        let mut count_build: QueryBuilder<Postgres> = QueryBuilder::new(&count_sql.to_string());

        if r.cid > 0 {
            count_build.push(" AND b.category_id = ").push_bind(r.cid);
            select_build.push(" AND b.category_id = ").push_bind(r.cid);
        } else {
            count_build.push(" AND b.category_id is not null ");
            select_build.push(" AND b.category_id is not null ");
        }

        let size = BLOG_PAGE_SIZE;

        let offset = (r.page - 1) * size;

        select_build.push(&format!(
            " ORDER BY {} OFFSET {} LIMIT {}",
            sort.to_order_by_string(String::from("b.")),
            offset,
            size
        ));

        let total: i64 = match count_build.build().fetch_one(&self.pool).await {
            Ok(row) => row.get("count"),
            Err(err) => {
                error!("数据库查询失败: {}", err);
                0
            }
        };

        if total == 0 {
            return PageInfo {
                page: 1,
                size: 10,
                total: 0,
                data: vec![],
            };
        }

        let result = select_build
            .build_query_as::<BlogVo>()
            .fetch_all(&self.pool)
            .await;

        return match result {
            Ok(blogs) => PageInfo {
                page: r.page,
                size,
                total,
                data: blogs,
            },
            Err(e) => PageInfo {
                page: 1,
                size: 10,
                total: 0,
                data: vec![],
            },
        };
    }

    pub async fn get_hot_blogs(&self) -> Vec<SimpleBlogVo> {
        let sql = format!("select id,title from blogs where deleted_at is null order by eye_count desc offset 0 limit {}", BLOG_PAGE_SIZE);
        let result = sqlx::query_as::<_, SimpleBlogVo>(&sql)
            .fetch_all(&self.pool)
            .await;
        return match result {
            Ok(blogs) => blogs,
            Err(e) => {
                error!("数据库查询失败: {}", e);
                Vec::new()
            }
        };
    }

    pub async fn update_eye_count(&self, id: i64, eye_count: i64) -> i64 {
        let sql = "update blogs set eye_count = $1 where id = $2";
        let result = sqlx::query(sql)
            .bind(&eye_count)
            .bind(&id)
            .execute(&self.pool)
            .await;
        return match result {
            Ok(r) => r.rows_affected() as i64,
            _ => 0,
        };
    }
    pub async fn get_blog_by_id(&self, id: &i64) -> Option<BlogContentVo> {
        let sql = "SELECT
        b.id, b.title, b.description, b.cover_image, b.source_url, b.content, b.eye_count, b.create_at,b,
        c.id AS c_id, c.name AS c_name, u.id AS u_id, u.nick_name AS u_nick_name, 0 AS like_count,
        t.id as t_id, t.name as t_name,
        b.update_at AS update_at
    FROM
        blogs AS b
    LEFT JOIN
        categories c ON b.category_id = c.id
    INNER JOIN
        users u ON b.user_id = u.id
    LEFT JOIN
        topics t ON b.topic_id = t.id
    WHERE
        b.deleted_at is null
        AND b.id = $1";

        let result = sqlx::query_as::<_, BlogContentVo>(&sql)
            .bind(id)
            .fetch_one(&self.pool)
            .await;

        return match result {
            Ok(mut blog) => {
                if let None = blog.topic {
                    let tag_sql = "select id,name from tags t join blogs_tags bt on t.id = bt.tag_id where deleted_at is null and bt.blog_id=$1";
                    match sqlx::query_as::<_, TagVo>(tag_sql)
                        .bind(id)
                        .fetch_all(&self.pool)
                        .await
                    {
                        Ok(tags) => {
                            blog.tags = tags;
                            Some(blog)
                        }
                        Err(e) => {
                            error!("数据库查询失败: {}", e);
                            Some(blog)
                        }
                    }
                } else {
                    Some(blog)
                }
            }
            Err(e) => {
                error!("数据库查询失败: {}", e);
                None
            }
        };
    }

    pub async fn get_blog_by_range_date(&self, range: &ArchiveRange) -> PageInfo<ArchiveBlogVo> {
        let count_sql = format!(
            "SELECT {} FROM blogs b
            WHERE b.deleted_at is null and b.create_at BETWEEN $1 and $2",
            "count(b.id)"
        );

        let count_result = sqlx::query(&count_sql)
            .bind(&range.1)
            .bind(&range.2)
            .fetch_one(&self.pool)
            .await;

        let total = match count_result {
            Ok(r) => r.get("count"),
            Err(err) => {
                error!("数据库查询失败: {}", err);
                0
            }
        };

        if total == 0 {
            return PageInfo {
                page: 1,
                size: 10,
                total: 0,
                data: vec![],
            };
        }

        let size = ARCHIVE_BLOG_PAGE_SIZE;

        let offset = (range.0 - 1) * size;

        let select_sql = format!(
            "SELECT {} FROM blogs b
            WHERE b.deleted_at is null and b.create_at BETWEEN $1 and $2 ORDER BY b.create_at desc OFFSET {} LIMIT {}",
            "b.id, b.title, b.description, b.create_at as create", offset, size
        );

        let select_result = sqlx::query_as::<_, ArchiveBlogVo>(&select_sql)
            .bind(&range.1)
            .bind(&range.2)
            .fetch_all(&self.pool)
            .await;

        return match select_result {
            Ok(data) => PageInfo {
                page: range.0,
                size,
                total,
                data,
            },
            Err(e) => {
                error!("数据库查询失败: {}", e);
                PageInfo {
                    page: 1,
                    size: 10,
                    total: 0,
                    data: vec![],
                }
            }
        };
    }

    pub async fn get_latest_blog(&self) -> Vec<SimpleBlogVo> {
        let sql = format!("select id,title from blogs where deleted_at is null order by create_at desc offset 0 limit {}", LATEST_BLOG_PAGE_SIZE);
        let result = sqlx::query_as::<_, SimpleBlogVo>(&sql)
            .fetch_all(&self.pool)
            .await;
        return match result {
            Ok(blogs) => blogs,
            Err(e) => {
                error!("数据库查询失败: {}", e);
                Vec::new()
            }
        };
    }

    pub async fn get_user_blog(&self, uid: &i64, req: &GetUserBlogRequest) -> PageInfo<BlogVo> {
        let count_sql = format!(
            "SELECT {} FROM blogs b
            JOIN categories c ON b.category_id = c.id
            JOIN users u on b.user_id = u.id
            WHERE b.deleted_at is null and b.user_id = $1",
            "count(b.id)"
        );

        let count_result = sqlx::query(&count_sql)
            .bind(&uid)
            .fetch_one(&self.pool)
            .await;

        let total = match count_result {
            Ok(r) => r.get("count"),
            Err(err) => {
                error!("数据库查询失败: {}", err);
                0
            }
        };

        if total == 0 {
            return PageInfo {
                page: 1,
                size: 10,
                total: 0,
                data: vec![],
            };
        }

        let size = BLOG_PAGE_SIZE;

        let offset = (req.page - 1) * size;

        let select_sql = format!(
            "SELECT {} FROM blogs b
            JOIN categories c ON b.category_id = c.id
            JOIN users u on b.user_id = u.id
            WHERE b.deleted_at is null and b.user_id = $1 order by {} offset {} limit {}",
            "b.id, b.title, b.description, b.cover_image,b.create_at,c.id AS c_id, c.name AS c_name,u.id as u_id,u.nick_name as u_nick_name",
            req.sort.to_order_by_string(String::from("b.")), offset, size
        );

        let select_result = sqlx::query_as::<_, BlogVo>(&select_sql)
            .bind(&uid)
            .fetch_all(&self.pool)
            .await;

        return match select_result {
            Ok(data) => PageInfo {
                page: req.page,
                size,
                total,
                data,
            },
            Err(err) => {
                error!("数据库查询失败: {}", err);
                PageInfo {
                    page: 1,
                    size: 10,
                    total: 0,
                    data: vec![],
                }
            }
        };
    }

    pub async fn get_user_top_blog(&self, uid: &i64) -> Vec<SimpleBlogVo> {
        let sql = format!("select id,title from blogs where deleted_at is null and user_id = $1 order by eye_count desc offset 0 limit {}", USER_TOP_BLOG_PAGE_SIZE);
        let result = sqlx::query_as::<_, SimpleBlogVo>(&sql)
            .bind(&uid)
            .fetch_all(&self.pool)
            .await;
        return match result {
            Ok(blogs) => blogs,
            Err(e) => {
                error!("数据库查询失败: {}", e);
                Vec::new()
            }
        };
    }

    pub async fn get_all_simple_blog(&self) -> Vec<SearchBlogVo> {
        let sql = "select id,title,description from blogs where deleted_at is null";
        let result = sqlx::query_as::<_, SearchBlogVo>(&sql)
            .fetch_all(&self.pool)
            .await;
        return match result {
            Ok(blogs) => blogs,
            Err(e) => {
                error!("数据库查询失败: {}", e);
                Vec::new()
            }
        };
    }

    pub async fn insert_blog(&self, req: &BlogRequest, uid: i64) -> Result<i64, E> {
        let sql = "insert into blogs(description, title, cover_image,
                source_url, content, create_at, update_at, category_id, user_id, topic_id)
values ($1,$2,$3,$4,$5,now(),now(),$6,$7,$8) returning id;";

        let mut transaction = self.pool.begin().await.unwrap();

        let query = sqlx::query::<sqlx::Postgres>(&sql)
            .bind(&req.description)
            .bind(&req.title)
            .bind(&req.cover_image)
            .bind(&req.source_url)
            .bind(&req.content)
            .bind(&req.category)
            .bind(&uid)
            .bind(&req.topic);

        let result = transaction.fetch_one(query).await;

        return match result {
            Ok(r) => {
                let b_id: i64 = r.get("id");
                if req.category != None && req.tags.len() > 0 && b_id > 0 {
                    let mut builder =
                        QueryBuilder::new("insert into blogs_tags(blog_id,tag_id) values");

                    for (i, t_id) in req.tags.iter().enumerate() {
                        builder
                            .push("(")
                            .push_bind(&b_id)
                            .push(",")
                            .push(" ")
                            .push_bind(t_id)
                            .push(")");

                        if i < req.tags.len() - 1 {
                            builder.push(",");
                        }
                    }

                    let query = builder.build();

                    let result = transaction.execute(query).await;

                    if let Err(e) = result {
                        error!("数据库执行失败: {}", e);
                        transaction.rollback().await.unwrap();
                        return Err(E::default());
                    }
                    let result1 = transaction.commit().await;
                    if let Err(r) = result1 {
                        error!("数据库执行失败: {}", r);
                        return Err(E::default());
                    }
                    return Ok(b_id);
                }
                if b_id > 0 {
                    return Ok(b_id);
                } else {
                    return Err(E::default());
                }
            }
            Err(e) => {
                error!("数据库执行失败: {}", e);
                transaction.rollback().await.unwrap();
                return Err(E::default());
            }
        };
    }

    pub async fn update_blog(&self, req: &BlogRequest, uid: i64) -> Option<E> {
        let mut builder = QueryBuilder::new("update blogs");

        builder
            .push(" set title = ")
            .push_bind(&req.title)
            .push(", description = ")
            .push_bind(&req.description)
            .push(", content = ")
            .push_bind(&req.content)
            .push(", cover_image = ")
            .push_bind(&req.cover_image)
            .push(", source_url = ")
            .push_bind(&req.source_url)
            .push(", category_id = ")
            .push_bind(&req.category)
            .push(", topic_id = ")
            .push_bind(&req.topic)
            .push(" where id = ")
            .push_bind(req.id.unwrap());

        if uid != -1 {
            builder.push(" and user_id = ").push_bind(&uid);
        }

        let mut transaction = self.pool.begin().await.unwrap();

        let result = transaction.execute(builder.build()).await;

        return match result {
            Ok(r) => {
                if r.rows_affected() == 0 {
                    return Some(E::error(
                        Status::ADD_ERROR,
                        String::from("处理失败, 只允许修改自己的博客"),
                    ));
                }
                if req.category != None && req.tags.len() > 0 {
                    let delete_tag = "delete from blogs_tags where blog_id = $1";

                    let query2 = sqlx::query(delete_tag).bind(req.id.unwrap());

                    let result2 = transaction.execute(query2).await;

                    return match result2 {
                        Ok(r) => {
                            let mut builder =
                                QueryBuilder::new("insert into blogs_tags(blog_id,tag_id) values");

                            for (i, t_id) in req.tags.iter().enumerate() {
                                builder
                                    .push("(")
                                    .push_bind(req.id.unwrap())
                                    .push(",")
                                    .push(" ")
                                    .push_bind(t_id)
                                    .push(")");

                                if i < req.tags.len() - 1 {
                                    builder.push(",");
                                }
                            }

                            let query = builder.build();

                            let result = transaction.execute(query).await;

                            if let Err(e) = result {
                                error!("数据库执行失败: {}", e);
                                transaction.rollback().await.unwrap();
                                return Some(E::default());
                            }
                            let result1 = transaction.commit().await;
                            if let Err(r) = result1 {
                                error!("数据库执行失败: {}", r);
                                return Some(E::default());
                            }

                            return None;
                        }
                        Err(e) => None,
                    };
                }
                let result1 = transaction.commit().await;

                if let Err(r) = result1 {
                    error!("数据库执行失败: {}", r);
                    return Some(E::default());
                }
                return None;
            }
            Err(e) => {
                error!("数据库执行失败: {}", e);
                transaction.rollback().await.unwrap();
                return Some(E::default());
            }
        };
    }

    pub async fn select_id_in_blogs(&self, ids: &Vec<i64>) -> Vec<RecommendBlogVo> {
        let params = (1..=ids.len())
            .map(|i| format!("${}", i))
            .collect::<Vec<String>>()
            .join(", ");

        let sql = format!(
            "select id,title,cover_image from blogs where deleted_at is null and id in ({})",
            params
        );

        let mut query = sqlx::query_as(&sql);
        for id in ids {
            query = query.bind(id);
        }
        let result = query.fetch_all(&self.pool).await;
        return match result {
            Ok(blogs) => blogs,
            Err(e) => {
                error!("数据库查询失败: {}", e);
                Vec::new()
            }
        };
    }

    pub async fn get_edit_blog(&self, id: i64) -> Option<BlogRequest> {
        let sql = "select id,title,description,content,source_url,cover_image,\
        topic_id as topic,category_id as category from blogs where deleted_at is null and id = $1";
        let result = sqlx::query_as::<_, BlogRequest>(sql)
            .bind(&id)
            .fetch_one(&self.pool)
            .await;
        return match result {
            Ok(mut r) => {
                if r.id != None && r.category != None {
                    let tag_sql = "select tag_id from blogs_tags where blog_id = $1";
                    let tag_result = sqlx::query_scalar::<_, i64>(tag_sql)
                        .bind(&r.id.unwrap())
                        .fetch_all(&self.pool)
                        .await;
                    if let Ok(t_ids) = tag_result {
                        r.tags = t_ids;
                    } else {
                        r.tags = vec![];
                    }
                    return Some(r);
                }
                return Some(r);
            }
            Err(e) => {
                error!("数据库查询失败: {}", e);
                None
            }
        };
    }
}

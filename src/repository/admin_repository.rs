use log::error;
use sqlx::{Pool, Postgres, QueryBuilder, Row};

use crate::common::constants::{
    ADMIN_BLOG_PAGE_COUNT, CATEGORY_ADMIN_PAGE_COUNT, FILE_ADMIN_PAGE_COUNT, TOPIC_ADMIN_PAGE_COUNT,
};
use crate::common::date_format::time_stamp_to_date;
use crate::models::blogs::BlogAdminVo;
use crate::models::category::{CategoryVo, OtherAdminVo};
use crate::models::file::FileAdminVo;
use crate::models::topic::{AdminTopicVo, TopicRequest};
use crate::request::admin_request::{
    AdminBlogFilter, OtherAdminFilter, UpdatePublicRequest, UpdateRole,
};
use crate::response::page_info::PageInfo;

pub struct AdminRepository {
    pool: Pool<Postgres>,
}

impl AdminRepository {
    pub fn new(db_pool: Pool<Postgres>) -> AdminRepository {
        AdminRepository { pool: db_pool }
    }

    pub async fn update_category_or_tag_name(&self, table: String, category: CategoryVo) -> i64 {
        let sql = format!("update {} set name = $1 where id = $2", table);
        let result = sqlx::query(&sql)
            .bind(&category.name)
            .bind(&category.id)
            .execute(&self.pool)
            .await;
        if let Ok(r) = result {
            return r.rows_affected() as i64;
        } else {
            return 0;
        }
    }

    pub async fn get_admin_other_filter(
        &self,
        table: String,
        req: OtherAdminFilter,
    ) -> PageInfo<OtherAdminVo> {
        let sql = format!("SELECT id, name, create_at, update_at FROM {}", table);
        let count_sql = format!("SELECT COUNT(id) FROM {}", table);

        let mut builder = QueryBuilder::<Postgres>::new(sql.clone());
        let mut count_builder = QueryBuilder::<Postgres>::new(count_sql.clone());

        if req.deleted {
            builder.push(" WHERE deleted_at IS NOT NULL");
            count_builder.push(" WHERE deleted_at IS NOT NULL");
        } else {
            builder.push(" WHERE deleted_at IS NULL");
            count_builder.push(" WHERE deleted_at IS NULL");
        }

        if let Some(keyword) = req.keyword {
            let keyword = format!("%{}%", keyword);
            builder
                .push(" AND name LIKE ")
                .push_bind(keyword.to_owned());
            count_builder
                .push(" AND name LIKE ")
                .push_bind(keyword.to_string());
        }

        if let (Some(start), Some(end)) = (req.start, req.end) {
            let start_date = time_stamp_to_date(start);
            let end_date = time_stamp_to_date(end);
            builder
                .push(" AND create_at BETWEEN ")
                .push_bind(start_date)
                .push(" AND ")
                .push_bind(end_date);
            count_builder
                .push(" AND create_at BETWEEN ")
                .push_bind(start_date)
                .push(" AND ")
                .push_bind(end_date);
        }

        let count_result = count_builder.build().fetch_one(&self.pool).await.unwrap();
        let mut result = PageInfo {
            page: req.page,
            size: CATEGORY_ADMIN_PAGE_COUNT,
            total: 0,
            data: vec![],
        };

        if let Some(count) = count_result.get("count") {
            result.total = count;
        }

        if result.total == 0 {
            return result;
        }

        let offset = (req.page - 1) * CATEGORY_ADMIN_PAGE_COUNT;
        let sort = req.sort.to_order_by_string(String::from(""));
        builder
            .push(format!(" ORDER BY {}", sort))
            .push(" OFFSET ")
            .push_bind(offset)
            .push(" LIMIT ")
            .push_bind(CATEGORY_ADMIN_PAGE_COUNT);

        let select_result = builder
            .build_query_as::<OtherAdminVo>()
            .fetch_all(&self.pool)
            .await
            .unwrap();

        result.data = select_result;

        result
    }

    pub async fn get_admin_topic_filter(
        &self,
        req: OtherAdminFilter,
        uid: i64,
    ) -> PageInfo<AdminTopicVo> {
        let sql = "SELECT t.id, t.name, t.create_at, t.update_at,t.cover_image,t.description, \
        u.id as u_id,u.nick_name as u_nick_name
        FROM topics t JOIN users u on u.id = t.user_id ";
        let count_sql = "SELECT COUNT(t.id) FROM  topics t JOIN users u on u.id = t.user_id";

        let mut builder = QueryBuilder::<Postgres>::new(sql.clone());

        let mut count_builder = QueryBuilder::<Postgres>::new(count_sql.clone());

        if req.deleted {
            builder.push(" WHERE t.deleted_at IS NOT NULL ");
            count_builder.push(" WHERE t.deleted_at IS NOT NULL ");
        } else {
            builder.push(" WHERE t.deleted_at IS NULL ");
            count_builder.push(" WHERE t.deleted_at IS NULL ");
        }

        if uid != -1 {
            builder.push(" AND t.user_id = ").push_bind(uid);
            count_builder.push(" AND t.user_id = ").push_bind(uid);
        }

        if let Some(keyword) = req.keyword {
            let keyword = format!("%{}%", keyword);
            builder
                .push(" AND t.name LIKE ")
                .push_bind(keyword.to_owned());
            count_builder
                .push(" AND t.name LIKE ")
                .push_bind(keyword.to_string());
        }

        if let (Some(start), Some(end)) = (req.start, req.end) {
            let start_date = time_stamp_to_date(start);
            let end_date = time_stamp_to_date(end);
            builder
                .push(" AND t.create_at BETWEEN ")
                .push_bind(start_date)
                .push(" AND ")
                .push_bind(end_date);
            count_builder
                .push(" AND t.create_at BETWEEN ")
                .push_bind(start_date)
                .push(" AND ")
                .push_bind(end_date);
        }

        let count_result = count_builder.build().fetch_one(&self.pool).await.unwrap();
        let mut result = PageInfo {
            page: req.page,
            size: CATEGORY_ADMIN_PAGE_COUNT,
            total: 0,
            data: vec![],
        };

        if let Some(count) = count_result.get("count") {
            result.total = count;
        }

        if result.total == 0 {
            return result;
        }

        let offset = (req.page - 1) * TOPIC_ADMIN_PAGE_COUNT;
        let sort = req.sort.to_order_by_string(String::from("t."));
        builder
            .push(format!(" ORDER BY {}", sort))
            .push(" OFFSET ")
            .push_bind(offset)
            .push(" LIMIT ")
            .push_bind(TOPIC_ADMIN_PAGE_COUNT);

        let select_result = builder
            .build_query_as::<AdminTopicVo>()
            .fetch_all(&self.pool)
            .await
            .unwrap();

        result.data = select_result;

        result
    }

    pub async fn get_admin_blog_filter(
        &self,
        req: AdminBlogFilter,
        deleted: bool,
        uid: i64,
    ) -> PageInfo<BlogAdminVo> {
        let sql = "SELECT
    b.id,b.title,b.description,b.create_at,b.eye_count,b.like_count,b.cover_image,b.source_url,
    u.id as u_id,u.nick_name as u_nick_name,c.id as c_id,c.name as c_name,t.id as t_id,t.name as t_name
FROM
    blogs b
        LEFT JOIN
    categories c ON b.category_id = c.id
        LEFT JOIN
    users u ON u.id = b.user_id
        LEFT JOIN
    topics t ON b.topic_id = t.id
";
        let count_sql = "SELECT
    count(b.id)
FROM
    blogs b
        LEFT JOIN
    categories c ON b.category_id = c.id
        LEFT JOIN
    users u ON u.id = b.user_id
        LEFT JOIN
    topics t ON b.topic_id = t.id
";

        let mut builder = QueryBuilder::<Postgres>::new(sql);

        let mut count_builder = QueryBuilder::<Postgres>::new(count_sql);

        if deleted {
            builder.push(" where b.deleted_at is not null");
            count_builder.push(" where b.deleted_at is not null ");
        } else {
            builder.push(" where b.deleted_at is null");
            count_builder.push(" where b.deleted_at is null ");
        }

        if uid != -1 {
            builder.push(" and b.user_id = ").push_bind(uid);
            count_builder.push(" and b.user_id = ").push_bind(uid);
        }

        if let Some(cid) = req.category {
            builder.push(" and b.category_id = ").push_bind(cid);
            count_builder.push(" and b.category_id = ").push_bind(cid);
        } else if let Some(tid) = req.topic {
            builder.push(" and b.topic_id = ").push_bind(tid);
            count_builder.push(" and b.topic_id = ").push_bind(tid);
        }

        if req.original != None {
            let flag = req.original.unwrap();
            if flag {
                builder.push(" and b.source_url is null ");
                count_builder.push(" and b.source_url is null ");
            } else {
                builder.push(" and b.source_url is not null ");
                count_builder.push(" and b.source_url is not null ");
            }
        }

        if req.start != None && req.end != None {
            let start_date = time_stamp_to_date(req.start.unwrap());

            let end_date = time_stamp_to_date(req.end.unwrap());

            builder
                .push(" and b.create_at BETWEEN ")
                .push_bind(start_date)
                .push(" and ")
                .push_bind(end_date);

            count_builder
                .push(" and b.create_at BETWEEN ")
                .push_bind(start_date)
                .push(" and ")
                .push_bind(end_date);
        }

        if let Some(keyword) = req.keyword {
            let keyword = format!("%{}%", keyword);
            builder
                .push(" and b.title like ")
                .push_bind(keyword.to_owned())
                .push(" or b.description like ")
                .push_bind(keyword.to_owned());
            count_builder
                .push(" and b.title like ")
                .push_bind(keyword.to_owned())
                .push(" or b.description like ")
                .push_bind(keyword.to_owned());
        }

        // 执行查询文件总数的SQL语句并获取结果
        let count_result = count_builder.build().fetch_one(&self.pool).await;

        let mut result = PageInfo {
            page: 1,
            size: ADMIN_BLOG_PAGE_COUNT,
            total: 0,
            data: vec![],
        };

        // 如果查询结果包含文件总数信息
        if let Ok(row) = count_result {
            if let Some(count) = row.get("count") {
                // 设置 PageInfo 结构体的 total 字段
                result.total = count;
            }
        }

        // 如果文件总数为0，则直接返回空的 PageInfo 结构体
        if result.total == 0 {
            return result;
        }

        let offset = (req.page - 1) * ADMIN_BLOG_PAGE_COUNT;

        let sort = req.sort.to_order_by_string(String::from("b."));

        builder
            .push(format!(" order by {} ", sort))
            .push(" offset ")
            .push_bind(offset)
            .push(" limit ")
            .push_bind(ADMIN_BLOG_PAGE_COUNT);

        let select_result = builder
            .build_query_as::<BlogAdminVo>()
            .fetch_all(&self.pool)
            .await;

        return match select_result {
            Ok(data) => {
                result.data = data;
                return result;
            }
            Err(e) => {
                error!("数据库查询失败: {}", e);
                return result;
            }
        };
    }

    pub async fn global_delete_by_ids(
        &self,
        table: &str,
        ids: &Vec<i64>,
        user_id: i64,
        deleted: bool,
    ) -> i64 {
        let mut builder = QueryBuilder::new(format!("update {} ", table));

        if deleted {
            builder.push(" set deleted_at = now() ");
        } else {
            builder.push(" set deleted_at = null ");
        }

        builder.push(" where id in(");

        for (i, b_id) in ids.iter().enumerate() {
            builder.push(" ").push_bind(b_id);

            if i < ids.len() - 1 {
                builder.push(",");
            }
        }

        builder.push(")");

        if user_id != -1 {
            builder.push(" and user_id = ").push_bind(user_id);
        }

        let query = builder.build().execute(&self.pool).await;

        if let Ok(r) = query {
            return r.rows_affected() as i64;
        } else {
            error!("删除博客失败 id:{:?} message:{:?}", ids, query.unwrap_err());
            return 0;
        }
    }

    pub async fn update_topic(&self, topic: TopicRequest, uid: i64) -> i64 {
        let mut sql = "update topics set name = $1,description = $2,cover_image = $3 where deleted_at is null and id = $4".to_string();

        if uid != -1 {
            sql.push_str(&format!(" and user_id = {} ", uid));
        }

        let result = sqlx::query(&sql)
            .bind(&topic.name)
            .bind(&topic.desc)
            .bind(&topic.cover)
            .bind(&topic.id)
            .execute(&self.pool)
            .await;

        if let Ok(r) = result {
            return r.rows_affected() as i64;
        } else {
            return 0;
        }
    }

    pub async fn get_file_list(&self, req: OtherAdminFilter, uid: i64) -> PageInfo<FileAdminVo> {
        // 查询文件总数的SQL语句
        let mut count_builder =QueryBuilder::<Postgres>::new( "select count(f.id) from files f join file_md5 fm on f.md5 = fm.md5 where f.deleted_at is null");

        // 初始化一个 QueryBuilder 用于构建文件查询SQL语句
        let mut builder = QueryBuilder::<Postgres>::new("select \
        f.id,f.old_name as name,f.create_at,f.size,f.is_public as public,f.user_id as uid,f.size,fm.md5,fm.url from files f \
        join file_md5 fm on f.md5 = fm.md5 where f.deleted_at is null");

        // 根据用户ID或公共文件过滤文件
        if uid != -1 {
            // 如果用户ID不是-1，添加用户ID过滤条件
            builder.push(" and f.user_id= ").push_bind(&uid);
            count_builder.push(" and f.user_id= ").push_bind(&uid);
        }

        // 如果请求中包含关键字，添加文件名关键字过滤条件
        if let Some(keyword) = &req.keyword {
            let keyword = format!("%{}%", keyword);
            builder
                .push(" and f.old_name like ")
                .push_bind(keyword.to_owned());
            count_builder
                .push(" f.and old_name like ")
                .push_bind(keyword.to_owned());
        }

        // 执行查询文件总数的SQL语句并获取结果
        let count_result = count_builder.build().fetch_one(&self.pool).await;

        // 初始化一个 PageInfo 结构体，用于存储查询结果
        let mut result = PageInfo {
            page: 1,
            size: FILE_ADMIN_PAGE_COUNT,
            total: 0,
            data: vec![],
        };

        // 如果查询结果包含文件总数信息
        if let Ok(row) = count_result {
            if let Some(count) = row.get("count") {
                // 设置 PageInfo 结构体的 total 字段
                result.total = count;
            }
        }

        // 如果文件总数为0，则直接返回空的 PageInfo 结构体
        if result.total == 0 {
            return result;
        }

        // 根据请求中的页码计算查询的偏移量
        let offset = (req.page - 1) * FILE_ADMIN_PAGE_COUNT;

        builder
            .push(format!(
                " order by {} ",
                req.sort.to_order_by_string(String::from("f."))
            ))
            .push(" offset ")
            .push_bind(offset)
            .push(" limit ")
            .push_bind(FILE_ADMIN_PAGE_COUNT);

        // 执行构建的SQL查询语句并获取文件查询结果
        let file_result = builder
            .build_query_as::<FileAdminVo>()
            .fetch_all(&self.pool)
            .await;

        // 如果文件查询结果成功
        if let Ok(rows) = file_result {
            // 更新 PageInfo 结构体的相关字段
            result.page = req.page;
            result.data = rows;
            result.size = FILE_ADMIN_PAGE_COUNT;
        }

        // 返回 PageInfo 结构体，包含查询结果信息
        return result;
    }

    pub async fn update_file_public(&self, req: UpdatePublicRequest, uid: i64) -> i64 {
        let mut sql = format!("update files set is_public = $1 where id = $2");

        if uid != -1 {
            sql.push_str(&format!(" and user_id = {}", uid))
        }

        let result = sqlx::query(&sql)
            .bind(&req.is_pub)
            .bind(&req.id)
            .execute(&self.pool)
            .await;

        if let Ok(r) = result {
            return r.rows_affected() as i64;
        } else {
            return 0;
        }
    }

    // pub async fn delete_blog_by_category(&self,cid:&i64,deleted:bool)->i64{
    //     let sql = format!("update blogs set deleted_at = {} where category_id = $1",if deleted{"now()"}else{"null"});
    //     let result = sqlx::query(&sql).bind(cid).execute(&self.pool).await;
    //     if let Ok(r) =result{
    //         return r.rows_affected() as i64
    //     }else{
    //         return 0
    //     }
    // }

    pub async fn delete_blog_by_categories(&self, ids: &Vec<i64>, deleted: bool, uid: i64) -> i64 {
        let sql = format!(
            "update blogs set deleted_at = {} ",
            if deleted { "now()" } else { "null" }
        );

        let mut builder = QueryBuilder::new(sql);

        builder.push(" where category_id in(");

        for (i, t_id) in ids.iter().enumerate() {
            builder.push(" ").push_bind(t_id);

            if i < ids.len() - 1 {
                builder.push(",");
            }
        }

        builder.push(")");

        if uid != -1 {
            builder.push(" and user_id = ").push_bind(uid);
        }

        let query = builder.build().execute(&self.pool).await;

        if let Ok(r) = query {
            return r.rows_affected() as i64;
        } else {
            error!(
                "删除专题博客失败 ids:{:?} message:{:?}",
                ids,
                query.unwrap_err()
            );
            return 0;
        }
    }

    // pub async fn delete_blog_by_topic(&self,tid:&i64,deleted:bool)->i64{
    //     let sql = format!("update blogs set deleted_at = {} where topic_id = $1",if deleted{"now()"}else{"null"});
    //     let result = sqlx::query(&sql).bind(tid).execute(&self.pool).await;
    //     if let Ok(r) =result{
    //         return r.rows_affected() as i64
    //     }else{
    //         return 0
    //     }
    // }

    pub async fn delete_blog_by_topics(&self, ids: &Vec<i64>, deleted: bool, uid: i64) -> i64 {
        let sql = format!(
            "update blogs set deleted_at = {} ",
            if deleted { "now()" } else { "null" }
        );

        let mut builder = QueryBuilder::new(sql);

        builder.push(" where topic_id in(");

        for (i, t_id) in ids.iter().enumerate() {
            builder.push(" ").push_bind(t_id);

            if i < ids.len() - 1 {
                builder.push(",");
            }
        }

        builder.push(")");

        if uid != -1 {
            builder.push(" and user_id = ").push_bind(uid);
        }

        let query = builder.build().execute(&self.pool).await;

        if let Ok(r) = query {
            return r.rows_affected() as i64;
        } else {
            error!(
                "删除专题博客失败 ids:{:?} message:{:?}",
                ids,
                query.unwrap_err()
            );
            return 0;
        }
    }

    pub async fn update_role(&self, req: &UpdateRole) -> i64 {
        let sql = "update users set role_id = $1 where username = $2";

        let result = sqlx::query(sql)
            .bind(&if req.role == "ADMIN" { 2 } else { 1 })
            .bind(&req.username)
            .execute(&self.pool)
            .await;

        return if let Ok(r) = result {
            r.rows_affected() as i64
        } else {
            0
        };
    }

    pub async fn delete_file_by_ids(&self, ids: Vec<i64>, uid: i64, force: bool) -> i64 {
        if uid != -1 && force {
            return 0;
        }

        let mut sql = "";

        if force {
            sql = "delete from files "
        } else {
            sql = "update files set deleted_at = now() "
        }

        let mut builder = QueryBuilder::new(sql);

        builder.push(" where id in(");

        for (i, b_id) in ids.iter().enumerate() {
            builder.push(" ").push_bind(b_id);

            if i < ids.len() - 1 {
                builder.push(",");
            }
        }

        builder.push(")");

        if uid != -1 {
            builder.push(" and user_id = ").push_bind(uid);
        }

        let query = builder.build().execute(&self.pool).await;

        if let Ok(r) = query {
            return r.rows_affected() as i64;
        } else {
            error!(
                "删除文件失败 ids:{:?} message:{:?}",
                ids,
                query.unwrap_err()
            );
            return 0;
        }
    }
}

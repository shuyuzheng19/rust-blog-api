use log::error;
use sqlx::{Executor, Pool, Postgres, QueryBuilder, Row};

use crate::common::constants::FILE_PAGE_COUNT;
use crate::controller::file_controller::FileFindRequest;
use crate::error::custom_error::{E, Status};
use crate::models::file::{FileInfo, FileVo};
use crate::response::page_info::PageInfo;

pub struct FileRepository {
    pool: Pool<Postgres>,
}

impl FileRepository {
    pub fn new(db_pool: Pool<Postgres>) -> FileRepository {
        FileRepository { pool: db_pool }
    }

    /// 通过文件的 MD5 值查找对应的 URL。
    pub async fn find_by_md5_to_url(&self, md5: &String) -> Option<String> {
        let sql = "SELECT url FROM file_md5 WHERE md5 = $1";

        match sqlx::query(sql).bind(md5).fetch_one(&self.pool).await {
            Ok(row) => {
                let url: String = row.get("url");
                if url.is_empty() {
                    None
                } else {
                    Some(url)
                }
            }
            Err(err) => {
                error!("数据库查询失败: {}", err);
                None
            }
        }
    }

    /// 插入文件信息到数据库，并使用事务保证一致性。
    pub async fn insert_file(&self, file_info: &mut FileInfo) -> Option<E> {
        // 开启事务
        let mut tx = match self.pool.begin().await {
            Ok(tx) => tx,
            Err(err) => {
                error!("事务开始失败: {}", err);
                return Some(E::error(
                    Status::DATABASE_ERROR,
                    String::from("数据库添加失败"),
                ));
            }
        };

        // 插入文件的 MD5 和 URL 到 file_md5 表
        let file_md5_sql = "INSERT INTO file_md5 (md5, url,absolute_path) VALUES ($1, $2,$3)";
        let file_md5_query = sqlx::query(file_md5_sql)
            .bind(&file_info.md5)
            .bind(&file_info.url)
            .bind(&file_info.absolute_path);

        if let Err(err) = tx.execute(file_md5_query).await {
            error!("file_md5 执行失败: {}", err);
            return Some(E::error(
                Status::DATABASE_ERROR,
                String::from("数据库添加失败"),
            ));
        }

        // 插入文件信息到 files 表
        let file_sql = "INSERT INTO files \
        (user_id, old_name, new_name, create_at, size, suffix, is_public, md5)\
         VALUES ($1, $2, $3, now(), $4, $5, $6, $7)";
        let file_query = sqlx::query(file_sql)
            .bind(&file_info.user_id)
            .bind(&file_info.old_name)
            .bind(&file_info.new_name)
            .bind(&file_info.size)
            .bind(&file_info.suffix)
            .bind(&file_info.is_public)
            .bind(&file_info.md5);

        if let Err(err) = tx.execute(file_query).await {
            error!("files 执行失败: {}", err);
            return Some(E::error(
                Status::DATABASE_ERROR,
                String::from("数据库添加失败"),
            ));
        }

        // 提交事务
        if let Err(err) = tx.commit().await {
            error!("事务提交失败: {}", err);
            return Some(E::error(
                Status::DATABASE_ERROR,
                String::from("数据库添加失败"),
            ));
        }

        None
    }

    /// 插入已存在的文件信息到数据库。
    pub async fn insert_already_file(&self, file_info: &FileInfo) -> Option<E> {
        let sql = "INSERT INTO public.files
                   (user_id, old_name, new_name, create_at, size, suffix, is_public, md5)
                   SELECT $1, $2, $3, now(), size, $4, $5, md5
                   FROM files
                   WHERE md5 = $6
                   LIMIT 1;";

        if let Err(err) = sqlx::query(sql)
            .bind(&file_info.user_id)
            .bind(&file_info.old_name)
            .bind(&file_info.new_name)
            .bind(&file_info.suffix)
            .bind(&file_info.is_public)
            .bind(&file_info.md5)
            .execute(&self.pool)
            .await
        {
            error!("已存在文件执行失败: {}", err);
            return Some(E::error(
                Status::DATABASE_ERROR,
                String::from("数据库执行失败"),
            ));
        }

        None
    }

    pub async fn find_file_by_page(&self, uid: i64, req: &FileFindRequest) -> PageInfo<FileVo> {
        // 查询文件总数的SQL语句
        let mut count_builder =QueryBuilder::<Postgres>::new( "select count(f.id) from files f join file_md5 fm on f.md5 = fm.md5 where deleted_at is null");

        // 初始化一个 QueryBuilder 用于构建文件查询SQL语句
        let mut builder = QueryBuilder::<Postgres>::new(
            "select \
        f.id,f.old_name,f.create_at,suffix,f.size,fm.md5,fm.url from files f \
        join file_md5 fm on f.md5 = fm.md5 where deleted_at is null",
        );

        // 根据用户ID或公共文件过滤文件
        if uid != -1 {
            // 如果用户ID不是-1，添加用户ID过滤条件
            builder.push(" and user_id= ").push_bind(&uid);
            count_builder.push(" and user_id= ").push_bind(&uid);
        } else {
            // 如果用户ID是-1，表示查询公共文件，添加公共文件过滤条件
            builder.push(" and is_public= ").push_bind(&true);
            count_builder.push(" and is_public= ").push_bind(&true);
        }

        // 如果请求中包含关键字，添加文件名关键字过滤条件
        if let Some(keyword) = &req.keyword {
            let keyword = format!("%{}%", keyword);
            builder
                .push(" and old_name like ")
                .push_bind(keyword.to_owned());
            count_builder
                .push(" and old_name like ")
                .push_bind(keyword.to_owned());
        }

        // 执行查询文件总数的SQL语句并获取结果
        let count_result = count_builder.build().fetch_one(&self.pool).await;

        // 初始化一个 PageInfo 结构体，用于存储查询结果
        let mut result = PageInfo {
            page: 1,
            size: FILE_PAGE_COUNT,
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

        if &req.sort == "size" {
            builder.push(" order by size desc");
        } else {
            builder.push(" order by create_at desc");
        }

        // 根据请求中的排序字段排序文件结果

        // 根据请求中的页码计算查询的偏移量
        let offset = (req.page - 1) * FILE_PAGE_COUNT;

        builder
            .push(" offset ")
            .push_bind(&offset)
            .push(" limit ")
            .push_bind(&FILE_PAGE_COUNT);

        // 执行构建的SQL查询语句并获取文件查询结果
        let file_result = builder.build_query_as().fetch_all(&self.pool).await;

        // 如果文件查询结果成功
        if let Ok(rows) = file_result {
            // 更新 PageInfo 结构体的相关字段
            result.page = req.page;
            result.data = rows;
            result.size = FILE_PAGE_COUNT;
        }

        // 返回 PageInfo 结构体，包含查询结果信息
        return result;
    }
}

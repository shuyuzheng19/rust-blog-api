use std::sync::Arc;

use log::{error, info};
use r2d2_redis::redis::Commands;
use sqlx::{Pool, Postgres};

use crate::cache::blog_cache::BlogCache;
use crate::cache::clear_page_info_keys;
use crate::common::redis_keys::{BLOG_LIST_PAGE_INFO_KEY, BLOG_MAP_KEY, EYE_COUNT_MAP, HOUR};
use crate::conf::config::CONFIG;
use crate::conf::redis_config::get_pool_connection;
use crate::error::custom_error::{E, Status};
use crate::models::blogs::{
    ArchiveBlogVo, BlogContentVo, BlogVo, RecommendBlogVo, SearchBlogVo, SimpleBlogVo,
};
use crate::repository::blog_repository::BlogRepository;
use crate::request::blog_request::{
    ArchiveRange, BlogFindRequest, BlogRequest, GetUserBlogRequest,
};
use crate::response::page_info::PageInfo;

pub struct BlogService(Arc<BlogRepository>, BlogCache);

impl BlogService {
    pub fn new(db_conn: Pool<Postgres>) -> BlogService {
        let blog_cache = BlogCache::new();
        let blog_repository = BlogRepository::new(db_conn);
        BlogService(Arc::new(blog_repository), blog_cache)
    }

    // 获取指定分类的博客列表
    pub async fn get_blog_list_by_category(&self, request: &BlogFindRequest) -> PageInfo<BlogVo> {
        if CONFIG.blog_page_cache {
            let key = format!(
                "{}_{}_{:?}_{}",
                BLOG_LIST_PAGE_INFO_KEY, request.page, request.sort, request.cid
            );

            // 尝试从 Redis 缓存中获取博客列表页信息
            if let Ok(r) = get_pool_connection().get::<String, String>(key.to_owned()) {
                let redis_page_info: PageInfo<BlogVo> = serde_json::from_str(&r).unwrap();
                return redis_page_info;
            } else {
                // 从数据库中获取博客列表页信息，并将结果缓存到 Redis 中
                let result: PageInfo<BlogVo> = self.0.find_page_category_list(request).await;
                let json_str = serde_json::to_string(&result).unwrap();
                get_pool_connection()
                    .set_ex::<String, String, String>(
                        key.to_owned(),
                        json_str,
                        CONFIG.blog_page_cache_expire * HOUR,
                    )
                    .unwrap();
                return result;
            }
        } else {
            // 不使用缓存，直接从数据库中获取博客列表页信息
            return self.0.find_page_category_list(request).await;
        }
    }

    // 获取指定日期范围的博客列表
    pub async fn get_archive_blog_by_range(
        &self,
        request: &ArchiveRange,
    ) -> PageInfo<ArchiveBlogVo> {
        return self.0.get_blog_by_range_date(request).await;
    }

    // 获取热门博客
    pub async fn get_hot_blog(&self) -> Vec<SimpleBlogVo> {
        let hots_blog = self.1.get_hot_blog();
        return if hots_blog.is_none() {
            let hot_blogs = self.0.get_hot_blogs().await;
            self.1.set_hot_blog(Option::from(&hot_blogs));
            hot_blogs
        } else {
            hots_blog.unwrap()
        };
    }

    // 获取最新博客
    pub async fn get_latest_blog(&self) -> Vec<SimpleBlogVo> {
        let latest_blog = self.1.get_latest_blog();
        return if latest_blog.is_none() {
            let latest_blogs = self.0.get_latest_blog().await;
            self.1.set_latest_blog(Option::from(&latest_blogs));
            latest_blogs
        } else {
            latest_blog.unwrap()
        };
    }

    pub async fn get_blog_by_id(&self, id: i64) -> Result<BlogContentVo, E> {
        // 检查参数是否合法
        if id <= 0 {
            return Err(E::error(
                Status::QUERY_OR_PARAMS_ERROR,
                String::from("非法参数"),
            ));
        }

        // 尝试从 Redis 缓存中获取博客信息
        let redis_user = self.1.get_blog_info(id);

        // 如果 Redis 获取失败，则从数据库中获取博客信息，并将结果缓存到 Redis 中
        return if redis_user.is_err() {
            let result = self.0.get_blog_by_id(&id).await;
            self.1.set_blog_info(id, &result);

            // 根据数据库查询结果进行处理
            match result {
                Some(blog) => Ok(blog),
                None => Err(E::error(
                    Status::BLOG_NOT_FOUND_ERROR,
                    String::from("该博客不存在"),
                )),
            }
        } else {
            // 如果 Redis 获取成功，则直接返回获取到的博客信息
            match redis_user.unwrap() {
                Some(blog) => Ok(blog),
                None => Err(E::error(
                    Status::BLOG_NOT_FOUND_ERROR,
                    String::from("该博客不存在"),
                )),
            }
        };
    }

    // 增加博客浏览次数
    pub async fn increase_in_view(&self, default_count: i64, id: i64) -> i64 {
        return self.1.increase_in_view(default_count, id);
    }

    // 获取用户的博客列表
    pub async fn get_blog_list_by_user(
        &self,
        uid: i64,
        req: GetUserBlogRequest,
    ) -> PageInfo<BlogVo> {
        return self.0.get_user_blog(&uid, &req).await;
    }

    // 获取用户置顶的博客列表
    pub async fn get_user_top_blog(&self, uid: &i64) -> Vec<SimpleBlogVo> {
        return self.0.get_user_top_blog(uid).await;
    }

    // 添加博客
    pub async fn add_blog(&self, blog_req: &BlogRequest, u_id: i64) -> Result<i64, E> {
        if let Some(err) = blog_req.check() {
            return Err(E::error(Status::CHECK_DATA_ERROR, err));
        }

        let result = self.0.insert_blog(blog_req, u_id).await;

        if result.is_ok() {
            if CONFIG.blog_page_cache {
                clear_page_info_keys()
            }
        }

        return result;
    }

    // 获取所有博客的简要信息
    pub async fn get_all_simple_blog(&self) -> Vec<SearchBlogVo> {
        return self.0.get_all_simple_blog().await;
    }

    pub async fn init_blog_eye_couunt(&self) {
        let result = self.1.get_blog_map_values();
        for (id, value) in result {
            self.0.update_eye_count(id, value).await;
        }
        get_pool_connection()
            .del::<&str, i64>(EYE_COUNT_MAP)
            .unwrap();
        get_pool_connection()
            .del::<&str, i64>(BLOG_MAP_KEY)
            .unwrap();
    }

    // 获取博客的编辑信息
    pub async fn get_edit_blog_info(&self, b_id: i64) -> Option<BlogRequest> {
        return self.0.get_edit_blog(b_id).await;
    }

    // 更新博客
    pub async fn update_blog(&self, req: BlogRequest, u_id: i64) -> Option<E> {
        if req.id == None {
            return Some(E::error(
                Status::CHECK_DATA_ERROR,
                String::from("博客ID不能为空"),
            ));
        }

        let result = self.0.update_blog(&req, u_id).await;

        if result.is_none() {
            if CONFIG.blog_page_cache {
                clear_page_info_keys()
            }
            let delete_result = self.1.delete_blog_info_by_id(req.id.unwrap());
            if delete_result {
                info!("博客内容已从缓存中清除，ID: {}", req.id.unwrap());
            } else {
                error!("博客内容清除失败，ID: {}", req.id.unwrap());
            }
        }

        return result;
    }

    // 获取推荐博客
    pub async fn get_recommend_blog(&self) -> Vec<RecommendBlogVo> {
        return self.1.get_recommend_blog();
    }

    // 设置推荐博客
    pub async fn set_recommend_blog(&self, ids: &Vec<i64>) -> Result<Vec<RecommendBlogVo>, E> {
        if ids.len() != 4 {
            return Err(E::error(
                Status::CHECK_DATA_ERROR,
                String::from("ID只能为4个"),
            ));
        }

        let blogs = self.0.select_id_in_blogs(ids).await;

        self.1.set_recommend_blog(&blogs);

        info!("已设置推荐博客：{:?}", ids);

        return Ok(blogs);
    }

    // 设置编辑中的博客
    pub async fn set_edit_blog(&self, u_id: i64, content: &String) -> bool {
        return self.1.set_save_edit_blog(u_id, content);
    }

    // 获取编辑中的博客
    pub async fn get_edit_blog(&self, u_id: i64) -> Option<String> {
        return self.1.get_save_edit_blog(u_id);
    }
}

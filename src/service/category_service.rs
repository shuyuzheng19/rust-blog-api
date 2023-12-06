use std::sync::Arc;

use log::info;
use r2d2_redis::redis::Commands;
use sqlx::{Pool, Postgres};

use crate::common::redis_keys::CATEGORY_LIST_KEY;
use crate::conf::redis_config::get_pool_connection;
use crate::models::category::CategoryVo;
use crate::repository::category_repository::CategoryRepository;

pub struct CategoryService(Arc<CategoryRepository>);

impl CategoryService {
    pub fn new(db_conn: Pool<Postgres>) -> CategoryService {
        let category_repository = CategoryRepository::new(db_conn);
        CategoryService(Arc::new(category_repository))
    }

    // 从数据库获取分类列表
    pub async fn get_category_for_db(&self) -> Vec<CategoryVo> {
        // 从数据库中获取分类列表
        self.0.get_category_list().await
    }

    // 添加分类
    pub async fn add_category(&self, name: &String) -> Option<CategoryVo> {
        // 添加分类到数据库
        return self.0.add_category(name).await;
    }

    // 从缓存获取分类列表
    pub async fn get_category_for_cache(&self) -> Vec<CategoryVo> {
        let result = get_pool_connection().get::<String, String>(CATEGORY_LIST_KEY.to_owned());
        return if let Ok(r) = result {
            serde_json::from_str(&r).unwrap()
        } else {
            // 如果缓存中没有数据，从数据库中获取分类列表
            let category_list = self.get_category_for_db().await;
            // 将数据存入缓存
            let json_str = serde_json::to_string(&category_list).unwrap();
            get_pool_connection()
                .set::<String, String, String>(CATEGORY_LIST_KEY.to_owned(), json_str)
                .unwrap();
            info!("将分类列表存入缓存");
            // 返回从数据库中获取的分类列表
            category_list
        };
    }
}

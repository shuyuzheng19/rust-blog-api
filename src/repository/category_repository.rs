use sqlx::{Pool, Postgres, Row};

use crate::models::category::CategoryVo;

pub struct CategoryRepository {
    pool: Pool<Postgres>,
}

impl CategoryRepository {
    pub fn new(db_pool: Pool<Postgres>) -> CategoryRepository {
        CategoryRepository { pool: db_pool }
    }

    /// 添加新的分类到数据库。
    pub async fn add_category(&self, name: &String) -> Option<CategoryVo> {
        let sql = "INSERT INTO categories (name, create_at, update_at) VALUES ($1, NOW(), NOW()) returning id";
        let result = sqlx::query(sql).bind(name).fetch_one(&self.pool).await;

        match result {
            Ok(r) => {
                let id: i64 = r.get("id");
                if id > 0 {
                    Some(CategoryVo {
                        id: id,
                        name: name.to_owned(),
                    }) // 成功：未影响任何行表示成功。
                } else {
                    None // 错误：未影响任何行，表示失败。
                }
            }
            Err(e) => {
                log::error!("添加分类时出错：{}", e);
                None
            }
        }
    }

    /// 从数据库中获取分类列表。
    pub async fn get_category_list(&self) -> Vec<CategoryVo> {
        let sql = "SELECT id, name FROM categories WHERE deleted_at IS NULL";
        let result = sqlx::query_as::<_, CategoryVo>(sql)
            .fetch_all(&self.pool)
            .await;

        match result {
            Ok(r) => r,
            Err(e) => {
                log::error!("从数据库中获取分类列表时出错：{}", e);
                vec![]
            }
        }
    }
}

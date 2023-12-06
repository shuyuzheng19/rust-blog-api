use actix_web::{get, HttpResponse, post, Responder};
use actix_web::web::{Data, Query};
use log::{error, info};
use r2d2_redis::redis::Commands;
use serde::Deserialize;

use crate::AppState;
use crate::common::redis_keys::CATEGORY_LIST_KEY;
use crate::common::result::R;
use crate::conf::redis_config::get_pool_connection;
use crate::error::custom_error::{E, Status};
use crate::middleware::jwt::JwtAdminRole;

// 添加 log

// 获取分类列表（从数据库）
#[get("/list")]
pub async fn get_category_list_for_db(service: Data<AppState>) -> impl Responder {
    // 调用分类服务获取分类列表
    let result = service.category_service.get_category_for_db().await;

    // 返回成功响应
    return R::success(result).response_to_json();
}

// 获取分类列表（从缓存）
#[get("/list2")]
pub async fn get_category_list_for_cache(service: Data<AppState>) -> impl Responder {
    // 调用分类服务获取分类列表
    let result = service.category_service.get_category_for_cache().await;

    // 返回成功响应
    return R::success(result).response_to_json();
}

// 添加新的分类
#[derive(Debug, Deserialize)]
pub struct AddCategory {
    pub name: String,
}

#[post("/add_category")]
pub async fn add_category(
    _: JwtAdminRole,
    name: Query<AddCategory>,
    service: Data<AppState>,
) -> Result<HttpResponse, E> {
    // 记录信息日志，表示收到添加分类请求
    info!("收到添加分类请求: {:?}", name);

    // 调用分类服务添加新的分类
    let result = service
        .category_service
        .add_category(&name.into_inner().name)
        .await;

    if let Some(r) = result {
        get_pool_connection()
            .del::<&str, String>(CATEGORY_LIST_KEY)
            .unwrap_or_default();

        // 记录信息日志，表示添加分类成功
        info!("添加分类成功");

        // 添加分类成功，返回成功响应
        return Ok(R::success(r).response_to_json());
    } else {
        // 记录错误日志，表示添加分类失败
        error!("添加分类失败");

        // 如果添加分类失败，则返回错误响应
        return Err(E::error(Status::ADD_ERROR, String::from("添加分类失败")));
    }
}

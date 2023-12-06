use actix_web::{get, HttpResponse, post, Responder};
use actix_web::web::{Data, Path, Query};
use log::{error, info};
use r2d2_redis::redis::Commands;
use serde::Deserialize;

use crate::AppState;
use crate::common::redis_keys::RANDOM_TAG_KEY;
use crate::common::result::R;
use crate::conf::redis_config::get_pool_connection;
use crate::controller::topic_controller::TopicOrTagBlogByPage;
use crate::error::custom_error::{E, Status};
use crate::middleware::jwt::JwtAdminRole;

// 获取标签列表
#[get("/list")]
pub async fn get_tag_list_for_db(service: Data<AppState>) -> impl Responder {
    let result = service.tag_service.get_category_list().await;
    return R::success(result).response_to_json();
}

// 获取随机标签列表
#[get("/random")]
pub async fn get_random_tag_list(service: Data<AppState>) -> impl Responder {
    let result = service.tag_service.get_random_tag().await;
    return R::success(result).response_to_json();
}

// 添加标签请求参数结构
#[derive(Debug, Deserialize)]
pub struct AddTag {
    pub name: String,
}

// 添加标签
#[post("/add_tag")]
pub async fn add_tag(
    _: JwtAdminRole,
    name: Query<AddTag>,
    service: Data<AppState>,
) -> Result<HttpResponse, E> {
    // 记录信息日志
    info!("正在添加标签：{}", name.name);

    let result = service.tag_service.add_tag(&name.into_inner().name).await;
    return if let Some(r) = result {
        get_pool_connection()
            .del::<&str, String>(RANDOM_TAG_KEY)
            .unwrap_or_default();
        // 记录信息日志
        info!("标签添加成功");
        return Ok(R::success(r).response_to_json());
    } else {
        // 记录错误日志
        error!("添加标签失败");
        Err(E::error(Status::ADD_ERROR, String::from("添加标签失败")))
    };
}

// 获取特定标签的信息
#[get("/get/{t_id}")]
pub async fn get_topic_by_id(t_id: Path<i64>, service: Data<AppState>) -> impl Responder {
    let result = service.tag_service.get_tag_by_id(t_id.into_inner()).await;
    return match result {
        Some(r) => R::success(r).response_to_json(),
        None => HttpResponse::Ok().json(E::default()),
    };
}

// 获取特定标签下的博客列表
#[get("/blogs")]
pub async fn get_tag_blogs(
    req: Query<TopicOrTagBlogByPage>,
    service: Data<AppState>,
) -> impl Responder {
    let req = req.into_inner();
    let result = service
        .tag_service
        .get_tag_blog_list(req.page, req.t_id)
        .await;
    return R::success(result).response_to_json();
}

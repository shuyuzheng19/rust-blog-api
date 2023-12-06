use actix_web::{get, HttpResponse, post, Responder};
use actix_web::web::{Data, Json, Path, Query};
use serde::Deserialize;

use crate::AppState;
use crate::common::constants::default_page;
use crate::common::result::R;
use crate::error::custom_error::E;
use crate::middleware::jwt::{JwtAdminRole, JwtSuperAdminRole};
use crate::models::topic::TopicRequest;

#[derive(Deserialize, Debug)]
pub struct TopicByPage {
    #[serde(default = "default_page")]
    pub page: i64,
}

#[derive(Deserialize, Debug)]
pub struct TopicOrTagBlogByPage {
    #[serde(default = "default_page")]
    pub page: i64,
    pub t_id: i64,
}

#[get("/current")]
pub async fn get_current_user_topic(service: Data<AppState>, user: JwtAdminRole) -> impl Responder {
    let result = service.topic_service.get_user_topics(user.user.id).await;
    return R::success(result).response_to_json();
}

#[get("/all")]
pub async fn get_all_topic_list(service: Data<AppState>, _: JwtSuperAdminRole) -> impl Responder {
    let result = service.topic_service.get_all_topics().await;
    return R::success(result).response_to_json();
}

#[get("/{id}/blogs")]
pub async fn get_all_topic_blogs(service: Data<AppState>, id: Path<i64>) -> impl Responder {
    let result = service
        .topic_service
        .get_topic_all_blogs(id.into_inner())
        .await;
    return R::success(result).response_to_json();
}

#[get("/list")]
pub async fn get_topic_list(
    page_req: Query<TopicByPage>,
    service: Data<AppState>,
) -> impl Responder {
    let result = service
        .topic_service
        .get_topic_list_by_page(page_req.page)
        .await;
    return R::success(result).response_to_json();
}

#[get("/user/{u_id}")]
pub async fn get_user_topics(u_id: Path<i64>, service: Data<AppState>) -> impl Responder {
    let result = service
        .topic_service
        .get_user_topics(u_id.into_inner())
        .await;
    return R::success(result).response_to_json();
}

#[get("/blogs")]
pub async fn get_topic_blogs(
    req: Query<TopicOrTagBlogByPage>,
    service: Data<AppState>,
) -> impl Responder {
    let req = req.into_inner();
    let result = service
        .topic_service
        .get_topic_blog_list(req.page, req.t_id)
        .await;
    return R::success(result).response_to_json();
}

#[get("/get/{t_id}")]
pub async fn get_topic_by_id(t_id: Path<i64>, service: Data<AppState>) -> impl Responder {
    let result = service
        .topic_service
        .get_topic_by_id(t_id.into_inner())
        .await;
    return match result {
        Some(r) => R::success(r).response_to_json(),
        None => HttpResponse::Ok().json(E::default()),
    };
}

#[post("/add_topic")]
pub async fn add_topic(
    jwt: JwtAdminRole,
    service: Data<AppState>,
    req: Json<TopicRequest>,
) -> Result<HttpResponse, E> {
    let result = service
        .topic_service
        .add_topic(jwt.user.id, req.into_inner())
        .await;
    if let Some(e) = result {
        return Err(e);
    } else {
        return Ok(R::success("添加专题成功").response_to_json());
    }
}

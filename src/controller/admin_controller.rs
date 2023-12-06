use std::fs;
use std::fs::File;
use std::io::Read;

use actix_web::{delete, get, HttpResponse, post, put, Responder};
use actix_web::web::{Data, Json, Path, Query};
use log::info;
use r2d2_redis::redis::Commands;
use serde::Deserialize;
use sqlx::types::chrono::Local;

use crate::AppState;
use crate::cache::{
    clear_category_info_keys, clear_tag_info_key, clear_topic_info_key, clear_user_info,
};
use crate::common::redis_keys::{BLOG_WEB_CONFIG, LATEST_BLOG_KEY};
use crate::common::result::R;
use crate::conf::config::CONFIG;
use crate::conf::redis_config::get_pool_connection;
use crate::error::custom_error::{E, Status};
use crate::middleware::jwt::{JwtAdminRole, JwtSuperAdminRole};
use crate::models::category::{CategoryVo, OtherAdminVo};
use crate::models::topic::TopicRequest;
use crate::request::admin_request::{
    AdminBlogFilter, OtherAdminFilter, UpdateGpt, UpdatePublicRequest, UpdateRole,
};
use crate::response::page_info::PageInfo;
use crate::response::website_info::BlogConfigInfo;

// 获取标签列表
#[get("/blog/current_blogs")]
pub async fn get_current_user_blog_list(
    jwt: JwtAdminRole,
    req: Query<AdminBlogFilter>,
    service: Data<AppState>,
) -> impl Responder {
    let result = service
        .admin_service
        .get_admin_blog_list(req.into_inner(), false, jwt.user.id)
        .await;
    return R::success(result).response_to_json();
}

#[get("/category/list")]
pub async fn get_category_list(
    _: JwtSuperAdminRole,
    req: Query<OtherAdminFilter>,
    service: Data<AppState>,
) -> impl Responder {
    let result: PageInfo<OtherAdminVo> = service
        .admin_service
        .get_admin_category_list(req.into_inner())
        .await;
    return R::success(result).response_to_json();
}

#[get("/tag/list")]
pub async fn get_all_tag_list(
    _: JwtSuperAdminRole,
    req: Query<OtherAdminFilter>,
    service: Data<AppState>,
) -> impl Responder {
    let result = service
        .admin_service
        .get_admin_tag_list(req.into_inner())
        .await;
    return R::success(result).response_to_json();
}

#[get("/blog/current_delete_blogs")]
pub async fn get_current_user_delete_blog_list(
    jwt: JwtAdminRole,
    req: Query<AdminBlogFilter>,
    service: Data<AppState>,
) -> impl Responder {
    let result = service
        .admin_service
        .get_admin_blog_list(req.into_inner(), true, jwt.user.id)
        .await;
    return R::success(result).response_to_json();
}

#[get("/blog/all_blogs")]
pub async fn get_super_all_blogs(
    jwt: JwtSuperAdminRole,
    req: Query<AdminBlogFilter>,
    service: Data<AppState>,
) -> impl Responder {
    let result = service
        .admin_service
        .get_admin_blog_list(req.into_inner(), false, -1)
        .await;
    return R::success(result).response_to_json();
}

#[get("/topic/list")]
pub async fn get_topic_list(
    _: JwtSuperAdminRole,
    req: Query<OtherAdminFilter>,
    service: Data<AppState>,
) -> impl Responder {
    let result = service
        .admin_service
        .get_admin_topic_list(req.into_inner(), -1)
        .await;
    return R::success(result).response_to_json();
}

#[get("/topic/current")]
pub async fn get_topic_current_list(
    jwt: JwtAdminRole,
    req: Query<OtherAdminFilter>,
    service: Data<AppState>,
) -> impl Responder {
    let result = service
        .admin_service
        .get_admin_topic_list(req.into_inner(), jwt.user.id)
        .await;
    return R::success(result).response_to_json();
}

#[get("/blog/delete_blogs")]
pub async fn get_super_all_delete_blogs(
    jwt: JwtSuperAdminRole,
    req: Query<AdminBlogFilter>,
    service: Data<AppState>,
) -> impl Responder {
    let result = service
        .admin_service
        .get_admin_blog_list(req.into_inner(), true, -1)
        .await;
    return R::success(result).response_to_json();
}

#[delete("/tag/delete/{id}")]
pub async fn delete_tag_by_id(
    _: JwtSuperAdminRole,
    id: Path<i64>,
    service: Data<AppState>,
) -> impl Responder {
    let result = service
        .admin_service
        .delete_tag_ids(&vec![id.into_inner()], true)
        .await;
    if result > 0 {
        return R::success(result).response_to_json();
    } else {
        return HttpResponse::Ok().json(E::default());
    }
}

#[put("/tag/update")]
pub async fn update_tag(
    _: JwtSuperAdminRole,
    tag: Json<CategoryVo>,
    service: Data<AppState>,
) -> impl Responder {
    let result = service.admin_service.update_tag(tag.into_inner()).await;
    if result > 0 {
        clear_tag_info_key();
        return R::success(result).response_to_json();
    } else {
        return HttpResponse::Ok().json(E::default());
    }
}

#[put("/category/update")]
pub async fn update_category(
    _: JwtSuperAdminRole,
    category: Json<CategoryVo>,
    service: Data<AppState>,
) -> impl Responder {
    let result = service
        .admin_service
        .update_category(category.into_inner())
        .await;
    if result > 0 {
        clear_category_info_keys();
        return R::success(result).response_to_json();
    } else {
        return HttpResponse::Ok().json(E::default());
    }
}

#[delete("/category/delete/{id}")]
pub async fn delete_category_by_id(
    _: JwtSuperAdminRole,
    id: Path<i64>,
    service: Data<AppState>,
) -> impl Responder {
    let result = service
        .admin_service
        .delete_category_ids(&vec![id.into_inner()], true)
        .await;
    if result > 0 {
        R::success(result).response_to_json()
    } else {
        HttpResponse::Ok().json(E::default())
    }
}

#[put("/tag/deletes")]
pub async fn batch_delete_tag_by_ids(
    _: JwtSuperAdminRole,
    ids: Json<Vec<i64>>,
    service: Data<AppState>,
) -> impl Responder {
    let result = service
        .admin_service
        .delete_tag_ids(&ids.into_inner(), true)
        .await;
    if result > 0 {
        R::success(result).response_to_json()
    } else {
        HttpResponse::Ok().json(E::default())
    }
}

#[put("/category/deletes")]
pub async fn batch_delete_category_by_ids(
    _: JwtSuperAdminRole,
    ids: Json<Vec<i64>>,
    service: Data<AppState>,
) -> impl Responder {
    let result = service
        .admin_service
        .delete_category_ids(&ids.into_inner(), true)
        .await;
    if result > 0 {
        R::success(result).response_to_json()
    } else {
        HttpResponse::Ok().json(E::default())
    }
}

#[put("/tag/un_deletes")]
pub async fn batch_un_delete_tag_by_ids(
    _: JwtSuperAdminRole,
    ids: Json<Vec<i64>>,
    service: Data<AppState>,
) -> impl Responder {
    let result = service
        .admin_service
        .delete_tag_ids(&ids.into_inner(), false)
        .await;
    if result > 0 {
        R::success(result).response_to_json()
    } else {
        HttpResponse::Ok().json(E::default())
    }
}

#[put("/category/un_deletes")]
pub async fn batch_un_delete_category_by_ids(
    _: JwtSuperAdminRole,
    ids: Json<Vec<i64>>,
    service: Data<AppState>,
) -> impl Responder {
    let result = service
        .admin_service
        .delete_category_ids(&ids.into_inner(), false)
        .await;
    if result > 0 {
        R::success(result).response_to_json()
    } else {
        HttpResponse::Ok().json(E::default())
    }
}

#[delete("/tag/un_delete/{id}")]
pub async fn un_delete_tag_by_id(
    _: JwtSuperAdminRole,
    id: Path<i64>,
    service: Data<AppState>,
) -> impl Responder {
    let result = service
        .admin_service
        .delete_tag_ids(&vec![id.into_inner()], false)
        .await;
    if result > 0 {
        R::success(result).response_to_json()
    } else {
        HttpResponse::Ok().json(E::default())
    }
}

#[put("/category/un_delete/{id}")]
pub async fn un_delete_category_by_id(
    _: JwtSuperAdminRole,
    id: Path<i64>,
    service: Data<AppState>,
) -> impl Responder {
    let result = service
        .admin_service
        .delete_category_ids(&vec![id.into_inner()], false)
        .await;
    if result > 0 {
        R::success(result).response_to_json()
    } else {
        HttpResponse::Ok().json(E::default())
    }
}

#[delete("/blog/delete/{id}")]
pub async fn delete_blog_by_id(
    jwt: JwtAdminRole,
    id: Path<i64>,
    service: Data<AppState>,
) -> impl Responder {
    let user_id = if jwt.user.role == "SUPER_ADMIN" {
        -1
    } else {
        jwt.user.id
    };

    let result = service
        .admin_service
        .delete_blog_ids(vec![id.into_inner()], user_id, true)
        .await;
    if result > 0 {
        R::success(result).response_to_json()
    } else {
        HttpResponse::Ok().json(E::default())
    }
}

#[put("/blog/un_delete/{id}")]
pub async fn un_delete_blog_by_id(
    jwt: JwtAdminRole,
    id: Path<i64>,
    service: Data<AppState>,
) -> impl Responder {
    let user_id = if jwt.user.role == "SUPER_ADMIN" {
        -1
    } else {
        jwt.user.id
    };

    let result = service
        .admin_service
        .delete_blog_ids(vec![id.into_inner()], user_id, false)
        .await;
    if result > 0 {
        R::success(result).response_to_json()
    } else {
        HttpResponse::Ok().json(E::default())
    }
}

#[put("/blog/deletes")]
pub async fn batch_delete_blog_by_ids(
    jwt: JwtAdminRole,
    ids: Json<Vec<i64>>,
    service: Data<AppState>,
) -> impl Responder {
    let user_id = if jwt.user.role == "SUPER_ADMIN" {
        -1
    } else {
        jwt.user.id
    };

    let result = service
        .admin_service
        .delete_blog_ids(ids.into_inner(), user_id, true)
        .await;
    if result > 0 {
        R::success(result).response_to_json()
    } else {
        HttpResponse::Ok().json(E::default())
    }
}

#[put("/blog/un_deletes")]
pub async fn batch_un_delete_blog_by_ids(
    jwt: JwtAdminRole,
    ids: Json<Vec<i64>>,
    service: Data<AppState>,
) -> impl Responder {
    let user_id = if jwt.user.role == "SUPER_ADMIN" {
        -1
    } else {
        jwt.user.id
    };

    let result = service
        .admin_service
        .delete_blog_ids(ids.into_inner(), user_id, false)
        .await;
    if result > 0 {
        R::success("result").response_to_json()
    } else {
        HttpResponse::Ok().json(E::default())
    }
}

#[put("/topic/update")]
pub async fn update_topic(
    jwt: JwtAdminRole,
    req: Json<TopicRequest>,
    service: Data<AppState>,
) -> impl Responder {
    let user_id = if jwt.user.role == "SUPER_ADMIN" {
        -1
    } else {
        jwt.user.id
    };
    let result = service
        .admin_service
        .update_topic(req.into_inner(), user_id)
        .await;
    if result > 0 {
        clear_topic_info_key();
        R::success("result").response_to_json()
    } else {
        HttpResponse::Ok().json(E::default())
    }
}

#[get("/file/list")]
pub async fn get_file_list(
    jwt: JwtAdminRole,
    req: Query<OtherAdminFilter>,
    service: Data<AppState>,
) -> impl Responder {
    let user_id = if jwt.user.role == "SUPER_ADMIN" {
        -1
    } else {
        jwt.user.id
    };
    let result = service
        .admin_service
        .get_admin_files(req.into_inner(), user_id)
        .await;
    return R::success(result).response_to_json();
}

#[put("/file/public")]
pub async fn update_file_public(
    jwt: JwtAdminRole,
    req: Json<UpdatePublicRequest>,
    service: Data<AppState>,
) -> impl Responder {
    let user_id = if jwt.user.role == "SUPER_ADMIN" {
        -1
    } else {
        jwt.user.id
    };
    let result = service
        .admin_service
        .update_file_public(req.into_inner(), user_id)
        .await;
    if result > 0 {
        return R::success(result).response_to_json();
    } else {
        return HttpResponse::Ok().json(E::default());
    }
}
// 设置推荐博客
#[post("/recommend")]
pub async fn set_recommend_blog(
    _: JwtSuperAdminRole,
    ids: Json<Vec<i64>>,
    service: Data<AppState>,
) -> Result<HttpResponse, E> {
    // 调用博客服务设置推荐博客
    let result = service.blog_service.set_recommend_blog(&ids.0).await;

    match result {
        Ok(r) => {
            // 记录成功日志
            info!("成功设置推荐博客: {:?}", ids.0);
            Ok(R::success(r).response_to_json())
        }
        Err(e) => {
            // 记录错误日志
            info!("设置推荐博客失败: {:?}", e);
            Err(e)
        }
    }
}

#[put("/config")]
pub async fn set_web_site_info(
    _: JwtSuperAdminRole,
    config: Json<BlogConfigInfo>,
) -> impl Responder {
    let json = serde_json::to_string(&config).unwrap();
    get_pool_connection()
        .set::<&str, String, String>(BLOG_WEB_CONFIG, json)
        .unwrap();
    R::success("更新网站配置成功").response_to_json()
}

#[put("/update_role")]
pub async fn update_role(
    _: JwtSuperAdminRole,
    req: Json<UpdateRole>,
    service: Data<AppState>,
) -> impl Responder {
    let req = &req.into_inner();
    let result = service.admin_service.update_role(req).await;
    if result > 0 {
        let username = &req.username; // 创建用户名的副本
        clear_user_info(username);
        R::success("修改角色成功").response_to_json()
    } else {
        HttpResponse::Ok().json(E::default())
    }
}

#[put("/set_gpt_token")]
pub async fn set_gpt_token(
    _: JwtSuperAdminRole,
    req: Json<UpdateGpt>,
    service: Data<AppState>,
) -> impl Responder {
    let req = req.into_inner();
    let keyword  = req.keyword;
    let _type = req.t;
    service.chat_service.lock().unwrap().update_token(_type,keyword);
    R::success("修改GPT成功").response_to_json()
}

#[derive(Debug, Deserialize)]
pub struct ForceDeleteFile {
    force: bool,
}

#[delete("/file/delete/{id}")]
pub async fn delete_file_by_id(
    jwt: JwtAdminRole,
    force: Query<ForceDeleteFile>,
    id: Path<i64>,
    service: Data<AppState>,
) -> impl Responder {
    let user_id = if jwt.user.role == "SUPER_ADMIN" {
        -1
    } else {
        jwt.user.id
    };
    let result = service
        .admin_service
        .delete_file(vec![id.into_inner()], user_id, force.force)
        .await;
    return R::success(result).response_to_json();
}

#[put("/file/deletes")]
pub async fn delete_file_by_ids(
    jwt: JwtAdminRole,
    force: Query<ForceDeleteFile>,
    ids: Json<Vec<i64>>,
    service: Data<AppState>,
) -> impl Responder {
    let user_id = if jwt.user.role == "SUPER_ADMIN" {
        -1
    } else {
        jwt.user.id
    };
    let result = service
        .admin_service
        .delete_file(ids.into_inner(), user_id, force.force)
        .await;
    return R::success(result).response_to_json();
}

#[delete("/topic/delete/{id}")]
pub async fn delete_topic_by_id(
    jwt: JwtAdminRole,
    id: Path<i64>,
    service: Data<AppState>,
) -> impl Responder {
    let user_id = if jwt.user.role == "SUPER_ADMIN" {
        -1
    } else {
        jwt.user.id
    };
    let result = service
        .admin_service
        .delete_topics_ids(&vec![id.into_inner()], true, user_id)
        .await;
    return R::success(result).response_to_json();
}

#[put("/topic/deletes")]
pub async fn delete_topic_by_ids(
    jwt: JwtAdminRole,
    ids: Json<Vec<i64>>,
    service: Data<AppState>,
) -> impl Responder {
    let user_id = if jwt.user.role == "SUPER_ADMIN" {
        -1
    } else {
        jwt.user.id
    };
    let result = service
        .admin_service
        .delete_topics_ids(&ids.into_inner(), true, user_id)
        .await;
    return R::success(result).response_to_json();
}

#[put("/topic/un_delete/{id}")]
pub async fn un_delete_topic_by_id(
    jwt: JwtAdminRole,
    id: Path<i64>,
    service: Data<AppState>,
) -> impl Responder {
    let user_id = if jwt.user.role == "SUPER_ADMIN" {
        -1
    } else {
        jwt.user.id
    };
    let result = service
        .admin_service
        .delete_topics_ids(&vec![id.into_inner()], false, user_id)
        .await;
    return R::success(result).response_to_json();
}

#[put("/topic/un_deletes")]
pub async fn un_delete_topic_by_ids(
    jwt: JwtAdminRole,
    ids: Json<Vec<i64>>,
    service: Data<AppState>,
) -> impl Responder {
    let user_id = if jwt.user.role == "SUPER_ADMIN" {
        -1
    } else {
        jwt.user.id
    };
    let result = service
        .admin_service
        .delete_topics_ids(&ids.into_inner(), false, user_id)
        .await;
    return R::success(result).response_to_json();
}

#[get("/init_search")]
pub async fn init_search_blog(_: JwtSuperAdminRole, service: Data<AppState>) -> impl Responder {
    let blogs = service.blog_service.get_all_simple_blog().await;
    let json_value = serde_json::to_value(&blogs).unwrap();
    let _ = service
        .search_client
        .delete_all_documents(&CONFIG.blog_search_index)
        .await;
    let _ = service
        .search_client
        .save_documents(&CONFIG.blog_search_index, json_value)
        .await;
    return R::success("初始化搜索成功").response_to_json();
}

#[get("/init_latest")]
pub async fn init_latest_blog(_: JwtSuperAdminRole) -> impl Responder {
    get_pool_connection()
        .del::<&str, i64>(LATEST_BLOG_KEY)
        .unwrap();
    return R::success("初始化最新博客成功").response_to_json();
}

#[get("/init_eye_count")]
pub async fn init_blog_count(_: JwtSuperAdminRole, service: Data<AppState>) -> impl Responder {
    let _ = service.blog_service.init_blog_eye_couunt().await;
    return R::success("初始化浏览量成功").response_to_json();
}

#[derive(Deserialize, Debug)]
pub struct InfoDate {
    date: String,
    download:Option<bool>
}

#[get("/log")]
pub async fn get_log_info(_: JwtSuperAdminRole, query: Query<InfoDate>) -> HttpResponse {
    let query = query.into_inner();

    let mut path = CONFIG.logger.copy_path.to_owned() + "/" + &query.date + ".log";

    let today_str = Local::now().format("%Y-%m-%d").to_string();

    if today_str==query.date{
        path=CONFIG.logger.log_path.to_owned();
    }

    if fs::metadata(&mut path).is_err(){
        return HttpResponse::Ok().body("该日志不存在，可能不存在或删除了！");
    }

    let result = File::open(&mut path);

    if result.is_err() {
        return HttpResponse::Ok().json(E::error(
            Status::CHECK_DATA_ERROR,
            "文件打开失败".to_string(),
        ));
    }

    let mut str: String = String::new();
    let _ = result.unwrap().read_to_string(&mut str);
    if query.download.is_some() && query.download.unwrap(){
        return HttpResponse::Ok()
            .set_header(
                "Content-Disposition",
                format!("attachment; filename={}.log", "Yuice"),
            )
            .set_header("Content-Type", "application/octet-stream")
            .body(str);
    }else{
        return HttpResponse::Ok().body(str)
    }
}

use actix_web::{get, HttpRequest, HttpResponse, post, Responder};
use actix_web::web::{Data, Json, Path, Query};
use log::info;
use serde::Deserialize;

use crate::AppState;
use crate::common::result::R;
use crate::conf::config::CONFIG;
use crate::error::custom_error::{E, Status};
use crate::middleware::jwt::JwtAdminRole;
use crate::models::blogs::SearchBlogVo;
use crate::request::blog_request::{
    ArchiveRangeRequest, BlogFindRequest, BlogRequest, GetUserBlogRequest, SearchQueryRequest,
};
use crate::response::page_info::PageInfo;
use crate::search::meillsearch_response::Hits;

// 获取按类别列出博客列表
#[get("/list")]
pub async fn get_blog_by_category_list(
    req: Query<BlogFindRequest>,
    state: Data<AppState>,
) -> impl Responder {
    // 调用博客服务获取按类别的博客列表
    let result = state.blog_service.get_blog_list_by_category(&req.0).await;

    // 返回成功响应
    R::success(result).response_to_json()
}

// 获取按用户列出博客列表
#[get("/user/{id}")]
pub async fn get_blog_by_user_list(
    uid: Path<i64>,
    req: Query<GetUserBlogRequest>,
    state: Data<AppState>,
) -> impl Responder {
    // 调用博客服务获取按用户的博客列表
    let result = state
        .blog_service
        .get_blog_list_by_user(uid.into_inner(), req.into_inner())
        .await;

    // 返回成功响应
    R::success(result).response_to_json()
}

// 获取用户置顶博客
#[get("/user/top/{id}")]
pub async fn get_user_top_blog(uid: Path<i64>, state: Data<AppState>) -> impl Responder {
    // 调用博客服务获取用户置顶博客
    let result = state
        .blog_service
        .get_user_top_blog(&uid.into_inner())
        .await;

    // 返回成功响应
    R::success(result).response_to_json()
}

// 保存博客
#[post("/save_blog")]
pub async fn save_blog(
    jwt: JwtAdminRole,
    req: Json<BlogRequest>,
    state: Data<AppState>,
) -> Result<HttpResponse, E> {
    let req = &req.into_inner();
    // 调用博客服务保存博客
    let result = state.blog_service.add_blog(req, jwt.user.id).await;

    match result {
        Err(e) => {
            // 记录错误日志
            info!(
                "用户添加博客失败, 用户名: {}, 角色: {}",
                jwt.user.username, jwt.user.role
            );
            Err(e)
        }
        Ok(id) => {
            // 记录成功日志
            info!("用户添加博客成功, 用户名: {}, 角色: {}",jwt.user.username, jwt.user.role);
            let search_request = SearchBlogVo {
                id,
                title: req.title.to_owned(),
                description: req.description.to_owned(),
            };
            let json_value = serde_json::to_value(&search_request).unwrap();
            state
                .search_client
                .save_documents(&CONFIG.blog_search_index, json_value)
                .await;
            Ok(R::success("添加成功").response_to_json())
        }
    }
}

// 获取修改博客
#[get("/get_edit/{b_id}")]
pub async fn get_edit_blog(
    jwt: JwtAdminRole,
    b_id: Path<i64>,
    state: Data<AppState>,
) -> Result<HttpResponse, E> {
    let id = b_id.into_inner();
    let result = state.blog_service.get_edit_blog_info(id).await;

    match result {
        Some(r) => {
            // 记录成功日志
            info!("获取修改博客成功, 博客ID: {}, 用户ID: {}", id, jwt.user.id);
            Ok(R::success(r).response_to_json())
        }
        None => {
            // 记录错误日志
            info!("获取修改博客失败, 博客ID: {}, 用户ID: {}", id, jwt.user.id);
            Err(E::default())
        }
    }
}

// 更新博客
#[post("/update_blog/{b_id}")]
pub async fn update_blog(
    b_id: Path<i64>,
    jwt: JwtAdminRole,
    req: Json<BlogRequest>,
    state: Data<AppState>,
) -> Result<HttpResponse, E> {
    let mut u_id;

    if jwt.user.role == "SUPER_ADMIN" {
        u_id = -1;
    } else {
        u_id = jwt.user.id;
    }

    let mut blog_request = req.into_inner();

    blog_request.id = Option::from(b_id.into_inner());

    let result = state.blog_service.update_blog(blog_request, u_id).await;

    match result {
        Some(e) => {
            // 记录错误日志
            info!(
                "用户修改博客失败, 用户ID: {}, 用户名: {}",
                jwt.user.id, jwt.user.username
            );
            Err(e)
        }
        None => {
            // 记录成功日志
            info!(
                "用户修改博客成功, 用户ID: {}, 用户名: {}",
                jwt.user.id, jwt.user.username
            );
            Ok(R::success("修改成功").response_to_json())
        }
    }
}

// 获取热门博客列表
#[get("/hots")]
pub async fn get_hot_blogs_list(state: Data<AppState>) -> impl Responder {
    // 调用博客服务获取热门博客列表
    let result = state.blog_service.get_hot_blog().await;

    // 返回成功响应
    R::success(result).response_to_json()
}

// 获取最新博客列表
#[get("/latest")]
pub async fn get_latest_blogs_list(state: Data<AppState>) -> impl Responder {
    // 调用博客服务获取最新博客列表
    let result = state.blog_service.get_latest_blog().await;

    // 返回成功响应
    R::success(result).response_to_json()
}

// 获取推荐博客列表
#[get("/recommend")]
pub async fn get_recommend_blog(service: Data<AppState>) -> impl Responder {
    // 调用博客服务获取推荐博客列表
    let result = service.blog_service.get_recommend_blog().await;

    // 返回成功响应
    R::success(result).response_to_json()
}

// 根据博客ID获取博客信息
#[get("/get/{id}")]
pub async fn get_blog_info_by_id(
    req: HttpRequest,
    id: Path<i64>,
    state: Data<AppState>,
) -> Result<HttpResponse, E> {
    let id = id.into_inner();
    let result = state.blog_service.get_blog_by_id(id).await;

    if result.is_ok() {
        let mut blog = result.unwrap();

        // 调用博客服务增加博客访问量
        blog.eye_count = state
            .blog_service
            .increase_in_view(blog.eye_count, blog.id)
            .await;

        info!("获取博客 博客id:{} title:{}",id,blog.title.to_owned());

        // 返回成功响应
        Ok(R::success(blog).response_to_json())
    } else {
        // 返回错误响应
        Err(result.unwrap_err())
    }
}

// 根据日期范围获取归档博客列表
#[get("/range")]
pub async fn get_range_blog_list(
    range_request: Query<ArchiveRangeRequest>,
    state: Data<AppState>,
) -> impl Responder {
    let blog_range_request = range_request.get_range_request();

    // 调用博客服务获取归档博客列表
    let result = state
        .blog_service
        .get_archive_blog_by_range(&blog_range_request)
        .await;

    // 返回成功响应
    R::success(result).response_to_json()
}

// 获取用户保存的博客草稿
#[get("/get_save_edit")]
pub async fn get_save_edit_blog_content(
    jwt: JwtAdminRole,
    service: Data<AppState>,
) -> Result<HttpResponse, E> {
    let u_id = jwt.user.id;

    // 调用博客服务获取用户保存的博客草稿
    let result = service.blog_service.get_edit_blog(u_id).await;

    match result {
        Some(r) => {
            // 记录成功日志
            info!("获取保存的博客成功, 用户ID: {}", u_id);
            Ok(R::success(r).response_to_json())
        }
        None => {
            // 记录错误日志
            Err(E::error(
                Status::EDIT_ERROR,
                String::from("获取失败，可能不存在"),
            ))
        }
    }
}

// 设置用户保存的博客草稿
#[derive(Debug, Deserialize)]
pub struct GetSaveEditContent {
    content: String,
}

#[post("/set_save_edit")]
pub async fn set_save_edit_blog_content(
    content: Json<GetSaveEditContent>,
    jwt: JwtAdminRole,
    service: Data<AppState>,
) -> Result<HttpResponse, E> {
    let u_id = jwt.user.id;

    // 调用博客服务设置用户保存的博客草稿
    service
        .blog_service
        .set_edit_blog(u_id, &content.into_inner().content)
        .await;

    // 记录成功日志
    info!("保存博客成功, 用户ID: {}", u_id);

    // 返回成功响应
    Ok(R::success(String::from("保存成功")).response_to_json())
}

// 搜索博客列表
#[get("/search")]
pub async fn search_blog_list(
    req: Query<SearchQueryRequest>,
    state: Data<AppState>,
) -> impl Responder {
    let search_request = &req.into_inner();

    // 调用搜索客户端搜索博客列表
    let result = state
        .search_client
        .search_documents(&CONFIG.blog_search_index, search_request)
        .await;

    let mut page_info: PageInfo<Hits> = PageInfo {
        page:search_request.page,
        size: 10,
        total: 0,
        data: vec![],
    };

    if let Some(r) = result {
        page_info.size = r.limit;
        page_info.data = r.hits;
        page_info.total = r.total_hits;
        info!("搜索博客 关键字: {}",search_request.keyword);
        // 返回成功响应
        R::success(page_info).response_to_json()
    } else {
        // 返回错误响应
        R::success(E::default()).response_to_json()
    }
}

// 获取相似的博客列表
#[get("/similar")]
pub async fn get_similar_blog(
    req: Query<SearchQueryRequest>,
    state: Data<AppState>,
) -> impl Responder {
    let mut request = req.into_inner();
    request.page = 1;

    // 调用搜索客户端获取相似的博客列表
    let result = state
        .search_client
        .search_documents(&CONFIG.blog_search_index, &request)
        .await;

    if let Some(r) = result {
        // 返回成功响应
        R::success(r.hits).response_to_json()
    } else {
        let vec: Vec<Hits> = Vec::new();

        // 返回成功响应
        R::success(vec).response_to_json()
    }
}

// 初始化博客搜索索引
#[get("/init_search")]
pub async fn init_search_blog(state: Data<AppState>) -> Result<HttpResponse, E> {
    // 调用博客服务获取所有简化的博客数据
    let simple_blogs = state.blog_service.get_all_simple_blog().await;

    let json_value = serde_json::to_value(&simple_blogs).unwrap();

    // 调用搜索客户端保存博客数据到搜索索引
    let result = state
        .search_client
        .save_documents(&CONFIG.blog_search_index, json_value)
        .await;

    match result {
        None => {
            // 记录成功日志
            info!("初始化博客搜索索引成功");
            Ok(R::success("索引初始化成功").response_to_json())
        }
        Some(e) => {
            // 返回错误响应
            Err(e)
        }
    }
}

// 创建新的搜索索引
#[get("/create_index/{index_name}")]
pub async fn create_search_index(
    index_name: Path<String>,
    state: Data<AppState>,
) -> Result<HttpResponse, E> {
    let name = &index_name.into_inner();

    // 调用搜索客户端创建新的搜索索引
    let result = state.search_client.create_index(name).await;

    if result.is_none() {
        // 记录成功日志
        Ok(R::success(format!("创建搜索索引 {} 成功", name)).response_to_json())
    } else {
        // 返回错误响应
        Err(result.unwrap())
    }
}

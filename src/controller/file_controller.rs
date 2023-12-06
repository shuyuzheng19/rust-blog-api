use actix_multipart::Multipart;
use actix_web::{get, HttpResponse, post, Responder};
use actix_web::web::{Data, Json, Query};
use log::{error, info};
use serde::Deserialize;

use crate::AppState;
use crate::common::constants::default_page;
use crate::common::get_file_extension;
use crate::common::result::R;
use crate::conf::config::CONFIG;
use crate::error::custom_error::E;
use crate::middleware::jwt::JwtAdminRole;
use crate::models::file::{FileCheckRequest, FileInfo};

// 上传请求参数结构
#[derive(Deserialize, Debug)]
pub struct UploadReq {
    pub is_pub: bool,
}

// 处理文件上传请求
#[post("/upload")]
async fn upload_file(
    upload_req: Query<UploadReq>,
    jwt: JwtAdminRole,
    file: Multipart,
    service: Data<AppState>,
) -> Result<HttpResponse, E> {
    // 提取上传请求的参数
    let req = upload_req.into_inner();

    // 记录信息日志，表示文件上传请求已经接收
    info!("收到文件上传请求，公开: {}", req.is_pub);

    // 调用文件服务处理文件上传
    let result = service
        .file_service
        .upload_file(
            CONFIG.upload.files.to_owned(),
            req.is_pub,
            jwt.user.id,
            file,
        )
        .await;

    return match result {
        Ok(r) => {
            // 记录信息日志，表示文件上传成功
            info!("文件上传成功: {:?}", r);
            Ok(R::success(r).response_to_json())
        }
        Err(e) => {
            // 记录错误日志，表示文件上传失败
            error!("文件上传失败: {:?}", e);
            Err(e)
        }
    };
}

// 处理图片上传请求
#[post("/upload/image")]
async fn upload_image(
    jwt: JwtAdminRole,
    file: Multipart,
    service: Data<AppState>,
) -> Result<HttpResponse, E> {
    // 记录信息日志，表示图片上传请求已经接收
    info!("收到图片上传请求");

    // 调用文件服务处理图片上传
    let result = service
        .file_service
        .upload_file(CONFIG.upload.image.to_owned(), false, jwt.user.id, file)
        .await;

    return match result {
        Ok(r) => {
            // 记录信息日志，表示图片上传成功
            info!("图片上传成功: {:?}", r);
            Ok(R::success(r).response_to_json())
        }
        Err(e) => {
            // 记录错误日志，表示图片上传失败
            error!("图片上传失败: {:?}", e);
            Err(e)
        }
    };
}

#[post("/check")]
async fn check_md5_and_insert_file(
    jwt: JwtAdminRole,
    is_pub: Query<UploadReq>,
    req: Json<FileCheckRequest>,
    service: Data<AppState>,
) -> impl Responder {
    let md5 = req.md5.to_owned();
    let result = service.file_service.find_by_md5(&md5).await;
    if result.is_none() {
        return HttpResponse::Ok().json(E::default());
    }
    let mut file_info = FileInfo::new();
    file_info.user_id = jwt.user.id;
    let req = req.into_inner();
    let name = req.name.to_owned();
    let suffix = get_file_extension(&name);
    let new_name = format!("{}.{}", md5, suffix);
    file_info.suffix = suffix.to_owned();
    file_info.new_name = new_name;
    file_info.old_name = name;
    file_info.md5 = md5;
    file_info.is_public = is_pub.into_inner().is_pub;
    let result = service.file_service.insert_already_file(&file_info).await;
    return if let Some(e) = result {
        return HttpResponse::Ok().json(e);
    } else {
        return R::success("验证成功").response_to_json();
    };
}

// 处理头像上传请求
#[post("/upload/avatar")]
async fn upload_avatar(file: Multipart, service: Data<AppState>) -> Result<HttpResponse, E> {
    // 记录信息日志，表示头像上传请求已经接收
    info!("收到头像上传请求");

    // 调用文件服务处理头像上传
    let result = service
        .file_service
        .upload_file(CONFIG.upload.avatar.to_owned(), false, -1, file)
        .await;

    return match result {
        Ok(r) => {
            // 记录信息日志，表示头像上传成功
            info!("头像上传成功: {:?}", r);
            Ok(R::success(r).response_to_json())
        }
        Err(e) => {
            // 记录错误日志，表示头像上传失败
            error!("头像上传失败: {:?}", e);
            Err(e)
        }
    };
}

#[derive(Deserialize, Debug)]
pub struct FileFindRequest {
    #[serde(default = "default_page")]
    pub page: i64,
    pub keyword: Option<String>,
    pub sort: String,
}

#[get("/current")]
async fn find_file_by_page_current(
    jwt: JwtAdminRole,
    req: Query<FileFindRequest>,
    service: Data<AppState>,
) -> impl Responder {
    let result = service
        .file_service
        .find_file_by_page(jwt.user.id, &req.into_inner())
        .await;
    return R::success(result).response_to_json();
}

#[get("/public")]
async fn find_file_by_page_public(
    req: Query<FileFindRequest>,
    service: Data<AppState>,
) -> impl Responder {
    let result = service
        .file_service
        .find_file_by_page(-1, &req.into_inner())
        .await;
    return R::success(result).response_to_json();
}

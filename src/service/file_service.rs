use std::fs::File;
use std::io::Write;
use std::sync::Arc;

use actix_web::body::MessageBody;
use futures::{StreamExt, TryStreamExt};
use log::error;
use sqlx::{Pool, Postgres};

use crate::common::{get_file_extension, is_image_file};
use crate::conf::config::CONFIG;
use crate::controller::file_controller::FileFindRequest;
use crate::error::custom_error::{E, Status};
use crate::models::file::{FileInfo, FileVo};
use crate::repository::file_repository::FileRepository;
use crate::response::page_info::PageInfo;

pub struct FileService(Arc<FileRepository>);

impl FileService {
    pub fn new(db_conn: Pool<Postgres>) -> FileService {
        let file_repository = FileRepository::new(db_conn);
        FileService(Arc::new(file_repository))
    }
    pub async fn upload_file(
        &self,
        u_type: String,
        is_pub: bool,
        user_id: i64,
        mut payload: actix_multipart::Multipart,
    ) -> Result<Vec<String>, E> {
        let mut urls: Vec<String> = Vec::new();

        while let Some(mut item) = payload.try_next().await.unwrap() {
            let content_disposition = item.content_disposition().clone();
            let file_name = content_disposition
                .get_filename()
                .unwrap_or_default()
                .to_string();
            let mut is_image = false;
            let mut file_bytes = Vec::new();

            let mut size: i64 = 0;
            let mut suffix: String = String::from("");

            if u_type == CONFIG.upload.avatar {
                // 如果是头像上传，需检查是否为图片文件
                if !is_image_file(&file_name) {
                    return Err(E::error(
                        Status::CHECK_DATA_ERROR,
                        String::from("这不是一个图片文件"),
                    ));
                }

                // 读取上传文件的内容
                while let Some(mut data) = item.next().await {
                    let data = data.unwrap();
                    file_bytes.extend_from_slice(&data);
                }

                size = file_bytes.len() as i64;
                suffix = get_file_extension(&file_name);

                // 检查图片文件大小是否超出限制
                if size > CONFIG.upload.max_file_size * 1024 * 1024 {
                    return Err(E::error(
                        Status::CHECK_DATA_ERROR,
                        String::from("图片文件大小超出"),
                    ));
                }
                is_image = true
            } else if u_type == CONFIG.upload.image {
                // 检查是否为图片文件（如果需要）

                if is_image && !is_image_file(&file_name) {
                    return Err(E::error(
                        Status::CHECK_DATA_ERROR,
                        String::from("这不是一个图片文件"),
                    ));
                }

                // 读取上传文件的内容
                while let Some(mut data) = item.next().await {
                    let data = data.unwrap();
                    file_bytes.extend_from_slice(&data);
                }

                size = file_bytes.len() as i64;
                suffix = get_file_extension(&file_name);

                // 检查文件大小是否超出限制
                if size > CONFIG.upload.max_image_size * 1024 * 1024 {
                    return Err(E::error(
                        Status::CHECK_DATA_ERROR,
                        String::from("文件大小超出"),
                    ));
                }

                is_image = true;
            } else {
                // 读取上传文件的内容
                while let Some(mut data) = item.next().await {
                    if let Err(l) = data{
                        continue
                    }else{
                        file_bytes.extend_from_slice(&data.unwrap());
                    }
                }

                size = file_bytes.len() as i64;
                suffix = get_file_extension(&file_name);

                // 检查文件大小是否超出限制
                if size > CONFIG.upload.max_file_size * 1024 * 1024 {
                    return Err(E::error(
                        Status::CHECK_DATA_ERROR,
                        String::from("文件大小超出"),
                    ));
                }
            }

            let md5 = format!("{:x}", md5::compute(&file_bytes));
            let mut file_info = FileInfo {
                id: 0,
                user_id,
                old_name: file_name.clone(),
                new_name: format!("{}.{}", &md5, suffix),
                create_at: Default::default(),
                size,
                suffix,
                absolute_path: String::new(),
                is_public: is_pub,
                md5: md5,
                url: String::new(),
            };

            // 检查文件是否已存在
            let url = self.0.find_by_md5_to_url(&file_info.md5).await;
            if url.is_some() {
                // 如果已存在，直接使用已有的URL
                file_info.url = url.unwrap();
                urls.push(file_info.url.to_owned());
                if !is_image {
                    &self.0.insert_already_file(&file_info);
                }
            } else {
                file_info.absolute_path =
                    format!("{}/{}/{}", CONFIG.upload.path, u_type, file_info.new_name);
                let file_creation_result = File::create(&file_info.absolute_path);

                if file_creation_result.is_err() {
                    error!("文件创建失败: {}", file_creation_result.unwrap_err());
                    return Err(E::error(
                        Status::UPLOAD_FILE_ERROR,
                        "文件上传失败,上传已被终止".to_string(),
                    ));
                }

                let mut file = file_creation_result.unwrap();

                if let Err(e) = file.write_all(&file_bytes) {
                    error!("文件写入失败: {}", e.to_string());
                    return Err(E::error(
                        Status::UPLOAD_FILE_ERROR,
                        "文件上传失败,上传已被终止".to_string(),
                    ));
                }

                file_info.url = format!("{}{}/{}", CONFIG.upload.uri, u_type, &file_info.new_name);
                urls.push(file_info.url.clone());
                if !is_image {
                    self.0.insert_file(&mut file_info).await;
                }
            }
        }

        Ok(urls)
    }

    pub async fn find_file_by_page(&self, uid: i64, req: &FileFindRequest) -> PageInfo<FileVo> {
        return self.0.find_file_by_page(uid, req).await;
    }

    pub async fn find_by_md5(&self, md5: &String) -> Option<String> {
        return self.0.find_by_md5_to_url(md5).await;
    }

    pub async fn insert_already_file(&self, file_info: &FileInfo) -> Option<E> {
        return self.0.insert_already_file(file_info).await;
    }
}

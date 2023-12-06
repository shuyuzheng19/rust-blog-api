extern crate core;

use std::fmt::{Debug, Display};
use std::{fs, time};
use std::fs::{File, OpenOptions};
use std::future::Future;
use std::io::{Read, Write};
use std::ops::Add;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use actix_cors::Cors;
use actix_web::{App, FromRequest, HttpServer, Responder, ResponseError};
use actix_web::dev::{Service, ServiceFactory, Transform};
use actix_web::guard::Guard;
use actix_web::http::header;
use actix_web::middleware::ErrorHandlers;
use actix_web::rt::time::{interval, sleep, sleep_until};
use actix_web::web::{Data, scope};
use serde::Serialize;
use sqlx::{Connection, FromRow, Pool, Postgres, Row};
use sqlx::types::chrono::{Local, TimeZone, Utc};

use crate::conf::config::CONFIG;
use crate::conf::logger_config::LoggerParams;
use crate::search::meilisearch_client::MeiliSearchClient;
use crate::service::admin_service::AdminService;
use crate::service::blog_service::BlogService;
use crate::service::category_service::CategoryService;
use crate::service::file_service::FileService;
use crate::service::gpt_service::GptService;
use crate::service::tag_service::TagService;
use crate::service::topic_service::TopicService;
use crate::service::user_service::UserService;

mod cache;
mod common;
mod conf;
mod controller;
mod error;
mod middleware;
mod models;
mod request;
mod response;
mod repository;
mod routers;
mod search;
mod service;
pub struct AppState {
    pub user_service: Arc<UserService>,
    pub blog_service: Arc<BlogService>,
    pub category_service: Arc<CategoryService>,
    pub search_client: Arc<MeiliSearchClient>,
    pub tag_service: Arc<TagService>,
    pub topic_service: Arc<TopicService>,
    pub file_service: Arc<FileService>,
    pub admin_service: Arc<AdminService>,
    pub chat_service: Arc<Mutex<GptService>>,
}

struct Connections {
    db_pool: Pool<Postgres>,
    // redis_conn:Arc<r2d2::Pool<r2d2_redis::RedisConnectionManager>>
}

impl Connections {
    async fn new() -> Self {
        let db = CONFIG.db.get_db().await;
        // let redis_pool = CONFIG.redis.get_redis_pool().await;
        return Self { db_pool: db };
    }
}

fn init_log() {
    println!("更新日志");
    let now = sqlx::types::chrono::Local::now();
    let now_sub = (now + Duration::from_secs(60 * 60 * 24))
        .date()
        .and_hms(0, 0, 0);
    let copy_path =
        CONFIG.logger.copy_path.to_owned() + "/" + &now_sub.format("%Y-%m-%d").to_string() + ".log";
    let mut result = File::create(copy_path).expect("create copy log path error");
    let mut f = OpenOptions::new().write(true).read(true).open(&CONFIG.logger.log_path).expect("open log file error");
    let mut content: String = String::new();
    f.read_to_string(&mut content).expect("read log content error");
    result.write_all(content.as_bytes()).expect("write new log file error");
    f.set_len(0).expect("clear log file error");
}

fn get_last_time_seconds()->u64{
    let current_time = sqlx::types::chrono::Local::now();

    let last_time = current_time.date().and_hms(0,0,0)+Duration::from_secs(60*60*24);

    let duration_until_midnight = last_time-current_time;

    return duration_until_midnight.num_seconds() as u64
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {


    CONFIG.logger.init();

    fs::create_dir_all(CONFIG.upload.path.to_owned() + "/" + &CONFIG.upload.image).unwrap();

    fs::create_dir_all(CONFIG.upload.path.to_owned() + "/" + &CONFIG.upload.avatar).unwrap();

    fs::create_dir_all(CONFIG.upload.path.to_owned() + "/" + &CONFIG.upload.files).unwrap();

    let mut connections = Connections::new().await;

    let user_service = Arc::new(UserService::new(connections.db_pool.clone()));

    let blog_service = Arc::new(BlogService::new(connections.db_pool.clone()));

    let search_config = CONFIG.meilisearch.get_search_client();

    let search_client = Arc::new(search_config);

    let category_service = Arc::new(CategoryService::new(connections.db_pool.clone()));

    let tag_service = Arc::new(TagService::new(connections.db_pool.clone()));

    let topic_service = Arc::new(TopicService::new(connections.db_pool.clone()));

    let file_service = Arc::new(FileService::new(connections.db_pool.clone()));

    let admin_service = Arc::new(AdminService::new(connections.db_pool.clone()));

    let chat_service = Arc::new(Mutex::new(GptService::new()));

    actix_web::rt::spawn({
        let blog_service_clone = blog_service.clone();
        async move {

            sleep(Duration::from_secs(get_last_time_seconds())).await;

            let mut interval = actix_web::rt::time::interval(Duration::from_secs(60*60*24));

            loop {
                interval.tick().await;
                println!("定时任务更新 更新时间{}",Local::now().to_rfc3339());
                blog_service_clone.init_blog_eye_couunt().await;
                init_log();
            }
        }
    });

    HttpServer::new(move || {
        let mut cors = Cors::default();
        let urls = CONFIG.origin.to_url_vec();
        for  url in urls {
            cors=cors.allowed_origin(url.as_str());
        }

        let methods: Vec<&str> = CONFIG.origin.methods.split(",").collect();

        cors=cors.allowed_methods(methods)
            .allowed_headers(vec![
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
        ])
        .supports_credentials();

        let scope = scope("/api/v1")
            .configure(routers::user_router)
            .configure(routers::blog_router)
            .configure(routers::category_router)
            .configure(routers::tag_router)
            .configure(routers::topic_router)
            .configure(routers::file_router)
            .configure(routers::admin_router);

        let app_data = Data::new(AppState {
            blog_service: blog_service.clone(),
            user_service: user_service.clone(),
            search_client: search_client.clone(),
            category_service: category_service.clone(),
            tag_service: tag_service.clone(),
            topic_service: topic_service.clone(),
            file_service: file_service.clone(),
            admin_service: admin_service.clone(),
            chat_service: chat_service.clone(),
        });

        let error_middleware =
            ErrorHandlers::new().default_handler(error::custom_error::default_error);
        App::new()
            .app_data(app_data)
            .service(scope)
            .wrap(cors)
            .wrap(error_middleware)
            .wrap(LoggerParams::new())
    })
    .bind(format!(
        "{}:{}",
        CONFIG.server.addr.to_owned(),
        CONFIG.server.port
    ))?
    .run()
    .await
}

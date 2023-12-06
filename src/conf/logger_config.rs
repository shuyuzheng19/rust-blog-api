use std::fmt::Debug;
use actix_web::http::Method;
use actix_web::middleware::Logger;
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use simplelog::{CombinedLogger, ConfigBuilder, WriteLogger};

use crate::common::{get_client_ip_city, get_client_platform_info, get_ip_address};
use crate::conf::config::CONFIG;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoggerConfig {
    pub log_path: String,
    pub disable_error:bool,
    pub copy_path: String,
    pub regex:String,
    pub excludes:String
}

pub struct LoggerParams{logger:Logger,format:String}

impl LoggerParams {
    pub fn new()->Logger{
        let format = "打印客户端信息: %{IP_CITY_USER_AGENT}xi | [%D]ms/%s".to_string();
        let mut logger = Logger::new(&format).custom_request_replace("IP_CITY_USER_AGENT", |req|{
            let request = &req.request();
            let method = request.method();
            let path= request.uri();
            if method==Method::OPTIONS{
                return format!("{} 这是预检请求.....",path)
            }
            let ip = get_ip_address(request);
            let platform = get_client_platform_info(
                req.headers()
                    .get("User-Agent")
                    .unwrap()
                    .to_str()
                    .unwrap_or("未知"),
            );
            let city = get_client_ip_city(&ip);
            return format!("{} | {} | {} | {} | {}",path,platform,ip,city,method)
        });
        let regex_list = CONFIG.logger.regex.split(",");

        for regex in regex_list {
            logger=logger.exclude_regex(regex);
        }

        let excludes = CONFIG.logger.excludes.split(",");

        for exclude in excludes {
            logger=logger.exclude(exclude);
        }

        return logger
    }
}

impl LoggerConfig {
    pub fn init(&self) {
        let controller_file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)
            .unwrap();

        let mut controller_log_config_build = ConfigBuilder::new();

        controller_log_config_build.set_time_offset_to_local().unwrap();

        // controller_log_config_build.set_target_level(LevelFilter::Info);

        controller_log_config_build.set_max_level(LevelFilter::Info);

        if CONFIG.logger.disable_error{
            controller_log_config_build.add_filter_ignore_str("rust_blog::error");
        }

        // controller_log_config_build.add_filter_allow_str("rust-blog-api::controller");

        // controller_log_config_build.add_filter_allow_str("rust-blog-api::repository");

        // controller_log_config_build.add_filter_allow_str("rust-blog-api::error");
        //
        // let mut console_log_config_build = ConfigBuilder::new();
        //
        // console_log_config_build.set_time_offset_to_local().unwrap();
        //
        // console_log_config_build.set_max_level(LevelFilter::Error);
        //
        // console_log_config_build.set_time_format_rfc3339();
        //
        // console_log_config_build.set_target_level(LevelFilter::Info);
        //
        // console_log_config_build.add_filter_ignore_str("rust-blog-api::controller");
        //
        // console_log_config_build.add_filter_ignore_str("rust-blog-api::repository");
        //
        // console_log_config_build.add_filter_ignore_str("rust-blog-api::error");

        CombinedLogger::init(vec![
            WriteLogger::new(
                LevelFilter::Info,
                controller_log_config_build.build(),
                controller_file
            ),
            // TermLogger::new(LevelFilter::Info, console_log_config_build.build(), TerminalMode::Mixed, ColorChoice::Auto),
        ])
        .unwrap();
    }
}

use std::{env, fs};

use lazy_static::lazy_static;
use serde::Deserialize;

use crate::conf::db_config::DbConfig;
use crate::conf::logger_config::LoggerConfig;
use crate::conf::redis_config::RedisConfig;
use crate::conf::search_config::MeiliSearchConfig;
use crate::conf::smtp_config::SmtpConfig;
use crate::conf::token_config::TokenConfig;
use crate::conf::upload_config::UploadConfig;

#[derive(Debug, Deserialize)]
pub struct GptToken {
    pub token: String,
    pub api: String,
    pub cookie:String
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub ip2region_path: String,
    pub blog_search_index:String,
    pub db: DbConfig,
    pub meilisearch: MeiliSearchConfig,
    pub token: TokenConfig,
    pub smtp: SmtpConfig,
    pub redis: RedisConfig,
    pub logger: LoggerConfig,
    pub my_email: String,
    pub blog_page_cache: bool,
    pub blog_page_cache_expire: usize,
    pub upload: UploadConfig,
    pub gpt: GptToken,
    pub server: ServerConfig,
    pub origin:OriginConfig
}

#[derive(Debug, Deserialize)]
pub struct OriginConfig{
    pub urls:String,
    pub methods:String
}

impl OriginConfig {
    pub fn to_url_vec(&self)->Vec<String>{
        let result = self.urls.split(",").map(|s|s.to_string()).collect();
        return result
    }
    // pub fn to_method_vec(self) -> Vec<&'static str> {
    //     let methods = &self.methods;
    //     let result:Vec<&'static str> = methods.split(",").collect();
    //     result
    // }
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub addr: String,
    pub port: i64,
}

lazy_static! {
    pub static ref CONFIG: Config = {
        let config_path = if cfg!(debug_assertions) {
            env::current_dir().unwrap().join("config.yaml")
        } else {
            env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .join("config.yaml")
        };

        println!("{:?}", config_path);

        let config_contents = fs::read_to_string(config_path).expect("Failed to read config file");
        return serde_yaml::from_str(&config_contents).unwrap();
    };
}

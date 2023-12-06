use std::collections::HashMap;

use actix_web::HttpRequest;
use log::info;
use serde_json::{json, Value};

use crate::common::{get_client_ip_city, get_client_platform_info, get_ip_address};

pub mod admin_controller;
pub mod blog_controller;
pub mod category_controller;
pub mod file_controller;
pub mod tag_controller;
pub mod topic_controller;
pub(crate) mod user_controller;

struct LogMap(String, Value);

pub struct LogMapBuild {
    map: HashMap<String, Value>,
}

impl LogMapBuild {
    pub fn new(key: String, value: Value) -> Self {
        let mut map: HashMap<String, Value> = HashMap::new();
        map.insert(key, value);
        return Self { map };
    }

    pub fn insert(mut self, key: String, value: Value) -> Self {
        &self.map.insert(key, value);
        return self;
    }

    pub fn build(self) -> HashMap<String, Value> {
        return self.map;
    }
}

fn print_log_info(info: &str, req: &HttpRequest, value: Option<Value>) {
    let ip = get_ip_address(req);

    let city = get_client_ip_city(&ip);

    let platform = get_client_platform_info(
        req.headers()
            .get("User-Agent")
            .unwrap()
            .to_str()
            .unwrap_or("未知"),
    );

    let path = req.uri().to_string();

    let json = json!({
        "ip":ip,
        "city":city,
        "platform": platform,
        "other": if value.is_none(){
            Value::Null
        }else{
            value.unwrap()
        },
        "path":path,
    });

    info!("{}: {}", info, json)
}

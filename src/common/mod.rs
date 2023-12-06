use actix_web::HttpRequest;
use lazy_static::lazy_static;
use rand::Rng;
use regex::Regex;

use crate::conf::config::CONFIG;

pub mod constants;
pub mod date_format;
pub mod redis_keys;
pub(crate) mod result;

// 定义用于匹配邮箱地址的正则表达式
lazy_static! {
    static ref EMAIL_REG: String =
        String::from(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$");
}

// 定义用于匹配图片文件扩展名的正则表达式
lazy_static! {
    static ref IMAGE_REG: String = String::from(r#"(?i)\.(jpg|jpeg|png|gif|bmp)$"#);
}

// 创建 IP2Region 搜索器实例
lazy_static! {
    static ref SEARCHER: ip2region::Searcher =
        ip2region::Searcher::new(&CONFIG.ip2region_path.to_owned()).unwrap();
}

// 验证邮箱地址是否有效
pub fn is_valid_email(email: &str) -> bool {
    let email_regex = Regex::new(&EMAIL_REG).unwrap();
    return email_regex.is_match(email);
}

// 判断 URL 是否为图片
pub fn is_image_url(img_url: &str) -> bool {
    let regex = Regex::new(&IMAGE_REG).unwrap();
    return regex.is_match(img_url);
}

// 判断文件名是否为图片文件
pub fn is_image_file(file_name: &str) -> bool {
    let suffix = get_file_extension(file_name);
    let allowed_extensions = ["jpg", "jpeg", "png", "gif"];
    return allowed_extensions.contains(&suffix.as_str());
}

// 获取客户端 IP 对应的城市信息
pub fn get_client_ip_city(ip: &str) -> String {
    return match SEARCHER.search(ip) {
        Ok(e) => {
            let r: Vec<&str> = e.split("|").collect();
            return format!("{} {} {}", r[0], r[2], r[3]);
        }
        Err(e) => String::from("未知"),
    };
}

// 获取客户端 IP 对应的城市信息（用于 HTTP 头部）
pub fn get_client_ip_city_for_header(ip: &str) -> String {
    return match SEARCHER.search(ip) {
        Ok(e) => e,
        Err(e) => String::from("未知"),
    };
}

// 获取文件扩展名
pub fn get_file_extension(file_name: &str) -> String {
    if let Some(dot_index) = file_name.rfind('.') {
        // 提取最后一个点之后的部分
        let extension = &file_name[dot_index + 1..];
        extension.to_string()
    } else {
        "".to_string()
    }
}

// 生成随机验证码（6位整数）
pub fn get_random_code_number() -> String {
    let mut rng = rand::thread_rng();
    let min_value = 100_000; // 最小值为 100000
    let max_value = 999_999; // 最大值为 999999
    let random_number = rng.gen_range(min_value..=max_value);
    return random_number.to_string();
}

// 获取客户端平台信息
pub fn get_client_platform_info(user_agent: &str) -> String {

    let mut os = String::new();
    let mut browser = String::new();

    if !user_agent.is_empty() {
        let user_agent = user_agent.to_lowercase();
        if user_agent.contains("windows") {
            os = String::from("Windows");
        } else if user_agent.contains("mac") {
            os = String::from("Mac");
        } else if user_agent.contains("android") {
            os = String::from("Android");
        } else if user_agent.contains("iphone") || user_agent.contains("ipad") {
            os = String::from("iOS");
        } else {
            return user_agent.to_string();
        }

        if user_agent.contains("micromessenger") {
            browser = String::from("微信客户端");
        } else if user_agent.contains("edg") {
            browser = String::from("Edge");
        } else if user_agent.contains("chrome") {
            browser = String::from("Chrome");
        } else if user_agent.contains("firefox") {
            browser = String::from("Firefox");
        } else if user_agent.contains("safari") {
            browser = String::from("Safari");
        }

        return format!("{} {}", os, browser);
    } else {
        return String::from("Unknown");
    }
}

pub fn get_size_str(size: f64) -> String {
    if size == 0.0 {
        return "0 B".to_string();
    }

    let mut size_str = String::new();

    if size < 1024.0 {
        size_str.push_str(&format!("{:.0}", size));
        size_str.push_str(" BIT");
    } else if size < 1024.0 * 1024.0 {
        size_str.push_str(&format!("{:.2}", size / 1024.0));
        size_str.push_str(" KB");
    } else if size < 1024.0 * 1024.0 * 1024.0 {
        size_str.push_str(&format!("{:.2}", size / (1024.0 * 1024.0)));
        size_str.push_str(" MB");
    } else if size < 1024.0 * 1024.0 * 1024.0 * 1024.0 {
        size_str.push_str(&format!("{:.2}", size / (1024.0 * 1024.0 * 1024.0)));
        size_str.push_str(" GB");
    } else {
        size_str.push_str(&format!(
            "{:.2}",
            size / (1024.0 * 1024.0 * 1024.0 * 1024.0)
        ));
        size_str.push_str(" TB");
    }

    size_str
}

pub fn get_body_content_length(req: &HttpRequest) -> i64 {
    if let Some(content_length) = req.headers().get("content-length") {
        if let Ok(content_length_str) = content_length.to_str() {
            if let Ok(content_length) = content_length_str.parse::<i64>() {
                return content_length;
            }
        }
    }
    return 0;
}

pub fn get_user_agent(request:&HttpRequest)->String{
    let headers = request.headers();

    if let Some(user_agent) = headers.get("UserAgent") {
        if let Ok(result) = user_agent.to_str() {
            return result.to_string()
        }
    }
    return "未知".to_string()
}
pub fn get_ip_address(request: &HttpRequest) -> String {
    let headers = request.headers();

    if let Some(ip_address) = headers.get("X-Forwarded-For") {
        if let Ok(ip_address) = ip_address.to_str() {
            if ip_address != "" && ip_address.to_lowercase() != "unknown" {
                return ip_address.to_string();
            }
        }
    }

    if let Some(ip_address) = headers.get("Proxy-Client-IP") {
        if let Ok(ip_address) = ip_address.to_str() {
            if ip_address != "" && ip_address.to_lowercase() != "unknown" {
                return ip_address.to_string();
            }
        }
    }

    if let Some(ip_address) = headers.get("WL-Proxy-Client-IP") {
        if let Ok(ip_address) = ip_address.to_str() {
            if ip_address != "" && ip_address.to_lowercase() != "unknown" {
                return ip_address.to_string();
            }
        }
    }

    // 如果以上都没有找到有效的 IP 地址，则使用 RemoteAddr
    request
        .connection_info()
        .realip_remote_addr()
        .map(|ip_address| ip_address.to_string())
        .unwrap_or_else(|| "".to_string())
}

use actix_web::cookie::time::Duration;
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Local;

use crate::conf::config::CONFIG;

// 序列化和反序列化 Token 配置
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenConfig {
    pub secret: String,
    pub expire: i64,
}

// JWT 的声明（Claims）
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,      // 过期时间
    iat: usize,      // 签发时间
    iss: i64,        // 签发者
    pub sub: String, // 主题（通常是用户名）
}

impl TokenConfig {
    // 创建 JWT Token
    pub async fn create_token(&self, user_id: i64, username: &String) -> String {
        let current_time = Local::now().timestamp();

        // 计算过期时间
        let expiration_time = current_time + Duration::days(CONFIG.token.expire).whole_seconds();

        let claims = Claims {
            exp: expiration_time as usize,
            iat: current_time as usize,
            iss: user_id,
            sub: username.to_owned(),
        };

        // 使用密钥编码 JWT
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        );

        // 返回 JWT 字符串
        return token.unwrap();
    }

    // 解析 JWT Token
    pub fn parse_token(&self, token: &String) -> Option<Claims> {
        let result = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        );

        if result.is_err() {
            return None;
        }

        // 解析成功，返回声明
        return Some(result.unwrap().claims);
    }
}

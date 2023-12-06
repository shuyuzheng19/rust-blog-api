use r2d2_redis::redis::{Commands, RedisError};

use crate::common::redis_keys::{
    BLOG_WEB_CONFIG, DAY, EMAIL_CODE_KEY, EMAIL_CODE_KEY_EXPIRE, USER_INFO_KEY,
    USER_INFO_KEY_EXPIRE, USER_TOKEN_KEY,
};
use crate::conf::config::CONFIG;
use crate::conf::redis_config::get_pool_connection;
use crate::models::user::UserVo;
use crate::response::website_info::BlogConfigInfo;

// 用户缓存结构
pub struct UserCache {}

impl UserCache {
    pub fn new() -> UserCache {
        return UserCache {};
    }

    // 设置用户信息到 Redis
    pub fn set_user(&self, username: &String, user: &Option<UserVo>) -> bool {
        return get_pool_connection()
            .set_ex::<String, String, String>(
                USER_INFO_KEY.to_owned() + username,
                serde_json::to_string(user).unwrap(),
                USER_INFO_KEY_EXPIRE,
            )
            .is_ok();
    }

    // 从 Redis 获取用户信息
    pub fn get_user(&self, username: &String) -> Result<Option<UserVo>, RedisError> {
        let result =
            get_pool_connection().get::<String, String>(USER_INFO_KEY.to_owned() + username);
        match result {
            Ok(r) => Ok(serde_json::from_str(&r).unwrap()),
            Err(e) => Err(e),
        }
    }

    // 设置邮箱验证码到 Redis
    pub fn set_email_code(&self, ip: &String, code: &String) -> bool {
        return get_pool_connection()
            .set_ex::<String, &String, String>(
                EMAIL_CODE_KEY.to_owned() + ip,
                code,
                EMAIL_CODE_KEY_EXPIRE,
            )
            .is_ok();
    }

    // 从 Redis 获取邮箱验证码
    pub fn get_email_code(&self, email: &String) -> Result<String, RedisError> {
        let result = get_pool_connection().get::<String, String>(EMAIL_CODE_KEY.to_owned() + email);
        return match result {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
        };
    }

    // 从 Redis 获取网站配置
    pub fn get_website_config(&self) -> Option<BlogConfigInfo> {
        let result = get_pool_connection().get::<String, String>(BLOG_WEB_CONFIG.to_owned());
        return match result {
            Ok(e) => Some(serde_json::from_str(&e).unwrap()),
            Err(e) => None,
        };
    }

    // 设置网站配置到 Redis
    pub fn set_website_config(&self, config: BlogConfigInfo) -> bool {
        return get_pool_connection()
            .set::<String, String, String>(
                BLOG_WEB_CONFIG.to_owned(),
                serde_json::to_string(&config).unwrap(),
            )
            .is_ok();
    }

    // 设置用户令牌到 Redis
    pub fn set_token(&self, username: &String, token: &String) -> bool {
        return get_pool_connection()
            .set_ex::<String, String, String>(
                USER_TOKEN_KEY.to_owned() + username,
                token.to_owned(),
                (CONFIG.token.expire as usize) * DAY,
            )
            .is_ok();
    }

    // 从 Redis 删除用户令牌
    pub fn remove_user_token(&self, username: &String) -> bool {
        return get_pool_connection()
            .del::<String, String>(USER_TOKEN_KEY.to_owned() + username)
            .is_ok();
    }

    // 从 Redis 获取用户令牌
    pub fn get_token(&self, username: &String) -> Option<String> {
        return if let Ok(r) =
            get_pool_connection().get::<String, String>(USER_TOKEN_KEY.to_owned() + username)
        {
            Some(r)
        } else {
            None
        };
    }
}

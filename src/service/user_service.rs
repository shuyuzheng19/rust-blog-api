use std::sync::Arc;

use lettre::message::header::ContentType;
use log::{error, info};
use sqlx::{Pool, Postgres};

use crate::cache::user_cache::UserCache;
use crate::common::{get_random_code_number, is_valid_email};
use crate::conf::config::CONFIG;
use crate::error::custom_error::{E, Status};
use crate::models::user::UserVo;
use crate::repository::user_repository::UserRepository;
use crate::request::email_request::EmailRequest;
use crate::request::user_request::{ContactRequest, UserRegisteredRequest, UserRequest};
use crate::response::website_info::{BlogConfigInfo, get_default_blog_config_info};

pub struct UserService(Arc<UserRepository>, UserCache, String);

impl UserService {
    pub fn new(db_conn: Pool<Postgres>) -> UserService {
        let user_cache = UserCache::new();
        let user_repository = UserRepository::new(db_conn);
        UserService(
            Arc::new(user_repository),
            user_cache,
            CONFIG.gpt.token.to_owned(),
        )
    }

    // 获取用户信息，先从缓存获取，缓存中不存在再从数据库获取
    pub async fn get_user(&self, username: &String) -> Option<UserVo> {
        let option = self.1.get_user(username);
        return if option.is_err() {
            // 记录错误日志信息
            let user = self.0.get_user_by_username(username).await;
            return if user.is_err() {
                None
            } else {
                let optional = Some(user.unwrap());
                // 记录成功日志信息
                self.1.set_user(username, &optional);
                optional
            };
        } else {
            option.unwrap()
        };
    }

    // 用户登录
    pub async fn login_user(&self, user_request: &UserRequest) -> Result<String, E> {
        let check = user_request.check();
        if !check {
            return Err(E::error(
                Status::CHECK_DATA_ERROR,
                String::from("账号或密码有空"),
            ));
        }

        let user = self.get_user(&user_request.username).await;

        if user.is_none() {
            return Err(E::error(
                Status::USER_NOT_FOUND_ERROR,
                String::from("该用户找不到"),
            ));
        }

        let user = user.unwrap();

        let check = user_request.check_user(&user).await;

        return if check {
            let token = CONFIG.token.create_token(user.id, &user.username).await;
            self.1.set_token(&user.username, &token);
            // 记录成功登录日志
            info!("用户登录成功：{}", user.username);
            Ok(token)
        } else {
            // 记录密码验证失败日志
            error!("用户登录失败：{}", user.username);
            Err(E::error(
                Status::PASSWORD_VALIDATE_ERROR,
                String::from("密码错误"),
            ))
        };
    }

    // 用户注册
    pub async fn registered_user(&self, user: &mut UserRegisteredRequest) -> Option<E> {
        let check_message = user.check();

        if check_message.is_some() {
            return Some(E::error(Status::CHECK_DATA_ERROR, check_message.unwrap()));
        }

        let redis_email_code = self.1.get_email_code(&user.email);

        if redis_email_code.is_err() || redis_email_code.unwrap() != user.code {
            return Some(E::error(Status::EMAIL_ERROR, String::from("错误的验证码")));
        }

        return match self.0.user_is_exists(&user.username, &user.email).await {
            None => {
                let md5_password = format!("{:x}", md5::compute(&user.password));
                user.password = md5_password;
                self.0.insert_user(user).await
            }
            Some(e) => Some(e),
        };
    }

    // 发送邮件验证码
    pub fn send_mail(&self, email: &String) -> Option<E> {
        let is_email = is_valid_email(email);

        if !is_email {
            return Some(E::error(
                Status::EMAIL_ERROR,
                String::from("错误的邮箱格式"),
            ));
        }

        let random_code = &get_random_code_number();

        let req = EmailRequest {
            to: email.to_owned(),
            subject: String::from("Yuice 验证码"),
            message: random_code.to_owned(),
        };

        return if CONFIG.smtp.send_email(req, ContentType::TEXT_PLAIN) {
            self.1.set_email_code(&email, &random_code);
            // 记录成功发送验证码日志
            info!("成功发送验证码至邮箱：{}", email);
            None
        } else {
            // 记录发送验证码失败日志
            error!("发送验证码至邮箱失败：{}", email);
            Some(E::error(
                Status::EMAIL_ERROR,
                String::from("发送验证码失败"),
            ))
        };
    }

    // 联系我
    pub fn contact_me(&self, req: &ContactRequest) -> Option<E> {
        if let Some(e) = req.check() {
            return Some(E::error(Status::CHECK_DATA_ERROR, e));
        }

        let html_content = format!(
            "<h3>{}</h3><p>对方名字: {}</p><p>对方邮箱: {}</p>留言内容:<p>{}</p>",
            req.subject, req.name, req.email, req.content
        );

        let email_request = EmailRequest {
            to: CONFIG.my_email.to_owned(),
            subject: req.subject.to_owned(),
            message: html_content,
        };

        let result = CONFIG
            .smtp
            .send_email(email_request, ContentType::TEXT_HTML);

        return if result {
            // 记录成功发送消息到邮箱的日志
            info!("成功发送消息至我的邮箱：{}", CONFIG.my_email);
            None
        } else {
            // 记录发送消息到邮箱失败的日志
            error!("发送消息至我的邮箱失败：{}", CONFIG.my_email);
            Some(E::error(
                Status::EMAIL_ERROR,
                String::from("发送消息到我的邮箱失败"),
            ))
        };
    }

    // 获取网站配置信息
    pub fn get_website_config(&self) -> BlogConfigInfo {
        return if let Some(e) = self.1.get_website_config() {
            e
        } else {
            get_default_blog_config_info()
        };
    }

    // 获取用户的Token
    pub fn get_user_token(&self, username: &String) -> Option<String> {
        return self.1.get_token(username);
    }

    // 用户注销
    pub async fn logout(&self, username: &String) -> bool {
        return self.1.remove_user_token(username);
    }
}

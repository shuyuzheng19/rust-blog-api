use serde::{Deserialize, Serialize};

use crate::common::{is_image_url, is_valid_email};
use crate::models::user::UserVo;

#[derive(Deserialize, Serialize)]
pub struct UserRequest {
    pub username: String,
    pub password: String,
}

impl UserRequest {
    pub fn check(&self) -> bool {
        !self.username.trim().is_empty() && !self.password.trim().is_empty()
    }
    pub async fn check_user(&self, user: &UserVo) -> bool {
        let md5_password = format!("{:x}", &md5::compute(&self.password));
        user.username == self.username && md5_password == user.password
    }
}

#[derive(Deserialize, Debug)]
pub struct UserRegisteredRequest {
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "nickName")]
    pub nick_name: String,
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "icon")]
    pub icon: String,
    #[serde(rename = "code")]
    pub code: String,
}

impl UserRegisteredRequest {
    pub fn check(&self) -> Option<String> {
        if self.username.len() < 8 || self.username.len() > 16 {
            return Some(String::from("账号要大于8个并且小于16个字符"));
        } else if self.nick_name.is_empty() {
            return Some(String::from("用户名称不能为空"));
        } else if self.password.len() < 8 || self.password.len() > 16 {
            return Some(String::from("密码要大于8个并且小于16个字符"));
        } else if self.email.is_empty() || !is_valid_email(&self.email) {
            return Some(String::from("不正确的邮箱格式"));
        } else if !is_image_url(&self.icon) {
            return Some(String::from("不正确的图片格式"));
        } else if self.code.is_empty() {
            return Some(String::from("验证码不能为空"));
        }
        return None;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactRequest {
    pub name: String,
    pub email: String,
    pub subject: String,
    pub content: String,
}

impl ContactRequest {
    pub fn check(&self) -> Option<String> {
        if self.email.is_empty() || !is_valid_email(&self.email) {
            return Some("错误的邮箱格式".to_string());
        } else if self.name.is_empty() {
            return Some("请输入你的名称".to_string());
        } else if self.subject.is_empty() {
            return Some("请输入主题内容".to_string());
        } else if self.content.is_empty() {
            return Some("请输入消息内容".to_string());
        }
        return None;
    }
}

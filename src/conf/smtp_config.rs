use lettre::{Message, SmtpTransport, Transport};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use serde::{Deserialize, Serialize};

use crate::request::email_request::EmailRequest;

// SMTP 配置结构
#[derive(Debug, Serialize, Deserialize)]
pub struct SmtpConfig {
    username: String, // SMTP 用户名
    password: String, // SMTP 密码
    host: String,     // SMTP 主机地址
    addr: String,     // 电子邮件地址（可能不需要）
}

impl SmtpConfig {
    // 发送电子邮件方法
    pub fn send_email(&self, req: EmailRequest, header: ContentType) -> bool {
        // 创建电子邮件消息
        let email = Message::builder()
            .from(self.username.parse().unwrap()) // 发件人
            .to(req.to.parse().unwrap()) // 收件人
            .subject(req.subject) // 主题
            .header(header) // 邮件头部
            .body(req.message) // 邮件正文
            .unwrap();

        // 设置 SMTP 凭据
        let creds = Credentials::new(self.username.to_owned(), self.password.to_owned());

        // 创建 SMTP 传输
        let mailer = SmtpTransport::relay(self.host.as_str())
            .unwrap()
            .credentials(creds)
            .build();

        // 发送邮件
        return match mailer.send(&email) {
            Ok(_) => true,   // 邮件发送成功
            Err(e) => false, // 邮件发送失败
        };
    }
}

use actix_web::{get, http, HttpRequest, HttpResponse, post, Responder};
use actix_web::http::header::ContentEncoding;
use actix_web::web::{Data, Json, Query};
use log::info;
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::common::{get_client_ip_city, get_ip_address};
use crate::common::result::R;
use crate::error::custom_error::{E, Status};
use crate::middleware::jwt::{JwtAdminRole, JwtUserRole};
use crate::request::user_request::{ContactRequest, UserRegisteredRequest, UserRequest};

#[derive(Deserialize, Debug)]
pub struct GetUserQuery {
    username: String,
}

#[get("/auth/get")]
pub async fn get_user_by_id(jwt: JwtUserRole) -> impl Responder {
    return R::success(jwt.user.to_vo()).response_to_json();
}

#[derive(Deserialize, Debug)]
pub struct SendEmailRequest {
    email: String,
}

#[get("/send_mail")]
pub async fn send_email_for_code(
    req: HttpRequest,
    email_req: Query<SendEmailRequest>,
    service: Data<AppState>,
) -> Result<HttpResponse, E> {
    let email = &email_req.into_inner().email;
    let result = service.user_service.send_mail(&email);
    return match result {
        Some(e) => Err(e),
        None => {
            info!("发送邮件成功 email:{}",email);
            Ok(R::success("发送邮件成功").response_to_json())
        }
    };
}

#[post("/contact_me")]
pub async fn contact_me(
    contact_request: Json<ContactRequest>,
    service: Data<AppState>,
) -> Result<HttpResponse, E> {
    let request = contact_request.into_inner();
    let result = service.user_service.contact_me(&request); // 传递引用

    match result {
        Some(e) => Err(e),
        None => {
            info!(
                "联系我 信息: email:{} subject:{} message:{}",
                request.email, request.subject, request.content
            );
            Ok(R::success("发送邮件成功").response_to_json())
        }
    }
}

#[get("/config")]
pub async fn get_web_site_info(service: Data<AppState>) -> impl Responder {
    return R::success(service.user_service.get_website_config()).response_to_json();
}

#[post("/registered")]
pub async fn registered_user(
    req: HttpRequest,
    user_req: Json<UserRegisteredRequest>,
    service: Data<AppState>,
) -> impl Responder {
    let mut user = user_req.into_inner();
    let result = service.user_service.registered_user(&mut user).await;
    return if result.is_none() {
        info!("注册用户成功 username:{}",user.username);
        Ok(R::success("注册用户成功").response_to_json())
    } else {
        Err(result.unwrap())
    };
}

#[post("/login")]
pub async fn login(
    user_request: Json<UserRequest>,
    service: Data<AppState>,
) -> Result<HttpResponse, E> {
    let user = &user_request.into_inner();
    let result = service.user_service.login_user(user).await;
    return if result.is_ok() {
        Ok(R::success(result.unwrap()).response_to_json())
    } else {
        Err(result.unwrap_err())
    };
}

#[get("/logout")]
pub async fn logout(jwt: JwtUserRole, service: Data<AppState>) -> impl Responder {
    let result = service.user_service.logout(&jwt.user.username).await;
    R::success(result).response_to_json()
}

#[derive(Deserialize, Debug)]
pub struct ChatQuery {
    message: String,
}
#[derive(Deserialize, Debug)]
pub struct ChinaIpQuery{
    pub callback:String
}

#[get("/is_cn")]
pub async fn is_cn(
    req: HttpRequest,
    _:Query<ChinaIpQuery>
) -> impl Responder {
    let ip = get_ip_address(&req);
    let city = get_client_ip_city(&ip);
    let is_cn = if city=="未知" || (city.contains("中国") && !city.contains("香港") && !city.contains("台湾")){true}else{false};
    HttpResponse::Ok()
        .content_type("application/javascript")
        .body(format!("ipCallback({})", is_cn))
}

#[get("/chat")]
pub async fn chat_gpt(
    _: JwtAdminRole,
    req: Query<ChatQuery>,
    service: Data<AppState>,
) -> impl Responder {
    let message = &req.message;

    let result = service.chat_service.lock().unwrap().chat(message).await;

    info!("GPT 提问内容: {}",message);

    if let Err(e) = result {
        HttpResponse::Unauthorized().json(E::error(
            Status::AUTHORIZED_ERROR,
            String::from("GPT Token可能已过期"),
        ))
    } else {
        HttpResponse::Ok()
            .insert_header((http::header::CONTENT_TYPE, "text/event-stream"))
            .insert_header(ContentEncoding::Identity)
            .streaming(result.unwrap().bytes_stream())
    }
}

#[derive(Serialize)]
struct Event {
    message: String,
}

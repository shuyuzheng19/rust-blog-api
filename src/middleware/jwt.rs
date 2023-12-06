use std::future::Future;
use std::pin::Pin;

use actix_web::{
    dev::{Service, Transform},
    FromRequest, HttpMessage, HttpRequest,
};
use actix_web::dev::Payload;
use actix_web::http::header::HeaderValue;
use actix_web::web::Data;
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::conf::config::CONFIG;
use crate::error::custom_error::{E, Status};
use crate::models::user::UserVo;
use crate::service::user_service::UserService;

//普通用户jwt验证
#[derive(Deserialize, Debug, Serialize)]
pub struct JwtUserRole {
    pub user: UserVo,
}

//管理员用户jwt验证
#[derive(Deserialize, Debug, Serialize)]
pub struct JwtAdminRole {
    pub user: UserVo,
}

//超级管理员用户jwt验证
#[derive(Deserialize, Debug, Serialize)]
pub struct JwtSuperAdminRole {
    pub user: UserVo,
}

enum RoleType {
    USER,
    ADMIN,
    SuperAdmin,
}

fn get_user_and_verify_token(
    token_header: Option<&HeaderValue>,
    user_service: &UserService,
) -> Result<String, E> {
    if token_header.is_none() {
        return Err(E::error(
            Status::AUTHENTICATE_ERROR,
            String::from("你还未登录，禁止访问"),
        ));
    }

    let token_header = token_header.unwrap().to_str().unwrap();

    const TOKEN_PREFIX: &str = "Bearer ";

    if !token_header.starts_with(TOKEN_PREFIX) {
        return Err(E::error(
            Status::AUTHENTICATE_ERROR,
            String::from("无效的Token 请检查..."),
        ));
    }

    let token = token_header.replace(TOKEN_PREFIX, "");

    let token_option = CONFIG.token.parse_token(&token);

    if token_option.is_none() {
        return Err(E::error(
            Status::AUTHENTICATE_ERROR,
            String::from("可能Token已失效，请重新登录..."),
        ));
    }

    let username = token_option.unwrap().sub;

    let redis_token = user_service.get_user_token(&username);

    if redis_token.is_none() || redis_token.unwrap() != token {
        return Err(E::error(
            Status::AUTHENTICATE_ERROR,
            String::from("服务器已拒绝你的访问，请重新登录"),
        ));
    }

    return Ok(username);
}

impl FromRequest for JwtUserRole {
    type Error = E;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let token_header = req.headers().get(actix_web::http::header::AUTHORIZATION);

        let app_state = req.app_data::<Data<AppState>>().unwrap();

        let user_service = app_state.user_service.clone();

        let result = get_user_and_verify_token(token_header, &user_service);

        if result.is_err() {
            return Box::pin(async { Err(result.unwrap_err()) });
        }

        let username = result.unwrap();

        return Box::pin(async move {
            let user = user_service.get_user(&username).await;
            if let Some(u) = user {
                return Ok(JwtUserRole { user: u });
            }
            return Err(E::error(
                Status::AUTHENTICATE_ERROR,
                String::from("该用户不存在"),
            ));
        });
    }
}

impl FromRequest for JwtAdminRole {
    type Error = E;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let token_header = req.headers().get(actix_web::http::header::AUTHORIZATION);

        let app_state = req.app_data::<Data<AppState>>().unwrap();

        let user_service = app_state.user_service.clone();

        let result = get_user_and_verify_token(token_header, &user_service);

        if result.is_err() {
            return Box::pin(async { Err(result.unwrap_err()) });
        }

        let username = result.unwrap();

        return Box::pin(async move {
            let user = user_service.get_user(&username).await;
            if let Some(u) = user {
                if u.role == "ADMIN" || u.role == "SUPER_ADMIN" {
                    return Ok(JwtAdminRole { user: u });
                } else {
                    return Err(E::error(
                        Status::AUTHENTICATE_ERROR,
                        String::from("你的权限不够，需要ADMIN角色"),
                    ));
                }
            }
            return Err(E::error(
                Status::AUTHENTICATE_ERROR,
                String::from("该用户不存在"),
            ));
        });
    }
}
//
impl FromRequest for JwtSuperAdminRole {
    type Error = E;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let token_header = req.headers().get(actix_web::http::header::AUTHORIZATION);

        let app_state = req.app_data::<Data<AppState>>().unwrap();

        let user_service = app_state.user_service.clone();

        let result = get_user_and_verify_token(token_header, &user_service);

        if result.is_err() {
            return Box::pin(async { Err(result.unwrap_err()) });
        }

        let username = result.unwrap();

        return Box::pin(async move {
            let user = user_service.get_user(&username).await;
            if let Some(u) = user {
                if u.role == "SUPER_ADMIN" {
                    return Ok(JwtSuperAdminRole { user: u });
                } else {
                    return Err(E::error(
                        Status::AUTHENTICATE_ERROR,
                        String::from("你的权限不够，需要SUPER_ADMIN角色"),
                    ));
                }
            }
            return Err(E::error(
                Status::AUTHENTICATE_ERROR,
                String::from("该用户不存在"),
            ));
        });
    }
}

//全局中间件 弃用
// pub struct JwtAuth;
// impl<S, B> Transform<S, ServiceRequest> for JwtAuth
//     where
//         S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//         S::Future: 'static,
//         B: 'static,
// {
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type Transform = JwtAuthMiddleware<S>;
//     type InitError =();
//     type Future = Ready<Result<Self::Transform, Self::InitError>>;
//
//     fn new_transform(&self, service: S) -> Self::Future {
//         ready(Ok(JwtAuthMiddleware { service }))
//     }
// }
//
// pub struct JwtAuthMiddleware<S> {
//     service: S,
// }
// type LocalBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;
//
// impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
//     where
//         S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//         S::Future: 'static,
//         B: 'static,
// {
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type Future = LocalBoxFuture<Result<Self::Response, Self::Error>>;
//
//     forward_ready!(service);
//
//     fn call(&self, mut req: ServiceRequest) -> Self::Future {
//
//         let path = req.path();
//
//         let user_type = match_path(path);
//
//         if user_type==""{
//             let res = self.service.call(req);
//
//             return Box::pin(async { Ok(res.await?) })
//         }
//
//         let token_header = req.headers().get(actix_web::http::header::AUTHORIZATION);
//
//         if token_header.is_none(){
//             return Box::pin(async {  Err(Error::from(E::error(Status::AUTHENTICATE_ERROR, String::from("你还未登录，禁止访问")))) })
//         }
//
//         let token_header = token_header.unwrap().to_str().unwrap();
//
//         const TOKEN_PREFIX:&str="Bearer ";
//
//         if !token_header.starts_with(TOKEN_PREFIX){
//             return Box::pin(async {  Err(Error::from(E::error(Status::INVALID_TOKEN_ERROR, String::from("无效的Token 请检查...")))) })
//         }
//
//         let token = token_header.replace(TOKEN_PREFIX,"");
//
//         let token_option = CONFIG.token.parse_token(token);
//
//         if token_option.is_none(){
//             return Box::pin(async {  Err(Error::from(E::error(Status::INVALID_TOKEN_ERROR, String::from("可能Token已失效，请重新登录...")))) })
//         }
//
//         let token = token_option.unwrap();
//
//         req.extensions_mut().insert::<GetUserInfo>(GetUserInfo(token.user.clone()));
//
//         let user = token.user;
//
//         let r = async move{
//
//             let role = &user.role;
//
//             let mut is_auth = false;
//
//             if role=="SUPER_ADMIN" {
//                 is_auth=true;
//             }else if role=="ADMIN" && user_type=="ADMIN"{
//                 is_auth=true;
//             }
//
//             return if is_auth {
//                 None
//             } else {
//                 Some(Error::from(E::error(Status::AUTHORIZED_ERROR, String::from("你当前的角色不允许访问当前接口"))))
//             }
//
//         };
//
//         let res = self.service.call(req);
//
//         return Box::pin(async move{
//             let rs = r.await;
//             return if rs.is_none() {
//                 Ok(res.await?)
//             } else {
//                 Err(rs.unwrap())
//             }
//         });
//     }
// }
//
// fn match_path(url:&str)->String{
//     let parts: Vec<&str> = url.split('/').collect();
//
//     if url.starts_with("/api/v1/") && parts.len()>4{
//         match parts[4] {
//             "auth" => String::from("USER"),
//             "admin" => String::from("ADMIN"),
//             "super" => String::from("SUPER_ADMIN"),
//             _ => String::new()
//         }
//     } else {
//         String::new()
//     }
// }

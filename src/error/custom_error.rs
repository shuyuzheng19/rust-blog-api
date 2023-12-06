use std::fmt::Display;
use std::hash::Hash;

use actix_web::{dev, HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use actix_web::dev::ServiceResponse;
use actix_web::http::header;
use actix_web::middleware::ErrorHandlerResponse;
use log::error;
use reqwest::header::HeaderValue;
use reqwest::StatusCode;
use serde::Serialize;
use serde_json::json;

// 自定义错误结构
#[derive(Debug, Serialize)]
pub struct E {
    pub code: Code,
    pub message: String,
}

type Code = i32;

// 自定义状态码
pub struct Status {}

impl Status {
    // 成功响应，状态码为 200
    pub const OK: Code = 200;

    // 通用的失败响应，状态码为 10000
    pub const FAIL: Code = 10000;

    // 身份验证失败，状态码为 401
    pub const AUTHENTICATE_ERROR: Code = 401;

    // 授权错误，状态码为 403
    pub const AUTHORIZED_ERROR: Code = 403;

    // 服务器内部错误，状态码为 500
    pub const SERVER_ERROR: Code = 500;

    // 数据验证失败，状态码为 10001
    pub const CHECK_DATA_ERROR: Code = 10001;

    // 用户未找到，状态码为 10002
    pub const USER_NOT_FOUND_ERROR: Code = 10002;

    // 密码验证失败，状态码为 10003
    pub const PASSWORD_VALIDATE_ERROR: Code = 10003;

    // 无效令牌错误，状态码为 10004
    pub const INVALID_TOKEN_ERROR: Code = 10004;

    // 数据库错误，状态码为 10005
    pub const DATABASE_ERROR: Code = 10005;

    // 数据为空错误，状态码为 10006
    pub const DATA_EMPTY_ERROR: Code = 10006;

    // 获取热门博客错误，状态码为 10007
    pub const GET_HOT_BLOG_ERROR: Code = 10007;

    // 查询或参数错误，状态码为 10008
    pub const QUERY_OR_PARAMS_ERROR: Code = 10008;

    // 博客未找到，状态码为 10009
    pub const BLOG_NOT_FOUND_ERROR: Code = 10009;

    // HTTP 请求错误，状态码为 10010
    pub const HTTP_REQUEST_ERROR: Code = 10010;

    // 邮件错误，状态码为 10011
    pub const EMAIL_ERROR: Code = 10011;

    // 编辑错误，状态码为 10012
    pub const EDIT_ERROR: Code = 10012;

    // 上传文件错误，状态码为 10013
    pub const UPLOAD_FILE_ERROR: Code = 10013;

    // 添加失败
    pub const ADD_ERROR: Code = 10014;

    // 删除失败
    pub const DELETE_ERROR: Code = 10014;
}

impl E {
    pub fn error(code: Code, message: String) -> E {
        return E { code, message };
    }

    pub fn default() -> E {
        return E {
            code: Status::FAIL,
            message: String::from("处理失败"),
        };
    }
}

// 实现 Display trait 以便格式化错误信息
impl Display for E {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = serde_json::to_string(&self).unwrap();
        write!(f, "{}", &result)
    }
}

// 实现 ResponseError trait 以处理错误响应
impl ResponseError for E {
    fn status_code(&self) -> StatusCode {
        return StatusCode::CREATED;
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        error!("全局自定义错误: {}", &self);
        return HttpResponse::build(self.status_code()).json(&self);
    }
}

// 数据验证失败错误处理函数
pub fn default_error<B>(
    mut res: dev::ServiceResponse<B>,
) -> actix_web::Result<ErrorHandlerResponse<B>> {
    let (req, res) = res.into_parts();

    let code = &res.status().as_u16();

    let error = match res.error(){
        Some(e)=>e.to_owned().to_string(),
        None=>"Server 未知错误".to_string()
    };

    // let error = json!({
    //     "code":&code,
    //     "path": req.path().to_owned(),
    //     "error_message": error,
    //     "ip": req.connection_info().clone().realip_remote_addr().unwrap().to_owned(),
    // });

    error!("HTTP ERROR 状态码: {} 错误信息: {}",code, error);

    let mut res = res.set_body(
        json!(
            {
                "code":code,
                "message":"后台不能正常处理请求...."
            }
        )
        .to_string(),
    );

    res.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str("application/json;charset=utf-8").unwrap(),
    );

    let res = ServiceResponse::new(req, res)
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(res))
}

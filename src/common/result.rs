use std::fmt::Display;

use actix_web::HttpResponse;
use serde::Serialize;

use crate::error::custom_error::Status;

#[derive(Serialize)]
pub struct R<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> R<T>
where
    T: Serialize,
{
    pub fn ok() -> R<T> {
        return R {
            code: Status::OK,
            message: String::from("成功"),
            data: None,
        };
    }
    pub fn success(data: T) -> R<T> {
        return R {
            code: Status::OK,
            message: String::from("成功"),
            data: Option::from(data),
        };
    }
    pub fn response_to_json(&self) -> HttpResponse {
        return HttpResponse::Ok().json(self);
    }
}

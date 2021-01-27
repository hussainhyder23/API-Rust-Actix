// use actix_web::{error::ResponseError, HttpResponse};
// use derive_more::Display;

// #[derive(Debug, Display)]
// pub enum ServiceError {
//     #[display(fmt = "Internal Server Error")]
//     InternalServerError,

//     #[display(fmt = "BadRequest: {}", _0)]
//     BadRequest(String),

//     #[display(fmt = "JWKSFetchError")]
//     JWKSFetchError,
// }

// // impl ResponseError trait allows to convert our errors into http responses with appropriate data
// impl ResponseError for ServiceError {
//     fn error_response(&self) -> HttpResponse {
//         match self {
//             ServiceError::InternalServerError => {
//                 HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
//             }
//             ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
//             ServiceError::JWKSFetchError => {
//                 HttpResponse::InternalServerError().json("Could not fetch JWKS")
//             }
//         }
//     }
// }


//Starting here

use actix_web::{get, web, App, HttpResponse, HttpServer, error::ResponseError, http::StatusCode};
use std::io::Read;

use serde::{Serialize};
#[macro_use]
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("Requested file was not found")]
    NotFound,
    #[error("You are forbidden to access requested file.")]
    Forbidden,
    #[error("Unknown Internal Error")]
    Unknown
}
impl CustomError {
    pub fn name(&self) -> String {
        match self {
            Self::NotFound => "NotFound".to_string(),
            Self::Forbidden => "Forbidden".to_string(),
            Self::Unknown => "Unknown".to_string(),
        }
    }
}
impl ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::NotFound  => StatusCode::NOT_FOUND,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
            error: self.name(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}

pub fn map_io_error(e: std::io::Error) -> CustomError {
    match e.kind() {
        std::io::ErrorKind::NotFound => CustomError::NotFound,
        std::io::ErrorKind::PermissionDenied => CustomError::Forbidden,
        _ => CustomError::Unknown,
    }
}
#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}
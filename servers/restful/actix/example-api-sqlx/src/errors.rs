use sqlx::error::Error as SqlxErr;
use actix_web::{error::Error as ActixErr, ResponseError, HttpResponse, http::StatusCode};
use serde;
use std::error::Error;
use std::fmt;


#[derive(serde::Serialize)]
struct AppErrorJson {
    err_msg: String,
}

/// Custom error type for actix web application that wraps all errors from various libs.
/// 
/// Using error type allows unify error handling.
/// 
/// By convention error types in rust implement the `std::error::Error` marker trait.
/// The `std::error::Error trait` in turn, requires to implement `Debug` and `Display` traits.
/// 
/// The `Debug` trait is auto-derived.
/// 
/// Any error type that implements `actix_web::ResponseError` trait can be converted into an HTTP response message by Actix Web.
/// 
/// Actix Web contains default implementation of the `actix_web::ResponseError` trait for many common errors type, such as
/// - `std::io::Error`
/// - `serde::ser::Error`
/// - `serde::de::Error`
#[derive(Debug)]
pub enum AppError {
    DB(String),
    IO(String),
    Serialize(String),
    Actix(String),
    UnprocessableInput(String)
}


impl Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DB(e) => write!(f, "DB error"),
            AppError::IO(e) => write!(f, "IO error"),
            AppError::Serialize(e) => write!(f, "Serialize/Deserialize error"),
            AppError::Actix(e) => write!(f, "Actix"),
            AppError::UnprocessableInput(e) => write!(f, "Unprocessable input"),
        }
    }
}

impl AppError {
    /// This method is used to serialize `AppError` to message for user, i.e., to `AppErrorJson` type.
    fn err_msg(&self) -> String {
        self.to_string()
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::DB(e) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::IO(e) => StatusCode::NOT_FOUND,
            AppError::Serialize(e) => StatusCode::BAD_REQUEST,
            AppError::Actix(e) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::UnprocessableInput(e) => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorJson {
            err_msg: self.err_msg(),
        })
    }
}

/// This enables actix web errors to be converted to AppError using `?` operator.
impl From<ActixErr> for AppError {
    fn from(e: ActixErr) -> Self {
        AppError::Actix(e.to_string())
    }
}

/// This enables sqlx errors to be converted to AppError using `?` operator.
impl From<SqlxErr> for AppError {
    fn from(e: SqlxErr) -> Self {
        AppError::DB(e.to_string())
    }
}
use std::fmt::{Display, Formatter};
use actix_web::{error, Error, HttpResponse};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use serde::Serialize;
use sqlx::Error as SQLxError;

#[derive(Debug)]
pub enum MyError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    InvalidInput(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_msg: String,
}

impl MyError {
    fn error_response(&self) -> String {
        match self {
            MyError::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            },
            MyError::ActixError(msg) => {
                println!("Actix error occurred: {:?}", msg);
                "Internal server error".into()
            },
            MyError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            },
            MyError::InvalidInput(msg) => {
                println!("Invalid input error occurred: {:?}", msg);
                msg.into()
            },
        }
    }
}

impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            MyError::DBError(_) | MyError::ActixError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::NotFound(_) => StatusCode::NOT_FOUND,
            MyError::InvalidInput(_) => StatusCode::BAD_REQUEST,
        }
    }

    //rewrite error_response
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(MyErrorResponse{
            error_msg: self.error_response()
        })
    }
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<Error> for MyError {
    fn from(error: Error) -> Self {
        MyError::ActixError(error.to_string())
    }
}

impl From<SQLxError> for MyError {
    fn from(error: SQLxError) -> Self {
        MyError::DBError(error.to_string())
    }
}
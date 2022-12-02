use std::fmt;
use actix_web::{HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use diesel::result::Error as DieselError;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct CustomError {
    pub status_code: u16,
    pub message: String,
}

impl CustomError {
    pub fn new(status_code: u16, message: String) -> Self {
        CustomError { status_code, message }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CustomError: {} {}", self.status_code, self.message)
    }
}

impl From<DieselError> for CustomError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::DatabaseError(_, err) => CustomError::new(500, format!("Database Error: {}", err.message().to_string())),
            DieselError::NotFound => CustomError::new(404, "Resource Not found".to_string()),
            _ => CustomError::new(500, "Internal Server Error".to_string()),
        }
    }
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let status_code = match StatusCode::from_u16(self.status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let error_message = match status_code.as_u16() < 500 {
            true => self.message.clone(),
            false => "Internal Server Error".to_string(),
        };
        HttpResponse::build(status_code).json(json!({ "message": error_message }))
    }
}
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use serde_json::json;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("MongoDB error: {0}")]
    MongoDB(#[from] mongodb::error::Error),
    #[error("Invalid ID format")]
    InvalidId(#[from] bson::oid::Error),
    #[error("Not found")]
    NotFound,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::MongoDB(_) => {
                HttpResponse::InternalServerError().json(json!({ "error": self.to_string() }))
            }
            ApiError::InvalidId(_) => {
                HttpResponse::BadRequest().json(json!({ "error": "Invalid ID format" }))
            }
            ApiError::NotFound => {
                HttpResponse::NotFound().json(json!({ "error": "Student not found" }))
            }
        }
    }
}

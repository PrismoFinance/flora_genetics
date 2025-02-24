use actix_web::HttpResponse;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("NCBI API request failed: {0}")]
    NcbiApiError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Internal server error")]
    InternalError,
}

impl actix_web::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::NcbiApiError(msg) => HttpResponse::BadRequest().body(msg.to_string()),
            ApiError::InvalidInput(msg) => HttpResponse::BadRequest().body(msg.to_string()),
            ApiError::RateLimitExceeded => HttpResponse::TooManyRequests().body("Rate limit exceeded"),
            ApiError::InternalError => HttpResponse::InternalServerError().body("Internal server error"),
        }
    }
}
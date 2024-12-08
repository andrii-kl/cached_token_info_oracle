use hex::FromHexError;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Invalid HMAC key length: {0}")]
    InvalidKeyLength(#[from] hmac::digest::InvalidLength),

    #[error("Failed to decoding a hex string in to Vec<u8>: {0}")]
    FromHexError(#[from] FromHexError),

    #[error("Config data is missing")]
    MissingConfig(),

    #[error("Access token is invalid or missing")]
    InvalidToken(),

    #[error("Token has been used before")]
    TokenAlreadyUsed(),

}

impl From<AppError> for (Status, Json<ErrorResponse>) {
    fn from(error: AppError) -> Self {
        let (message, code) = match error {
            AppError::InvalidKeyLength(_) => (error.to_string(), 400),
            AppError::FromHexError(_) => (error.to_string(), 400),
            AppError::MissingConfig() => (error.to_string(), 400),
            AppError::InvalidToken() => (error.to_string(), 400),
            AppError::TokenAlreadyUsed() => (error.to_string(), 400),
        };

        let error_response = ErrorResponse { message, code };
        (
            Status::from_code(code).unwrap_or(Status::InternalServerError),
            Json(error_response),
        )
    }
}


#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
    pub code: u16,
}

impl ErrorResponse {
    pub fn new(message: String, code: u16) -> Self {
        ErrorResponse { message, code }
    }
}

pub fn build_response(error: ErrorResponse) -> (Status, Json<ErrorResponse>) {
    let status = Status::from_code(error.code).unwrap_or(Status::InternalServerError);
    (status, Json(error))
}

pub fn bad_request(message: &str) -> (Status, Json<ErrorResponse>) {
    build_response(ErrorResponse::new(String::from(message), 400))
}
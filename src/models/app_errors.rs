use hex::FromHexError;
use rocket::serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Invalid HMAC key length: {0}")]
    InvalidKeyLength(#[from] hmac::digest::InvalidLength),

    #[error("Failed to decoding a hex string in to Vec<u8>: {0}")]
    FromHexError(#[from] FromHexError)

}


#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
    pub code: u16,
}
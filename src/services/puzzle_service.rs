use crate::models::app_errors::AppError;
use crate::models::core_token_models::PuzzleTask;
use hex::{decode, encode};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use uuid::Uuid;


pub async fn create_puzzle_task(pk:&[u8]) -> Result<PuzzleTask, AppError> {
    let my_uuid = Uuid::new_v4();
    let signature = sign_message_hmac_hex(pk, my_uuid.as_bytes());

    Ok(PuzzleTask::new(my_uuid.to_string(), signature?))
}

type HmacSha256 = Hmac<Sha256>;

pub fn sign_message_hmac(key: &[u8], message: &[u8]) -> Result<Vec<u8>, AppError> {
    let mut mac = HmacSha256::new_from_slice(key)?;
    mac.update(message);

    Ok(mac.finalize().into_bytes().to_vec())
}

pub fn sign_message_hmac_hex(key: &[u8], message: &[u8]) -> Result<String, AppError> {
    let signature = sign_message_hmac(key, message)?;

    Ok(encode(&signature))
}

pub fn verify_signature(key: &[u8], message: &[u8], signature: &[u8]) -> Result<bool, AppError> {
    let mut mac = HmacSha256::new_from_slice(key)?;

    mac.update(message);

    Ok(mac.verify_slice(signature).is_ok())
}

pub fn verify_signature_hex(key: &[u8], message: &[u8], signature: & str) -> Result<bool, AppError> {
    let mut mac = HmacSha256::new_from_slice(key)?;

    mac.update(message);

    Ok(mac.verify_slice(decode(&signature)?.as_slice()).is_ok())
}
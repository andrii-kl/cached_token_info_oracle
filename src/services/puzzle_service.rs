use crate::models::app_errors::AppError;
use crate::models::core_token_models::{AccessToken, PuzzleTask};
use hex::{decode, encode};
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use uuid::Uuid;


pub fn issue_access_token(pk:&[u8]) -> Result<AccessToken, AppError> {
    let my_uuid = Uuid::new_v4();
    let token = format!("{}{}", "access_", my_uuid.to_string());
    let signature = sign_message_hmac_hex(pk, token.to_string().as_bytes());

    Ok(AccessToken::new(token.to_string(), signature?))
}

pub fn create_puzzle_task(pk:&[u8], difficulty: u8) -> Result<PuzzleTask, AppError> {
    let my_uuid = Uuid::new_v4();
    let signature = sign_message_hmac_hex(pk, my_uuid.to_string().as_bytes());

    Ok(PuzzleTask::new(my_uuid.to_string(), signature?, difficulty))
}

pub fn verify_nonce(message: &str, nonce: &u64, difficulty: u8) -> bool {
    let prefix = "0".repeat(difficulty as usize);
    let data = format!("{}{}", message, nonce);
    let hash = Sha256::digest(data.as_bytes());
    let hash_hex = encode(hash);

    hash_hex.starts_with(&prefix)
}

pub fn find_nonce(message: &str, difficulty: u8) -> u64 {
    let prefix = "0".repeat(difficulty as usize);
    let mut nonce = 0;

    loop {
        let data = format!("{}{}", message, nonce);
        let hash = Sha256::digest(data.as_bytes());
        let hash_hex = encode(hash);

        if hash_hex.starts_with(&prefix) {
            return nonce;
        }

        nonce += 1;
    }
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

pub fn verify_access_token_hex(key: &[u8], message: &[u8], signature: &str) -> Result<bool, AppError> {
    if !message.starts_with("access_".as_bytes()) {
        Ok(false)
    } else {
        verify_signature_hex(key, message, signature)
    }
}

pub fn verify_signature_hex(key: &[u8], message: &[u8], signature: &str) -> Result<bool, AppError> {
    let mut mac = HmacSha256::new_from_slice(key)?;

    mac.update(message);

    Ok(mac.verify_slice(decode(&signature)?.as_slice()).is_ok())
}
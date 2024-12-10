use crate::api::pow_api::{get_puzzle_task, verify_puzzles};
use crate::api::token_oracle_api::get_token_prices;
use crate::models::api_models::{PuzzleSolution, TokenInfo};
use hex::encode;
use sha2::{Digest, Sha256};

pub async fn get_token_price() -> anyhow::Result<Vec<TokenInfo>> {
    let puzzle_task = get_puzzle_task().await?;

    println!("Looking nonce for Task: {}", &puzzle_task.task);
    
    let nonce = find_nonce(&puzzle_task.task, puzzle_task.difficulty);
    
    println!("Task: {} nonce: {}", &puzzle_task.task, nonce);

    let access_token = verify_puzzles(PuzzleSolution{
        task:puzzle_task.task, 
        nonce: nonce,
        signature: puzzle_task.signature,
    }).await?;

    let token_info: Vec<TokenInfo> = get_token_prices(access_token).await?;
    
    Ok(token_info)
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
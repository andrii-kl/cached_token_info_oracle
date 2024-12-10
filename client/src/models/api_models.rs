use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenInfo {
    pub token_ticker: String,
    pub price_list: Vec<TokenPrice>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TokenPrice {
    pub currency: String,
    pub price: String
}

#[derive(Deserialize, Serialize,  Debug)]
pub struct PuzzleSolution {
    pub task: String,
    pub nonce: u64,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PuzzleTask {
    pub task: String,
    pub signature: String,
    pub difficulty: u8
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AccessToken {
    pub access_token: String,
    pub signature: String,
}
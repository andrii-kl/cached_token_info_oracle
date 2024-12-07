use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TokenInfo {
    pub token_ticker: String,
    pub price_list: Vec<TokenPrice>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TokenPrice {
    pub currency: String,
    pub price: String
}

#[derive(Serialize, Debug, FromForm)]
pub struct PuzzleTask {
    task: String,
    signature: String,
}

impl PuzzleTask {
    pub fn new(task: String, signature: String) -> Self {
        PuzzleTask { task, signature }
    }
}
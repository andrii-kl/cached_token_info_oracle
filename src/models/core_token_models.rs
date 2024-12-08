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

#[derive(Deserialize, Serialize,  Debug, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct PuzzleSolution {
    pub task: String,
    pub nonce: u64,
    pub signature: String,
}

#[derive(Deserialize, Serialize, Debug, FromForm)]
pub struct AccessToken {
    pub access_token: String,
    pub signature: String,
}

impl AccessToken {
    pub fn new(access_token: String, signature: String) -> Self {
        AccessToken { access_token, signature }
    }

    pub fn default() -> AccessToken {
        Self::new(
            String::from("000000000-0000-0000-0000-000000000000"),
            String::from("00000")
        )
    }
}


impl PuzzleTask {
    pub fn new(task: String, signature: String) -> Self {
        PuzzleTask { task, signature }
    }
    pub fn get_task(&self) -> &String {
        &self.task
    }
    pub fn get_signature(&self) -> &String {
        &self.signature
    }
}

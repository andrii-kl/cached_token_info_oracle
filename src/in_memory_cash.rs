use crate::TokenInfo;
use lazy_static::lazy_static;
use rocket::yansi::Paint;
use std::collections::HashMap;
use tokio::sync::RwLock;

lazy_static! {
    // TokenTicker | TokenInfo
    pub static ref CASH: RwLock<HashMap<String, TokenInfo>> = RwLock::new(HashMap::new());
}


pub async fn add(value: TokenInfo) {
    let mut write_guard = CASH.write().await;
    write_guard.insert(value.token_ticker.to_string(), value);
}

pub async fn add_all(values: Vec<TokenInfo>) {
    let mut write_guard = CASH.write().await;
    for value in values {
        write_guard.insert(value.token_ticker.to_string(), value);
    }
}

// TODO
// pub async fn get_token(key: &Currency) -> TokenInfo {
// }

pub async fn get_all_tokens() -> Vec<TokenInfo> {
    let read_guard = CASH.read().await;
    read_guard.values().into_iter().cloned().collect()
}

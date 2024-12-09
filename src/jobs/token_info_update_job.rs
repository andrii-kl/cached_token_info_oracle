use reqwest;
use tokio;

use crate::api::coingecko_api;
use tokio::time::{interval, Duration};
use super::super::db::in_memory_token_info;

async fn fetch_token_info_data() {
    match coingecko_api::fetch_token_info_data().await {
        Ok(latest_token_info) => {
            in_memory_token_info::add_all(latest_token_info).await;
        }
        Err(e) => {
            println!("Error fetching data: {}", e);
        }
    }
    // TODO store data in to DB
}

pub async fn run(update_interval: u64) {
    println!("Fetch token information Job started");

    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(update_interval));
        loop {
            fetch_token_info_data().await;

            interval.tick().await;
        }
    });
}
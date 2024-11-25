use reqwest;
use tokio;

use crate::api::coingecko_api;
use tokio::time::{interval, Duration};

async fn fetch_token_info_data() {
    match coingecko_api::fetch_token_info_data().await {
        Ok(latest_token_info) => {
            super::super::in_memory_cash::add_all(latest_token_info).await;
        }
        Err(e) => {
            println!("Error fetching data: {}", e);
        }
    }
    // TODO store data in to DB
}

pub async fn run() {
    //TODO Get interval from the config file
    println!("Fetch token information Job started");

    tokio::spawn(async {
        let mut interval = interval(Duration::from_secs(15));
        loop {
            fetch_token_info_data().await;

            interval.tick().await;
        }
    });
}
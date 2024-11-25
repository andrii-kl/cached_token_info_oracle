#[macro_use] extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use lazy_static::lazy_static;
use rocket::{Build, Rocket};
use rocket::yansi::Paint;
use tokio::sync::{Mutex, RwLock};
use crate::in_memory_cash::CASH;

mod in_memory_cash;
mod api;
mod jobs;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TokenInfo {
    token_ticker: String,
    price_list: Vec<TokenPrice>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TokenPrice {
    pub currency: String,
    pub price: String
}

// Try visiting:
// http://127.0.0.1:8000/prices
#[get("/prices")]
async fn prices() -> String {
    let token_prices = in_memory_cash::get_all_tokens().await;
    serde_json::to_string(&token_prices).unwrap()
}

#[get("/")]
async fn home() -> String {
    //TODO add home page
    prices().await
}

fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![prices, home])
}

#[rocket::main]
async fn main() {
    jobs::token_info_update_job::run().await;

    // Recall that an uninspected `Error` will cause a pretty-printed panic,
    // so rest assured errors do not go undetected when using `#[launch]`.
    let _ = rocket().launch().await;
}
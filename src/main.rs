#[macro_use] extern crate rocket;

use rocket::serde::{Deserialize, Serialize};
use rocket::yansi::Paint;

mod in_memory_cash;
mod api;
mod jobs;
mod controllers;
mod models;
mod config;

#[rocket::main]
async fn main() {
    let conf = config::get_config();

    jobs::token_info_update_job::run(conf.coingecko_config().token_update_period_sec().clone()).await;

    let _ = controllers::token_info_oracle_controller::rocket().launch().await;
}
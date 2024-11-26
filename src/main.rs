#[macro_use] extern crate rocket;

use rocket::serde::{Deserialize, Serialize};
use rocket::yansi::Paint;

mod in_memory_cash;
mod api;
mod jobs;
mod controllers;
mod models;


#[rocket::main]
async fn main() {
    jobs::token_info_update_job::run().await;

    let _ = controllers::token_info_oracle_controller::rocket().launch().await;
}
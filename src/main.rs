#[macro_use] extern crate rocket;

use rocket::serde::{Deserialize, Serialize};
use rocket::yansi::Paint;
use crate::controllers::{controller_config, token_info_oracle_controller};
use crate::jobs::token_info_update_job;

mod in_memory_cash;
mod api;
mod jobs;
mod controllers;
mod models;
mod config;
mod services;

#[rocket::main]
async fn main() {
    let conf = config::get_config();

    let job_task = tokio::spawn(async move {
        token_info_update_job::run(conf.coingecko_config().token_update_period_sec().clone()).await;
    });

    let controller_task = tokio::spawn(async move {
        let _ = controller_config::rocket(conf.ddos_protection().clone(), conf.puzzle_signer_pk().clone()).launch().await;
    });


    // Await all tasks concurrently
    let _ = tokio::try_join!(job_task, controller_task);
}
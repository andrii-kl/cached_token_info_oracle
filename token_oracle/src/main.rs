#[macro_use] extern crate rocket;

use crate::controllers::controller_config;
use crate::jobs::token_info_update_job;
use rocket::serde::{Deserialize, Serialize};
use rocket::yansi::Paint;

mod api;
mod jobs;
mod controllers;
mod models;
mod config;
mod services;
mod db;

#[rocket::main]
async fn main() {
    let conf = config::get_config();

    let job_task = tokio::spawn(async move {
        token_info_update_job::run(conf.coingecko_config.token_update_period_sec).await;
    });

    let controller_task = tokio::spawn(async move {
        let _ = controller_config::rocket(conf.clone()).launch().await;
    });

    // Await all tasks concurrently
    let _ = tokio::try_join!(job_task, controller_task);
}
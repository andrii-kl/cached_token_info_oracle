use rocket::{Build, Rocket};
use crate::config::AppConfig;
use crate::in_memory_cash;
use rocket::State;
use crate::models::core_token_models::AccessToken;

#[derive(FromForm)]
struct PuzzleQuery {
    puzzle_id: String,
    puzzle_result: String,
}


#[get("/prices")]
async fn prices(_access_token: AccessToken) -> String {
    let token_prices = in_memory_cash::get_all_tokens().await;
    serde_json::to_string(&token_prices).unwrap()
}

#[get("/")]
async fn home() -> String {
    "Welcome to the Token Information Oracle application".to_string()
}

pub fn routes() -> Vec<rocket::Route> {
    routes![prices, home]
}

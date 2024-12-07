use rocket::{Build, Rocket};
use crate::config::AppConfig;
use crate::in_memory_cash;
use rocket::State;

#[derive(FromForm)]
struct PuzzleQuery {
    puzzle_id: String,
    puzzle_result: String,
}


#[get("/prices?<query..>")]
async fn prices(query: Option<PuzzleQuery>, ddos_protection: &State<bool>) -> String {
    if **ddos_protection {
        "DDOS Protection is enabled.".to_string();
    } else {
        "DDOS Protection is disabled.".to_string();
    }

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

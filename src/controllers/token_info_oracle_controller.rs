use rocket::{Build, Rocket};
use crate::in_memory_cash;

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

pub fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![prices, home])
}
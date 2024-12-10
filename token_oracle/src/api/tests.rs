use crate::api::coingecko_api;
use serde_json::Deserializer;
use std::collections::HashMap;

#[test]
fn coin_gecko_custom_deserialize_test() {
    let json_data = r#"
    {
        "bitcoin": {
            "usd": 96000,
            "eur": 92148
        },
        "ethereum": {
            "usd": 3312.2,
            "eur": 3179.31
        }
    }
    "#;

    let mut deserializer = Deserializer::from_str(json_data);
    let token_prices: HashMap<String, HashMap<String, String>> = coingecko_api::deserialize_nested_hashmap(&mut deserializer).unwrap();

    println!("Parsed data: {:?}", token_prices);

    assert_eq!(token_prices.len(), 2);
}
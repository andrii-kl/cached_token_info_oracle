use reqwest::header::{ACCEPT, CONTENT_TYPE};
use rocket::serde::Deserialize;
use serde_json::{Deserializer, Value};
use std::collections::HashMap;
use std::time::Duration;
use reqwest::{Error, StatusCode};
use tokio::time::sleep;
use crate::models::core_token_models::{TokenInfo, TokenPrice};

/**
/ Request examples:
/ https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum&vs_currencies=usd
/ https://pro-api.coingecko.com/api/v3/coins/list
**/
pub async  fn fetch_token_info_data() -> Result<Vec<TokenInfo>, Error> {
    let coingecko_config = crate::config::get_config().coingecko_config();

    let mut attempts_counter = 0;
    let retries: u16 = coingecko_config.number_attempts().clone();

    let url = format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}",
                      coingecko_config.token_ids(),
                      coingecko_config.token_currencies());

    loop {
        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .send()
            .await;


        match response {
            Ok(resp) if resp.status().is_success() => {
                let api_response = resp.text().await?;
                // Deserialize the JSON string into a HashMap
                let mut deserializer = Deserializer::from_str(&api_response);
                let token_prices_map: HashMap<String, HashMap<String, String>> = deserialize_nested_hashmap(&mut deserializer)
                    .unwrap_or(HashMap::new());

                return Ok(convert_map_to_object(token_prices_map))
            },
            Ok(res) if res.status() == StatusCode::INTERNAL_SERVER_ERROR && attempts_counter < retries => {
                // Retry on server errors (5xx)
                attempts_counter += 1;
                println!("Retrying... Attempt {}/{}", attempts_counter, retries);
                sleep(Duration::from_secs(5)).await
            },
            Ok(res) => {
                println!("Retrying... Attempt {}/{}", attempts_counter, retries);
                return Ok(Vec::new())
            }
            Err(e) if attempts_counter < retries => {
                // Retry on network or timeout errors
                attempts_counter += 1;
                println!("Error: {}. Retrying... Attempt {}/{}", e, attempts_counter, retries);
                sleep(Duration::from_secs(5)).await;
            }
            Err(e) => {
                return Err(e); // No more retries
            }
        }
    }
}



fn convert_map_to_object(token_prices_map: HashMap<String, HashMap<String, String>>) -> Vec<TokenInfo> {
    let mut list_of_objects: Vec<TokenInfo> = Vec::new();

    for (key, value) in token_prices_map.iter() {
        let mut price_list: Vec<TokenPrice> = Vec::new();

        for (currency, price) in value.iter() {
            price_list.push(TokenPrice{currency:currency.to_string(), price: price.to_string()})
        }

        list_of_objects.push(TokenInfo{token_ticker: key.to_string(), price_list: price_list});
    }

    list_of_objects
}


pub fn deserialize_nested_hashmap<'de, D>(
    deserializer: D,
) -> Result<HashMap<String, HashMap<String, String>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw: HashMap<String, HashMap<String, Value>> = HashMap::deserialize(deserializer)?;

    let mut result = HashMap::new();

    for (key, inner_map) in raw {
        let mut new_inner_map = HashMap::new();

        for (inner_key, inner_value) in inner_map {
            let string_value = match inner_value {
                Value::Number(num) => num.to_string(), // Convert numbers to strings
                Value::String(s) => s,                // Keep strings as-is
                _ => {
                    return Err(serde::de::Error::custom(
                        "Unsupported type for inner value; expected number or string",
                    ))
                }
            };

            new_inner_map.insert(inner_key, string_value);
        }

        result.insert(key, new_inner_map);
    }

    Ok(result)
}
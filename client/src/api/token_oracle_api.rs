use crate::models::api_models::{AccessToken, TokenInfo};

pub async fn get_token_prices(access_token: AccessToken) -> anyhow::Result<Vec<TokenInfo>> {
    let oracle_url = &crate::config::get_config().oracle_url;
    let url = format!("{}/prices", &oracle_url);
    let access_token_json = serde_json::to_string(&access_token)?;

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("X-Access-Token", access_token_json)
        .send()
        .await?;

    let api_response = response.text().await?;
    let token_info: Vec<TokenInfo> = serde_json::from_str(&api_response)?;

    Ok(token_info)
}
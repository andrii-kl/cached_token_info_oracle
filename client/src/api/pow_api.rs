use crate::models::api_models::{AccessToken, PuzzleSolution, PuzzleTask};
use anyhow::Result;

pub async fn get_puzzle_task() -> Result<PuzzleTask> {
    let oracle_url = &crate::config::get_config().oracle_url;
    let url = format!("{}/puzzle/get_task", &oracle_url);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .send()
        .await?;
    
    let api_response = response.text().await?;
    let puzzle_task: PuzzleTask = serde_json::from_str(&api_response)?;

    Ok(puzzle_task)
}

pub async fn verify_puzzles(puzzle_solution : PuzzleSolution) -> Result<AccessToken> {
    let oracle_url = &crate::config::get_config().oracle_url;
    let url = format!("{}/puzzle/check_resolution", &oracle_url);

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .json(&puzzle_solution)
        .send()
        .await?;

    let api_response = response.text().await?;
    let access_token: AccessToken = serde_json::from_str(&api_response)?;

    Ok(access_token)
}


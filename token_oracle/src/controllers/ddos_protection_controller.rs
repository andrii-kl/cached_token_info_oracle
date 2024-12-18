use rocket::{Build, Rocket, State};
use rocket::http::Status;
use rocket::serde::json::Json;
use thiserror::__private::AsDisplay;
use crate::config::AppConfig;
use crate::db::in_memory_token_info;
use crate::models::app_errors::{bad_request, build_response, AppError, ErrorResponse};
use crate::models::core_token_models::{AccessToken, PuzzleSolution, PuzzleTask};
use crate::services::puzzle_service::{create_puzzle_task, issue_access_token, verify_nonce, verify_signature, verify_signature_hex};


#[post("/check_resolution", data = "<solution>")]
async fn check_solution(solution: Json<PuzzleSolution>, app_config: &State<AppConfig>) -> Result<Json<AccessToken>, (Status, Json<ErrorResponse>)>  {
    println!("{:?}", &solution);

    match verify_signature_hex(&app_config.puzzle_signer_pk.as_bytes(), &solution.task.as_bytes(), &solution.signature){
        Ok(is_valid) => {
            if is_valid {
                // TODO make difficulty configurable
                match verify_nonce(&solution.task, &solution.nonce, 1){
                    true => {
                        match issue_access_token(&app_config.puzzle_signer_pk.as_bytes()){
                            Ok(access_token) => {
                                Ok(Json(access_token))
                            }
                            Err(error) => Err(error.into())
                        }
                    }
                    false => {
                        Err(bad_request("Solution for task is not correct"))
                    }
                }
            } else {
                Err(bad_request("Task signature is not valid"))
            }
        }
        Err(error) => Err(error.into()),
    }
}

#[get("/get_task")]
async fn get_task(app_config: &State<AppConfig>) -> Result<Json<PuzzleTask>, (Status, Json<ErrorResponse>)> {
    match create_puzzle_task(&app_config.puzzle_signer_pk.as_bytes(), app_config.puzzle_difficulty) {
        Ok(puzzle_task) => Ok(Json(puzzle_task)),
        Err(error) => Err(error.into()),
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_task, check_solution]
}
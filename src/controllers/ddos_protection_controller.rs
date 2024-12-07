use rocket::{Build, Rocket, State};
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::in_memory_cash;
use crate::models::app_errors::{AppError, ErrorResponse};
use crate::models::core_token_models::PuzzleTask;
use crate::services::puzzle_service::create_puzzle_task;

struct AccessToken {
    token: String,
    signature: String,
}


// #[get("/check_resolution")]
// async fn check_solution(task: PuzzleTusk, solution: String) -> PuzzleTusk {
//
//
// }

#[get("/get_task")]
async fn get_task(puzzle_signer_pk: &State<String>) -> Result<Json<PuzzleTask>, (Status, Json<ErrorResponse>)> {

    match create_puzzle_task(puzzle_signer_pk.as_bytes()).await {
        Ok(puzzle_task) => Ok(Json(puzzle_task)),
        Err(error) => {
            let error_response = match error {
                AppError::InvalidKeyLength(_) => ErrorResponse {
                    message: error.to_string(),
                    code: 400,
                },
                AppError::FromHexError(_) => ErrorResponse {
                    message: error.to_string(),
                    code: 400,
                },
            };

            Err((Status::from_code(error_response.code).unwrap_or(Status::InternalServerError), Json(error_response)))
        }
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_task]
}
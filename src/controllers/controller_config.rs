use crate::controllers::ddos_protection_controller;
use crate::controllers::token_info_oracle_controller;
use rocket::{Build, Rocket};
use rocket::response::Responder;
use rocket::{Request, Response};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use crate::config::AppConfig;
use crate::models::app_errors::AppError;
use crate::models::core_token_models::AccessToken;
use crate::services::puzzle_service::{verify_signature, verify_signature_hex};

pub fn rocket(app_conf: AppConfig) -> Rocket<Build> {
    rocket::build()
        .mount("/", token_info_oracle_controller::routes())
        .mount("/puzzle", ddos_protection_controller::routes())
        .manage(app_conf)
}

#[derive(Debug)]
pub struct Unauthorized;

impl<'r> Responder<'r, 'static> for Unauthorized {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        Response::build()
            .status(Status::Unauthorized)
            .sized_body("Access token is invalid or missing".len(), std::io::Cursor::new("Access token is invalid or missing"))
            .ok()
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AccessToken {
    type Error = AppError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let app_conf = match request.rocket().state::<AppConfig>() {
            None => {return Outcome::Error((Status::InternalServerError, AppError::MissingConfig()))}
            Some(value) => { value }
        };

        if !app_conf.ddos_protection() {
            return Outcome::Success(AccessToken::default())
        }

        if let Some(header) = request.headers().get_one("X-Access-Token") {
            if let Ok(access_token) = serde_json::from_str::<AccessToken>(header) {
                return match verify_signature_hex(app_conf.puzzle_signer_pk().as_bytes(),
                                                  access_token.access_token.to_string().as_bytes(),
                                                  &access_token.signature
                ) {
                    Ok(result) => {
                        match result {
                            true => { Outcome::Success(access_token) }
                            false => { Outcome::Error((Status::Unauthorized, AppError::InvalidToken())) }
                        }
                    }
                    Err(_) => Outcome::Error((Status::Unauthorized, AppError::InvalidToken()))
                }

            }
        }
        Outcome::Error((Status::Unauthorized, AppError::InvalidToken()))
    }
}


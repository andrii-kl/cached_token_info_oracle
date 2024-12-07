use crate::controllers::ddos_protection_controller;
use crate::controllers::token_info_oracle_controller;
use rocket::{Build, Rocket};

pub fn rocket(ddos_protection: bool, puzzle_signer_pk: String) -> Rocket<Build> {
    rocket::build()
        .mount("/", token_info_oracle_controller::routes())
        .mount("/puzzle", ddos_protection_controller::routes())
        .manage(ddos_protection)
        .manage(puzzle_signer_pk)
}

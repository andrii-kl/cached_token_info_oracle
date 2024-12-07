use std::env::VarError;
use config::{Config, File};
use serde::Deserialize;
use once_cell::sync::Lazy;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    coingecko_config: CoingeckoConfig,
    ddos_protection: bool,
    puzzle_signer_pk: String
}

#[derive(Debug, Deserialize)]
pub struct CoingeckoConfig {
    api_url: String,
    api_key: String,
    token_currencies: String,
    token_ids: String,
    token_update_period_sec: u64,
    number_attempts: u16,
}

impl CoingeckoConfig {
    pub fn api_url(&self) -> &String {
        &self.api_url
    }
    pub fn api_key(&self) -> &String {
        &self.api_key
    }
    pub fn token_currencies(&self) -> &String {
        &self.token_currencies
    }
    pub fn token_ids(&self) -> &String {
        &self.token_ids
    }
    pub fn token_update_period_sec(&self) -> &u64 {
        &self.token_update_period_sec
    }
    pub fn number_attempts(&self) -> &u16 {
        &self.number_attempts
    }
}

impl AppConfig {
    // Getter for `coingecko_config`
    pub fn coingecko_config(&self) -> &CoingeckoConfig {
        &self.coingecko_config
    }

    pub fn ddos_protection(&self) -> &bool {
        &self.ddos_protection
    }
    pub fn puzzle_signer_pk(&self) -> &String {
        &self.puzzle_signer_pk
    }
}

static SETTINGS: Lazy<AppConfig> = Lazy::new(|| {
    let env = std::env::var("APP_ENV");

    let config = match env {
        Ok(app_env) => {
            Config::builder()
                .add_source(File::with_name("config.toml"))
                .add_source(File::with_name(&format!("coingecko_conf_{}.toml", app_env)))
                .build()
                .expect("Failed to load configuration")
        }
        Err(_) => {
            panic!("Can't read env var")
        }
    };

    let mut app_conf = config
        .try_deserialize::<AppConfig>()
        .expect("Failed to deserialize configuration");

    app_conf
});

// Function to get a reference to the global settings
pub fn get_config() -> &'static AppConfig {
    &SETTINGS
}
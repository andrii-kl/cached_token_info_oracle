use config::{Config, File};
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub coingecko_config: CoingeckoConfig,
    pub ddos_protection: bool,
    pub puzzle_signer_pk: String,
    pub puzzle_difficulty: u8,
    pub one_time_access_tokens: bool
}

#[derive(Debug, Deserialize, Clone)]
pub struct CoingeckoConfig {
    pub api_url: String,
    pub api_key: String,
    pub token_currencies: String,
    pub token_ids: String,
    pub token_update_period_sec: u64,
    pub number_attempts: u16,
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
        Err(error) => {
            panic!("{}", error)
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
use config::{Config, File};
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub oracle_url:String,
}

static SETTINGS: Lazy<AppConfig> = Lazy::new(|| {
    let config = Config::builder()
                .add_source(File::with_name("client/config.toml"))
                .build()
                .unwrap_or_else(|e| {
                    panic!("Failed to load configuration: {}", e);
                });

    let app_conf = config
        .try_deserialize::<AppConfig>()
        .unwrap_or_else(|e| {
            panic!("Failed to deserialize configuration: {}", e);
        });

    app_conf
});

// Function to get a reference to the global settings
pub fn get_config() -> &'static AppConfig {
    &SETTINGS
}
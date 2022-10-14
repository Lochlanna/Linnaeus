// This module contain functions that are useful to setup the environment ready for testing

use super::Linnaeus;
use config::Config;
use serde::Deserialize;
use simple_logger::SimpleLogger;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    api_key: String,
    api_private_key: String,
    api_passphrase: String,
    base_url: String,
    ws_url: String,
}

impl AppConfig {
    pub fn load(filenames: &[(&str, bool)]) -> Self {
        let mut config = Config::builder()
            .add_source(config::Environment::with_prefix("COIN").try_parsing(true));
        for (file, required) in filenames {
            config = config.add_source(config::File::with_name(file).required(*required));
        }
        let app: AppConfig = config.build().unwrap().try_deserialize().unwrap();
        app
    }
    pub fn api_key(&self) -> &str {
        &self.api_key
    }
    pub fn api_private_key(&self) -> &str {
        &self.api_private_key
    }
    pub fn api_passphrase(&self) -> &str {
        &self.api_passphrase
    }
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
    pub fn ws_url(&self) -> &str {
        &self.ws_url
    }
}

use std::sync::Once;

static INIT: Once = Once::new();
const CONFIG_FILE: &str = "../TestConfig.toml";
const SECRET_FILE: &str = "../TestSecrets.toml";
/// Setup function that is only run once, even if called multiple times.
pub fn setup() -> Linnaeus {
    INIT.call_once(|| {
        SimpleLogger::new()
            .env()
            .with_level(log::LevelFilter::Info)
            .init()
            .unwrap();
    });
    let cfg = AppConfig::load(&[(CONFIG_FILE, false), (SECRET_FILE, false)]);
    let bin = Linnaeus::new(
        cfg.api_key(),
        cfg.api_private_key(),
        cfg.api_passphrase(),
        cfg.base_url(),
        cfg.ws_url()
    );
    bin
}
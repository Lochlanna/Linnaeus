// This module contain functions that are useful to setup the environment ready for testing

use std::io;
use std::io::Write;
use super::Linnaeus;
use config::Config;
use serde::Deserialize;
use simple_logger::SimpleLogger;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    keys: Vec<(String, String)>,
    base_url: String,
    ws_url: String,
}

pub trait LogErr {
    fn trace(self) -> Self;
    fn error(self) -> Self;
}

impl<A,B> LogErr for Result<A,B> where B: std::error::Error{
    fn trace(self) -> Self {
        match &self {
            Err(e) => {
                trace!("Error -> {}", e);
            }
            _ => {}
        }
        self
    }
    fn error(self) -> Self {
        match &self {
            Err(e) => {
                error!("Error -> {}", e);
            }
            _ => {}
        }
        self
    }
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
    pub fn keys(&self) -> Vec<KrakenKeyPair> {
        self.keys
            .iter()
            .map(|(api_key, private_key)| KrakenKeyPair::new(api_key, private_key))
            .collect()
    }
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
    pub fn ws_url(&self) -> &str {
        &self.ws_url
    }
}

use linnaeus_request::KrakenKeyPair;
use std::sync::Once;
use log::{error, trace};

static INIT: Once = Once::new();
const CONFIG_FILE: &str = "../TestConfig.toml";
const SECRET_FILE: &str = "../TestSecrets.toml";
/// Setup function that is only run once, even if called multiple times.
pub fn setup() -> Linnaeus {
    INIT.call_once(|| {
        SimpleLogger::new()
            .env()
            .with_level(log::LevelFilter::Trace)
            .init()
            .unwrap();
    });
    let cfg = AppConfig::load(&[(CONFIG_FILE, false), (SECRET_FILE, false)]);
    let bin = Linnaeus::new(cfg.keys(), cfg.base_url(), cfg.ws_url());
    bin
}

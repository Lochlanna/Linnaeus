use std::str::FromStr;

use display_json::DebugAsJson;
use display_json::DisplayAsJsonPretty;
use log::{error, warn};
use serde::{Deserialize, Serialize};
use strum::Display as EnumDisplay;
use strum::EnumString;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SignatureGenerationError {
    #[error("Error serializing signature generation input -> {0}")]
    SerializationError(#[from] serde_urlencoded::ser::Error),
    #[error("Error doing a base64 decode of secret key -> {0}")]
    SecretError(#[from] base64::DecodeError),
    #[error("Error creating hmac generator from secret")]
    InvalidSecret,
}

#[derive(DebugAsJson, Deserialize, Serialize, EnumDisplay, EnumString)]
pub enum Severity {
    #[serde(rename = "E")]
    #[strum(serialize="E",serialize="e")]
    Error,
    #[serde(rename = "W")]
    #[strum(serialize="W",serialize="w")]
    Warning,
}

#[derive(DebugAsJson, Deserialize, Serialize, EnumDisplay, EnumString)]
pub enum Category {
    General,
    Auth,
    API,
    Query,
    Order,
    Trade,
    Funding,
    Service,
}
#[derive(Error, DebugAsJson, Deserialize, Serialize)]
#[error("Kraken error -> {severity}{category}:{message}")]
pub struct KrakenError {
    severity: Severity,
    category: Category,
    message: String,
}

impl KrakenError {
    pub fn log(&self) {
        match self.severity {
            Severity::Error => error!("error from kraken: {}", self),
            Severity::Warning => warn!("warning from kraken: {}", self),
        }
    }
}

impl TryFrom<&str> for KrakenError {
    type Error = RequestError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (severity_category, message) = match value.split_once(':') {
            Some(split) => split,
            None => {
                return Err(RequestError::ParsingError(format!(
                    "Expecting : as dividor. Input was {}",
                    value
                )))
            }
        };
        let severity = match Severity::from_str(&severity_category[..1]) {
            Ok(s) => s,
            Err(_) => {
                return Err(RequestError::ParsingError(format!(
                    "Invalid character for severity. Expecting E or W. Input was {}",
                    value
                )))
            }
        };

        let category = match Category::from_str(&severity_category[1..]) {
            Ok(c) => c,
            _ => {
                return Err(RequestError::ParsingError(format!(
                    "Invalid Category. Input was {}",
                    value
                )))
            }
        };
        Ok(Self {
            severity,
            category,
            message: message.to_string(),
        })
    }
}

#[derive(Error, DebugAsJson, Deserialize, Serialize, DisplayAsJsonPretty)]
pub struct KrakenErrors {
    pub errors: Vec<KrakenError>,
}

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("Http Error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Error while signing request -> {0}")]
    SignatureGeneration(#[from] SignatureGenerationError),
    #[error("KrakenErrors -> {0}")]
    Kraken(#[from] KrakenErrors),
    #[error("Couldn't deserialize data from request -> {0} -> string was {1}")]
    DeserializationError(#[source] serde_json::error::Error, String),
    #[error("Couldn't parse response -> {0}")]
    ParsingError(String),
    #[error("An error occurred with message -> {0}")]
    Other(String),
}

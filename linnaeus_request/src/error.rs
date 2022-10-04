use display_json::DebugAsJson;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SignatureGenerationError {
    #[error("Error serializing signature generation input -> {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Error doing a base64 decode of secret key -> {0}")]
    SecretError(#[from] base64::DecodeError),
    #[error("Error creating hmac generator from secret")]
    InvalidSecret,
}

#[derive(Error, DebugAsJson, Deserialize, Serialize)]
#[error("Kraken error -> {code}:{message}")]
pub struct KrakenError {
    #[serde(skip_deserializing)]
    code: u16,
    message: String,
}

impl KrakenError {
    pub fn set_code(&mut self, code: u16) {
        self.code = code;
    }
    pub fn code(&self) -> u16 {
        self.code
    }
    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("Http Error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Error while signing request -> {0}")]
    SignatureGeneration(#[from] SignatureGenerationError),
    #[error(transparent)]
    Kraken(#[from] KrakenError),
    #[error("Couldn't deserialize data from request -> {0} -> string was {1}")]
    DeserializationError(#[source] serde_json::error::Error, String),
    #[error("An error occurred with message -> {0}")]
    Other(String),
}

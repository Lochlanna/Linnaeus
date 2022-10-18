use std::str::FromStr;

use display_json::DebugAsJson;
use display_json::DisplayAsJsonPretty;
use log::{error, warn};
use serde::{Deserialize, Serialize};
use strum::Display as EnumDisplay;
use strum::EnumString;
use thiserror::Error;
use derive_getters::Getters;

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

#[derive(DebugAsJson, Deserialize, Serialize, EnumDisplay, EnumString)]
pub enum KrakenErrorMessage {
    ///The request payload is malformed, incorrect or ambiguous.
    #[strum(serialize="Invalid arguments")]
    InvalidArguments,
    ///Index pricing is unavailable for stop/profit orders on this pair.
    #[strum(serialize="Invalid arguments:Index unavailable")]
    InvalidArgumentsIndexUnavailable,
    ///The matching engine or API is offline
    #[strum(serialize="Unavailable")]
    Unavailable,
    ///Request can't be made at this time. (See SystemStatus endpoint.)
    #[strum(serialize="Market in cancel_only mode")]
    MarketInCancelOnlyMode,
    ///Request can't be made at this time. (See SystemStatus endpoint.)
    #[strum(serialize="Market in post_only mode")]
    MarketInPostOnlyMode,
    ///The request timed out according to the default or specified deadline
    #[strum(serialize="Deadline elapsed")]
    DeadlineElapsed,
    ///An invalid API-Key header was supplied (see Authentication section)
    #[strum(serialize="Invalid key")]
    InvalidKey,
    ///An invalid API-Sign header was supplied (see Authentication section)
    #[strum(serialize="Invalid signature")]
    InvalidSignature,
    ///An invalid nonce was supplied (see Authentication section)
    #[strum(serialize="Invalid nonce")]
    InvalidNonce,
    ///API key doesn't have permission to make this request.
    #[strum(serialize="Permission denied")]
    PermissionDenied,
    ///User/tier is ineligible for margin trading
    #[strum(serialize="Cannot open position")]
    CannotOpenPosition,
    ///User has exceeded their margin allowance
    #[strum(serialize="Margin allowance exceeded")]
    MarginAllowanceExceeded,
    ///Client has insufficient equity or collateral
    #[strum(serialize="Margin level too low")]
    MarginLevelTooLow,
    ///Client would exceed the maximum position size for this pair
    #[strum(serialize="Margin position size exceeded")]
    MarginPositionSizeExceeded,
    ///Exchange does not have available funds for this margin trade
    #[strum(serialize="Insufficient margin")]
    InsufficientMargin,
    ///Client does not have the necessary funds
    #[strum(serialize="Insufficient funds")]
    InsufficientFunds,
    ///Order size does not meet ordermin. (See AssetPairs endpoint.)
    #[strum(serialize="Order minimum not met")]
    OrderMinimumNotMet,
    #[strum(serialize="Orders limit exceeded")]
    OrdersLimitExceeded,
    #[strum(serialize="Rate limit exceeded")]
    RateLimitExceeded,
    #[strum(serialize="Positions limit exceeded")]
    PositionsLimitExceeded,
    #[strum(serialize="Unknown position")]
    UnknownPosition,
    Other(String)
}


#[derive(Error, DebugAsJson, Deserialize, Serialize, Getters)]
#[error("Kraken error -> {severity}{category}:{message}")]
pub struct KrakenError {
    severity: Severity,
    category: Category,
    message: KrakenErrorMessage,
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
        let message = KrakenErrorMessage::try_from(message).unwrap_or(KrakenErrorMessage::Other(message.to_string()));
        Ok(Self {
            severity,
            category,
            message,
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

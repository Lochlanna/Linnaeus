use std::collections::HashMap;

use chrono::{FixedOffset, Utc};
use derive_getters::Getters;
use derive_new::new;
use derive_setters::Setters;
use display_json::{DebugAsJson, DisplayAsJsonPretty};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum::Display as EnumDisplay;

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters)]
pub struct ServerTime {
    unixtime: u64,
    rfc1123: String,
}

impl ServerTime {
    pub fn time(&self) -> Result<chrono::DateTime<FixedOffset>, chrono::ParseError> {
        chrono::DateTime::parse_from_str(&self.rfc1123, chrono_parser::formats::RFC1123)
    }
}

#[derive(Debug, Serialize, Deserialize, EnumDisplay)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    // Kraken is operating normally. All order types may be submitted and trades can occur.
    Online,
    // The exchange is offline. No new orders or cancellations may be submitted.
    Maintenance,
    // Resting (open) orders can be cancelled but no new orders may be submitted. No trades will occur.
    CancelOnly,
    // Only post-only limit orders can be submitted. Existing orders may still be cancelled. No trades will occur.
    PostOnly,
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters)]
pub struct SystemStatus {
    status: Status,
    timestamp: chrono::DateTime<Utc>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Setters)]
pub struct AssetInfoParams {
    #[serde(serialize_with = "crate::api::concat_strings_serializer")]
    asset: Vec<String>,
    aclass: Option<String>,
}

impl AssetInfoParams {
    pub fn new(assets: Vec<String>) -> Self {
        AssetInfoParams {
            asset: assets,
            aclass: None,
        }
    }
    pub fn new_with_class(assets: Vec<String>, class: String) -> Self {
        AssetInfoParams {
            asset: assets,
            aclass: Some(class),
        }
    }
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters)]
pub struct Asset {
    #[serde(rename = "aclass")]
    class: String,
    #[serde(rename = "altname")]
    alt_name: String,
    decimals: u16,
    display_decimals: u16,
}

pub type AssetInfo = HashMap<String, Asset>;
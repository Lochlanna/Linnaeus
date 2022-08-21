use crate::kraken::*;
use linnaeus_derive::kraken;
use chrono::{DateTime, FixedOffset};
use crate::kraken::enums;

#[derive(Debug, Deserialize, Serialize)]
#[kraken(POST,"/public/Time", TimeResult, false)]
pub struct Time {}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimeResult{
    unixtime: u128,
    rfc1123: String
}

#[derive(Debug, Deserialize, Serialize)]
#[kraken(POST,"/public/SystemStatus", SystemStatusResult, false)]
pub struct SystemStatus {}

#[derive(Debug, Deserialize, Serialize)]
pub struct SystemStatusResult{
    status: enums::KrakenSystemStatus,
    timestamp: DateTime<FixedOffset>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssetInfoResponse {
    #[serde(rename(serialize = "aclass", deserialize = "aclass"))]
    class: String,
    #[serde(rename(serialize = "altname", deserialize = "altname"))]
    alt_name: String,
    decimals:u32,
    display_decimals: u32
}
type AssetInfoResponseCollection = std::collections::HashMap<String,AssetInfoResponse>;

#[derive(Debug, Deserialize, Serialize)]
#[kraken(POST,"/public/Assets", AssetInfoResponseCollection, false)]
pub struct AssetInfo {
    asset: enums::Asset,
    #[serde(skip_serializing_if = "Option::is_none", rename(serialize = "aclass", deserialize = "aclass"))]
    class: Option<String>
}

impl AssetInfo {
    pub fn new(asset: enums::Asset, class: Option<String>) -> Self {
        Self { asset, class }
    }
}

//TODO Tradable asset pairs?

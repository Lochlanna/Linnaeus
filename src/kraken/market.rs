use crate::kraken::*;
use linnaeus_derive::kraken;
use chrono::{DateTime, FixedOffset};

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
#[serde(rename_all = "snake_case")]
enum KrakenSystemStatus {
    Online,
    Maintenance,
    CancelOnly,
    PostOnly,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SystemStatusResult{
    status: KrakenSystemStatus,
    timestamp: DateTime<FixedOffset>
}
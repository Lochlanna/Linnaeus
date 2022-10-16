pub mod user_staking;
pub mod user_funding;
pub mod user_trading;
pub mod user_data;
pub mod market_data;


use chrono::{DateTime, TimeZone};
use chrono::Utc;

pub fn concat_strings_serializer<S>(i: &[String],serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
    let out = i.join(",");
    serializer.serialize_str(&out)
}


pub fn datetime_from_timestamp_deserializer<'de, D>(deserialize: D) -> Result<DateTime<Utc>, D::Error> where D: serde::Deserializer<'de> {
    let timestamp= serde::Deserialize::deserialize(deserialize)?;
    Ok(Utc.timestamp(timestamp, 0))
}
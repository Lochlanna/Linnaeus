pub mod user_staking;
pub mod user_funding;
pub mod user_trading;
pub mod user_data;
pub mod market_data;


use chrono::{DateTime, TimeZone};
use chrono::Utc;
use rust_decimal::Decimal;
use serde::de::Error;

pub fn concat_strings_serializer<S>(i: &[String],serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
    let out = i.join(",");
    serializer.serialize_str(&out)
}


pub fn datetime_from_timestamp_deserializer<'de, D>(deserialize: D) -> Result<DateTime<Utc>, D::Error> where D: serde::Deserializer<'de> {
    let timestamp= serde::Deserialize::deserialize(deserialize)?;
    Ok(Utc.timestamp(timestamp, 0))
}

pub fn datetime_from_float_timestamp_deserializer<'de, D>(deserialize: D) -> Result<DateTime<Utc>, D::Error> where D: serde::Deserializer<'de> {
    let timestamp:Decimal = serde::Deserialize::deserialize(deserialize)?;
    let subsec = timestamp.fract() * rust_decimal_macros::dec!(1e+9);
    let nanos: u32 = match subsec.try_into() {
        Ok(subsec) => subsec,
        Err(e) => return Err(D::Error::custom(format!("Failed to get sub-seconds from decimal -> {}", e)))
    };
    let seconds: i64 = match timestamp.floor().try_into() {
        Ok(subsec) => subsec,
        Err(e) => return Err(D::Error::custom(format!("Failed to get sub-seconds from decimal -> {}", e)))
    };
    Ok(Utc.timestamp(seconds, nanos))
}

pub fn u128_from_string<'de, D>(deserialize: D) -> Result<u128, D::Error> where D: serde::Deserializer<'de> {
    let u128_str: String = serde::Deserialize::deserialize(deserialize)?;
    match u128_str.parse::<u128>() {
        Ok(v) => Ok(v),
        Err(e) => Err(D::Error::custom(format!("Failed to convert string to u128 -> {}", e)))
    }
}
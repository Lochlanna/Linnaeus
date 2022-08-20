use derive_getters::Getters;
use display_json::{DebugAsJson, DisplayAsJsonPretty};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TimestampSecondsWithFrac};

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    price: Decimal,
    whole_lot_volume: i64,
    lot_volume: Decimal,
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClosePrice {
    price: Decimal,
    lot_volume: Decimal,
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DayRolling {
    today: Decimal,
    last_24_hours: Decimal,
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct Ticker {
    #[serde(rename = "a")]
    ask: Price,
    #[serde(rename = "b")]
    bid: Price,
    #[serde(rename = "c")]
    close: ClosePrice,
    #[serde(rename = "v")]
    volume: DayRolling,
    #[serde(rename = "p")]
    volume_weighted_average_price: DayRolling,
    #[serde(rename = "l")]
    low_price: DayRolling,
    #[serde(rename = "h")]
    high_price: DayRolling,
    #[serde(rename = "o")]
    open_price: DayRolling,
}

#[serde_as]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OHLC {
    #[serde_as(as = "TimestampSecondsWithFrac<String>")]
    time: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "etime")]
    #[serde_as(as = "TimestampSecondsWithFrac<String>")]
    end_time: chrono::DateTime<chrono::Utc>,
    open: Decimal,
    high: Decimal,
    low: Decimal,
    close: Decimal,
    #[serde(rename = "vwap")]
    volume_weighted_average_price: Decimal,
    volume: Decimal,
    count: u64,
}

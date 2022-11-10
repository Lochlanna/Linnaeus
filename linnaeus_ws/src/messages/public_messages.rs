use derive_getters::Getters;
use display_json::{DebugAsJson, DisplayAsJsonPretty};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TimestampSecondsWithFrac, DisplayFromStr};

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct Price {
    price: Decimal,
    whole_lot_volume: i64,
    lot_volume: Decimal,
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct ClosePrice {
    price: Decimal,
    lot_volume: Decimal,
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
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
pub struct OHLC {
    #[serde_as(as = "TimestampSecondsWithFrac<String>")]
    time: chrono::DateTime<chrono::Utc>,
    #[serde_as(as = "TimestampSecondsWithFrac<String>")]
    end_time: chrono::DateTime<chrono::Utc>,
    open: Decimal,
    high: Decimal,
    low: Decimal,
    close: Decimal,
    volume_weighted_average_price: Decimal,
    volume: Decimal,
    count: u64,
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Clone)]
pub enum Side {
    #[serde(rename = "b")]
    Buy,
    #[serde(rename = "s")]
    Sell
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Clone)]
pub enum OrderType {
    #[serde(rename = "b")]
    Market,
    #[serde(rename = "s")]
    Limit
}

#[serde_as]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct Trade {
    price: Decimal,
    volume: Decimal,
    #[serde_as(as = "TimestampSecondsWithFrac<String>")]
    time: chrono::DateTime<chrono::Utc>,
    side: Side,
    order_type: OrderType,
    misc: String
}

pub type Trades = Vec<Trade>;

#[serde_as]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct Spread {
    price: Decimal,
    volume: Decimal,
    #[serde_as(as = "TimestampSecondsWithFrac<String>")]
    timestamp: chrono::DateTime<chrono::Utc>,
    bid_volume: Decimal,
    ask_volume: Decimal,
}

pub type Spreads = Vec<Spread>;

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Clone)]
pub enum BookUpdateType {
    #[serde(rename = "r")]
    Republished
}

#[serde_as]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct PriceLevel {
    price: Decimal,
    volume: Decimal,
    #[serde_as(as = "TimestampSecondsWithFrac<String>")]
    timestamp: chrono::DateTime<chrono::Utc>,
    update_type: Option<BookUpdateType>
}

#[serde_as]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct Book {
    #[serde(rename = "as")]
    ask_levels: Vec<PriceLevel>,
    #[serde(rename = "bs")]
    bid_levels: Vec<PriceLevel>,
    #[serde(rename = "c")]
    #[serde_as(as = "Option<DisplayFromStr>")]
    checksum: Option<u32>
}
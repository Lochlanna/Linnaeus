use std::fmt::Formatter;
use chrono::{TimeZone, Utc};
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

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Clone)]
pub enum BookUpdateType {
    #[serde(rename = "r")]
    Republished
}

#[serde_as]
#[derive(Serialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct PriceLevel {
    price: Decimal,
    volume: Decimal,
    #[serde_as(as = "TimestampSecondsWithFrac<String>")]
    timestamp: chrono::DateTime<chrono::Utc>,
    update_type: Option<BookUpdateType>
}
//This is nasty. Kraken why you like this
impl<'de> Deserialize<'de> for PriceLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
    {
        struct PriceLevelVisitor {}

        impl<'de> serde::de::Visitor<'de> for PriceLevelVisitor {
            type Value = PriceLevel;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("[Decimal, Decimal, Timestamp(frac seconds string), Option<BookUpdateType>]")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::SeqAccess<'de>,
            {
                use serde::de::Error as DeError;

                //TODO may need to do some magic to determine if there is a pair here...
                let price: Decimal = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(0, &self))?;

                let volume: Decimal = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(0, &self))?;

                let timestamp: String = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(0, &self))?;

                let timestamp= timestamp.parse::<f64>().or_else(|e| Err(DeError::custom(format!("unable to deserialize timestamp string as f64 -> {}", e))))?;

                let timestamp = Utc.timestamp(timestamp.floor() as i64, (timestamp.fract() * 1e+9) as u32);

                let update_type: Option<BookUpdateType> = seq
                    .next_element()?.unwrap_or(None);

                Ok(PriceLevel {
                    price,
                    volume,
                    timestamp,
                    update_type,
                })
            }
        }

        deserializer.deserialize_seq(PriceLevelVisitor {})
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct Book {
    #[serde(alias = "as")]
    #[serde(alias = "a")]
    ask_levels: Option<Vec<PriceLevel>>,
    #[serde(alias = "bs")]
    #[serde(alias = "b")]
    bid_levels: Option<Vec<PriceLevel>>,
    #[serde(rename = "c")]
    #[serde_as(as = "Option<DisplayFromStr>")]
    checksum: Option<u32>
}
use std::collections::HashMap;
use std::time::Duration;

use chrono::{FixedOffset, Utc};
use derive_getters::Getters;
use derive_setters::Setters;
use display_json::{DebugAsJson, DisplayAsJsonPretty};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum::Display as EnumDisplay;
use derive_new::new;

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters)]
pub struct ServerTime {
    #[serde(rename = "unixtime")]
    unix_time: u64,
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

#[derive(Debug, Serialize, Deserialize, EnumDisplay)]
#[serde(rename_all = "snake_case")]
pub enum TradableAssetPairDetailLevel {
    Info, //All info
    Leverage, //Leverage info
    Fees, //Fees schedule
    Margin //Margin info
}

impl Default for TradableAssetPairDetailLevel {
    fn default() -> Self {
        Self::Info
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Setters, Default)]
pub struct TradableAssetPairsParams {
    #[serde(serialize_with = "crate::api::concat_strings_serializer")]
    #[serde(rename = "pair")]
    pairs: Vec<String>,
    #[serde(rename = "info")]
    detail_level: TradableAssetPairDetailLevel
}

impl TradableAssetPairsParams {
    pub fn new(pairs: Vec<String>, detail_level: TradableAssetPairDetailLevel) -> Self {
        Self {
            pairs,
            detail_level
        }
    }
    pub fn add_pair(&mut self, pair: String) {
        self.pairs.push(pair)
    }
    pub fn add_pairs(&mut self, mut pairs: Vec<String>) {
        self.pairs.append(&mut pairs);
    }
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters)]
pub struct Fee {
    volume: Decimal,
    percent_fee: Decimal
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters)]
pub struct TradingAssetPair {
    #[serde(rename = "altname")]
    alt_name: String,
    #[serde(rename = "wsname")]
    websocket_name: Option<String>,
    #[serde(rename = "aclass_base")]
    base_asset_class: String, // TODO enum?
    #[serde(rename = "base")]
    base_asset_id: String,
    #[serde(rename = "aclass_quote")]
    quote_asset_class: String,
    #[serde(rename = "quote")]
    quote_asset_id: String,
    lot: Option<String>, // Deprecated but optional in case!
    pair_decimals: i32,
    cost_decimals: i32,
    lot_decimals: i32,
    lot_multiplier: i32,
    leverage_buy: Vec<i32>,
    leverage_sell: Vec<i32>,
    fees: Vec<Fee>,
    fees_maker: Vec<Fee>,
    fee_volume_currency: String,
    margin_call: i32,
    margin_stop: i32,
    #[serde(rename = "ordermin")]
    order_min: Decimal
}

pub type TradingAssetPairs = HashMap<String, TradingAssetPair>;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Setters, new)]
pub struct TickerInfoParams {
    pair: String
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters)]
pub struct Ask {
    price: Decimal,
    whole_lot_volume: Decimal,
    lot_volume: Decimal,
}

pub type Bid = Ask;

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters)]
pub struct LastTradeClosed {
    price: Decimal,
    lot_volume: Decimal,
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters)]
pub struct Volume {
    today: Decimal,
    last_24h: Decimal,
}

pub type NumberOfTrades = Volume;
pub type Low = Volume;
pub type High = Volume;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters)]
pub struct TickerInformation {
    #[serde(rename = "a")]
    ask: Ask,
    #[serde(rename = "b")]
    bid: Bid,
    #[serde(rename = "c")]
    last_trade_closed: LastTradeClosed,
    #[serde(rename = "v")]
    volume: Volume,
    #[serde(rename = "p")]
    volume_weighted_average_price: Volume,
    #[serde(rename = "t")]
    number_of_trades: NumberOfTrades,
    #[serde(rename = "l")]
    low: Low,
    #[serde(rename = "h")]
    high: High,
    #[serde(rename = "o")]
    open: Decimal
}

pub type MultiTickerInformation = HashMap<String, TickerInformation>;

#[derive(Debug, Serialize, Deserialize, EnumDisplay)]
pub enum Interval {
    #[serde(rename = "1")]
    OneMin,
    #[serde(rename = "5")]
    FiveMin,
    #[serde(rename = "15")]
    FifteenMin,
    #[serde(rename = "30")]
    ThirtyMin,
    #[serde(rename = "60")]
    OneHour,
    #[serde(rename = "240")]
    FourHour,
    #[serde(rename = "1440")]
    OneDay,
    #[serde(rename = "10080")]
    OneWeek,
    #[serde(rename = "21600")]
    FifteenDay
}

impl From<Interval> for Duration {
    fn from(i: Interval) -> Self {
        match i {
            Interval::OneMin => Duration::from_secs(60),
            Interval::FiveMin => Duration::from_secs(5*60),
            Interval::FifteenMin => Duration::from_secs(15*60),
            Interval::ThirtyMin => Duration::from_secs(30*60),
            Interval::OneHour => Duration::from_secs(60*60),
            Interval::FourHour => Duration::from_secs(240*60),
            Interval::OneDay => Duration::from_secs(1440*60),
            Interval::OneWeek => Duration::from_secs(10080*60),
            Interval::FifteenDay => Duration::from_secs(21600*60),
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Default)]
pub struct OHLCDataParams {
    pair: String,
    interval: Option<Interval>,
    since: Option<u64>
}

impl OHLCDataParams {
    pub fn new(pair: String, interval: Option<Interval>, since: Option<chrono::DateTime<Utc>>) -> Self {
        match since {
            None => Self { pair, interval, since: None },
            Some(since) => {
                Self {
                    pair,
                    interval,
                    since: Some(since.timestamp() as u64)
                }
            }
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters)]
pub struct TickData {
    #[serde(deserialize_with = "crate::api::datetime_from_timestamp_deserializer")]
    time: chrono::DateTime<Utc>,
    open: Decimal,
    high: Decimal,
    low: Decimal,
    close: Decimal,
    volume_weighted_average_price: Decimal,
    volume: Decimal,
    count: i64
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters)]
pub struct OHLCData {
    last: i64,
    #[serde(flatten)]
    tick_data: HashMap<String, Vec<TickData>>
}
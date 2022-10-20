use std::collections::HashMap;
use std::time::Duration;

use chrono::{FixedOffset, Utc};
use derive_getters::Getters;
use derive_setters::Setters;
use display_json::{DebugAsJson, DisplayAsJsonPretty};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::{skip_serializing_none, serde_as, StringWithSeparator, TimestampSeconds, TimestampSecondsWithFrac, DisplayFromStr};
use strum::Display as EnumDisplay;
use derive_new::new;
use serde_with::formats::{CommaSeparator};
use kraken_enums::{TradeablePair, Currency};

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
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

#[derive(Debug, Serialize, Deserialize, EnumDisplay, Clone)]
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

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct SystemStatus {
    status: Status,
    timestamp: chrono::DateTime<Utc>,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Setters, Clone)]
pub struct AssetInfoParams {
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, Currency>")]
    asset: Vec<Currency>,
    aclass: Option<String>,
}

impl AssetInfoParams {
    pub fn new(assets: Vec<Currency>) -> Self {
        AssetInfoParams {
            asset: assets,
            aclass: None,
        }
    }
    pub fn new_with_class(assets: Vec<Currency>, class: String) -> Self {
        AssetInfoParams {
            asset: assets,
            aclass: Some(class),
        }
    }
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct Asset {
    #[serde(rename = "aclass")]
    class: String,
    #[serde(rename = "altname")]
    alt_name: String,
    decimals: u16,
    display_decimals: u16,
}

pub type AssetInfo = HashMap<String, Asset>;

#[derive(Debug, Serialize, Deserialize, EnumDisplay, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TradableAssetPairDetailLevel {
    //All info
    Info,
    //Leverage info
    Leverage,
    //Fees schedule
    Fees,
    //Margin info
    Margin,
}

impl Default for TradableAssetPairDetailLevel {
    fn default() -> Self {
        Self::Info
    }
}

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Setters, Default, Clone)]
pub struct TradableAssetPairsParams {
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, TradeablePair>")]
    #[serde(rename = "pair")]
    pairs: Vec<TradeablePair>,
    #[serde(rename = "info")]
    detail_level: TradableAssetPairDetailLevel,
}

impl TradableAssetPairsParams {
    pub fn new(pairs: Vec<TradeablePair>, detail_level: TradableAssetPairDetailLevel) -> Self {
        Self {
            pairs,
            detail_level,
        }
    }
    pub fn add_pair(&mut self, pair: TradeablePair) {
        self.pairs.push(pair)
    }
    pub fn add_pairs(&mut self, mut pairs: Vec<TradeablePair>) {
        self.pairs.append(&mut pairs);
    }
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct Fee {
    volume: Decimal,
    percent_fee: Decimal,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct TradingAssetPair {
    #[serde(rename = "altname")]
    alt_name: String,
    #[serde(rename = "wsname")]
    websocket_name: Option<String>,
    #[serde(rename = "aclass_base")]
    base_asset_class: String,
    // TODO enum?
    #[serde(rename = "base")]
    base_asset_id: String,
    #[serde(rename = "aclass_quote")]
    quote_asset_class: String,
    #[serde(rename = "quote")]
    quote_asset_id: String,
    lot: Option<String>,
    // Deprecated but optional in case!
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
    order_min: Decimal,
}

pub type TradingAssetPairs = HashMap<String, TradingAssetPair>;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, new, Clone)]
pub struct TickerInfoParams {
    pair: TradeablePair,
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct Ask {
    price: Decimal,
    whole_lot_volume: Decimal,
    lot_volume: Decimal,
}

pub type Bid = Ask;

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct LastTradeClosed {
    price: Decimal,
    lot_volume: Decimal,
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct Volume {
    today: Decimal,
    last_24h: Decimal,
}

pub type NumberOfTrades = Volume;
pub type Low = Volume;
pub type High = Volume;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
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
    open: Decimal,
}

pub type MultiTickerInformation = HashMap<String, TickerInformation>;

#[derive(Debug, Serialize, Deserialize, EnumDisplay, Clone)]
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
    FifteenDay,
}

impl From<Interval> for Duration {
    fn from(i: Interval) -> Self {
        match i {
            Interval::OneMin => Duration::from_secs(60),
            Interval::FiveMin => Duration::from_secs(5 * 60),
            Interval::FifteenMin => Duration::from_secs(15 * 60),
            Interval::ThirtyMin => Duration::from_secs(30 * 60),
            Interval::OneHour => Duration::from_secs(60 * 60),
            Interval::FourHour => Duration::from_secs(240 * 60),
            Interval::OneDay => Duration::from_secs(1440 * 60),
            Interval::OneWeek => Duration::from_secs(10080 * 60),
            Interval::FifteenDay => Duration::from_secs(21600 * 60),
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Default, Clone)]
pub struct OHLCDataParams {
    pair: Option<TradeablePair>,
    interval: Option<Interval>,
    since: Option<u64>,
}

impl OHLCDataParams {
    pub fn new(pair: TradeablePair, interval: Option<Interval>, since: Option<chrono::DateTime<Utc>>) -> Self {
        match since {
            None => Self { pair: Some(pair), interval, since: None },
            Some(since) => {
                Self {
                    pair: Some(pair),
                    interval,
                    since: Some(since.timestamp() as u64),
                }
            }
        }
    }
}

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct TickData {
    #[serde_as(as = "TimestampSeconds<i64>")]
    time: chrono::DateTime<Utc>,
    open: Decimal,
    high: Decimal,
    low: Decimal,
    close: Decimal,
    volume_weighted_average_price: Decimal,
    volume: Decimal,
    count: i64,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct OHLCData {
    last: i64,
    #[serde(flatten)]
    tick_data: HashMap<String, Vec<TickData>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Clone)]
pub struct OrderBookParams {
    pair: TradeablePair,
    count: u16,
}

impl OrderBookParams {
    pub fn new(pair: TradeablePair) -> Self {
        Self {
            pair,
            count: 100,
        }
    }

    pub fn new_with_count(pair: TradeablePair, count: u16) -> Self {
        Self {
            pair,
            count,
        }
    }
}

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct OrderBookAsk {
    price: Decimal,
    volume: Decimal,
    #[serde_as(as = "TimestampSeconds<i64>")]
    timestamp: chrono::DateTime<Utc>,
}

pub type OrderBookBid = OrderBookAsk;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct OrderBook {
    asks: Vec<OrderBookAsk>,
    bids: Vec<OrderBookBid>,
}

pub type OrderBooks = HashMap<String, OrderBook>;


#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Default, Clone)]
pub struct RecentTradesParams {
    pair: Option<TradeablePair>,
    since: Option<u64>,
}

impl RecentTradesParams {
    pub fn new(pair: TradeablePair, since: Option<chrono::DateTime<Utc>>) -> Self {
        match since {
            None => Self { pair: Some(pair), since: None },
            Some(since) => {
                Self {
                    pair: Some(pair),
                    since: Some(since.timestamp() as u64),
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, EnumDisplay, Clone)]
pub enum Side {
    #[serde(rename = "b")]
    Buy,
    #[serde(rename = "s")]
    Sell,
}

#[derive(Debug, Serialize, Deserialize, EnumDisplay, Clone)]
pub enum TradeType {
    #[serde(rename = "m")]
    Market,
    #[serde(rename = "l")]
    Limit,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct TradeData {
    price: Decimal,
    volume: Decimal,
    #[serde_as(as = "TimestampSecondsWithFrac<f64>")]
    time: chrono::DateTime<Utc>,
    side: Side,
    trade_type: TradeType,
    miscellaneous: String,
}


#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct RecentTrades {
    #[serde_as(as = "DisplayFromStr")]
    last: u64,
    #[serde(flatten)]
    trade_data: HashMap<String, Vec<TradeData>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, new, Clone)]
pub struct RecentSpreadsParams {
    pair: TradeablePair,
    since: Option<u64>,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct SpreadData {
    id: u64,
    buy: Decimal,
    sell: Decimal
}

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct RecentSpreads {
    last: u64,
    #[serde(flatten)]
    spread_data: HashMap<String, Vec<SpreadData>>,
}
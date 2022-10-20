use crate::{Deserialize, Serialize};
use chrono::Utc;
use derive_getters::Getters;
use derive_new::new;
use derive_setters::Setters;
use display_json::{DebugAsJson, DisplayAsJsonPretty};
use rust_decimal::Decimal;
use serde_with::formats::CommaSeparator;
use serde_with::{
    serde_as, skip_serializing_none, DefaultOnError, StringWithSeparator, TimestampSeconds,
    TimestampSecondsWithFrac,
};
use std::collections::HashMap;
use strum::Display as EnumDisplay;
use strum::EnumString;

pub type AccountBalances = HashMap<String, Decimal>;

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Setters, new)]
pub struct TradeBalancesParams {
    asset: String,
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters)]
pub struct TradeBalances {
    ///Equivalent balance (combined balance of all currencies)
    #[serde(rename = "eb")]
    equivalent_balance: Decimal,
    ///Trade balance (combined balance of all equity currencies)
    #[serde(rename = "tb")]
    trade_balance: Decimal,
    ///Margin amount of open positions
    #[serde(rename = "m")]
    margin: Decimal,
    ///Unrealized net profit/loss of open positions
    #[serde(rename = "n")]
    net_open: Decimal,
    ///Cost basis of open positions
    #[serde(rename = "c")]
    cost_basis_open: Decimal,
    ///Current floating valuation of open positions
    #[serde(rename = "v")]
    floating_valuation_open: Decimal,
    ///Equity: trade balance + unrealized net profit/loss
    #[serde(rename = "e")]
    equity: Decimal,
    ///Free margin: Equity - initial margin (maximum margin available to open new positions)
    #[serde(rename = "mf")]
    free_margin: Decimal,
    ///Margin level: (equity / initial margin) * 100
    #[serde(rename = "ml")]
    margin_level: Option<Decimal>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Setters, new, Default)]
pub struct OpenOrdersParams {
    trades: bool,
    userref: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, EnumDisplay)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Pending,
    Open,
    Closed,
    Canceled,
    Expired,
}

#[derive(Debug, Serialize, Deserialize, EnumDisplay)]
#[serde(rename_all = "snake_case")]
pub enum Trigger {
    Last,
    Index,
}

impl Default for Trigger {
    fn default() -> Self {
        Self::Last
    }
}

#[derive(Debug, Serialize, Deserialize, EnumDisplay, EnumString)]
#[serde(rename_all = "snake_case")]
pub enum MiscInfo {
    #[strum(serialize = "stopped")]
    Stopped,
    #[strum(serialize = "touched")]
    Touched,
    #[strum(serialize = "liquidated")]
    Liquidated,
    #[strum(serialize = "partial")]
    Partial,
}

#[derive(Debug, Serialize, Deserialize, EnumDisplay)]
#[serde(rename_all = "snake_case")]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Serialize, Deserialize, EnumDisplay)]
#[serde(rename_all = "kebab-case")]
pub enum OrderType {
    Market,
    Limit,
    StopLoss,
    TakeProfit,
    StopLossLimit,
    TakeProfitLimit,
    SettlePosition,
}

#[derive(Debug, Serialize, Deserialize, EnumDisplay, EnumString)]
#[serde(rename_all = "snake_case")]
pub enum OrderFlags {
    ///post-only order (available when ordertype = limit)
    #[strum(serialize = "post")]
    Post,
    ///prefer fee in base currency (default if selling)
    #[strum(serialize = "fcib")]
    Fcib,
    ///prefer fee in quote currency (default if buying, mutually exclusive with fcib)
    #[strum(serialize = "fciq")]
    Fciq,
    ///disable market price protection for market orders
    #[strum(serialize = "nompp")]
    Nompp,
    ///order volume expressed in quote currency. This is supported only for market orders.
    #[strum(serialize = "viqc")]
    Viqc,
}

#[skip_serializing_none]
#[serde_as]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters)]
pub struct OrderDescription {
    pair: kraken_enums::TradeablePair,
    #[serde(rename = "type")]
    side: OrderSide,
    order_type: Option<OrderType>,
    price: Decimal,
    #[serde(rename = "price2")]
    secondary_price: Decimal,
    #[serde_as(deserialize_as = "DefaultOnError")]
    leverage: Option<Decimal>,
    #[serde(rename = "order")]
    order_description: String,
    #[serde(rename = "close")]
    conditional_close_description: Option<String>,
}

#[serde_as]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters)]
pub struct Order {
    #[serde(rename = "refid")]
    referral_order_transaction_id: Option<String>,
    status: OrderStatus,
    #[serde(rename = "opentm")]
    #[serde_as(as = "TimestampSecondsWithFrac<f64>")]
    open_time: chrono::DateTime<Utc>,
    #[serde(rename = "starttm")]
    #[serde_as(as = "TimestampSecondsWithFrac<f64>")]
    start_time: chrono::DateTime<Utc>,
    #[serde(rename = "expiretm")]
    #[serde_as(as = "TimestampSecondsWithFrac<f64>")]
    expire_time: chrono::DateTime<Utc>,
    #[serde(rename = "descr")]
    description: OrderDescription,
    #[serde(rename = "vol")]
    volume: Decimal,
    #[serde(rename = "vol_exec")]
    vol_executed: Decimal,
    cost: Decimal,
    fee: Decimal,
    price: Decimal,
    #[serde(rename = "stopprice")]
    stop_price: Decimal,
    #[serde(rename = "limitprice")]
    limit_price: Decimal,
    #[serde(default)]
    trigger: Trigger,
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, MiscInfo>")]
    misc: Vec<MiscInfo>,
    #[serde(rename = "oflags")]
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, OrderFlags>")]
    order_flags: Vec<OrderFlags>,
    #[serde(default)]
    trades: Vec<String>,
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty)]
pub struct OpenOrdersWrapper {
    pub open: HashMap<String, Order>,
}

#[derive(Debug, Serialize, Deserialize, EnumDisplay, EnumString)]
#[serde(rename_all = "snake_case")]
pub enum TimeType {
    Open,
    Close,
    Both,
}
impl Default for TimeType {
    fn default() -> Self {
        Self::Both
    }
}

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Setters, new, Default)]
pub struct ClosedOrdersParams {
    trades: bool,
    userref: Option<i32>,
    #[serde_as(as = "Option<TimestampSeconds<i64>>")]
    start: Option<chrono::DateTime<Utc>>,
    #[serde_as(as = "Option<TimestampSeconds<i64>>")]
    end: Option<chrono::DateTime<Utc>>,
    offset: Option<usize>,
    #[serde(rename = "closetime")]
    close_time: TimeType,
}

#[serde_as]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters)]
pub struct ClosedOrder {
    #[serde(flatten)]
    order: Order,
    #[serde_as(as = "TimestampSecondsWithFrac<f64>")]
    close_time: chrono::DateTime<Utc>,
    reason: Option<String>,
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters)]
pub struct ClosedOrders {
    closed: HashMap<String, Order>,
    count: usize,
}

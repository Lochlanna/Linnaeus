use derive_getters::Getters;
use display_json::{DebugAsJson, DisplayAsJsonPretty};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TimestampSecondsWithFrac};

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    Buy,
    Sell
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum OrderType {
    Market,
    Limit,
    StopLoss,
    TakeProfit,
    StopLossLimit,
    TakeProfitLimit,
    SettlePosition,
}

#[serde_as]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
#[serde(rename_all = "lowercase")]
pub struct OwnTrade {
    order_transaction_id: String,
    position_trade_id: String,
    pair: String,
    #[serde_as(as = "TimestampSecondsWithFrac<String>")]
    time: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "type")]
    side: Side,
    order_type: OrderType,
    price: Decimal,
    cost: Decimal,
    fee: Decimal,
    volume: Decimal,
    margin: Decimal,
    #[serde(rename = "userref")]
    user_reference_id: i64
}

pub type OwnTradePair = (String, OwnTrade);

pub type OwnTrades = Vec<OwnTradePair>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Pending,
    Open,
    Closed,
    Canceled,
    Expired,
}

#[serde_as]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
#[serde(rename_all = "lowercase")]
pub struct OrderDescription {
    pair: String,
    #[serde(rename = "position")]
    position_id: Option<String>,
    #[serde(rename = "type")]
    side:Side,
    order_type: OrderType,
    price: Decimal,
    #[serde(rename = "price2")]
    secondary_price: Decimal,
    leverage: Decimal,
    #[serde(rename = "order")]
    order_description: String,
    #[serde(rename = "close")]
    close_description: String
}

#[serde_as]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
#[serde(rename_all = "lowercase")]
pub struct OpenOrder {
    #[serde(rename = "refid")]
    reference_id: String,
    #[serde(rename = "userref")]
    user_reference_id: i64,
    status: OrderStatus,
    #[serde_as(as = "TimestampSecondsWithFrac<String>")]
    #[serde(rename = "opentm")]
    open_time: chrono::DateTime<chrono::Utc>,
    #[serde_as(as = "TimestampSecondsWithFrac<String>")]
    #[serde(rename = "starttm")]
    start_time: chrono::DateTime<chrono::Utc>,
    #[serde_as(as = "TimestampSecondsWithFrac<String>")]
    #[serde(rename = "expiretm")]
    expire_time: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "descr")]
    description: OrderDescription,
    #[serde(rename = "vol")]
    volume: Decimal,
    #[serde(rename = "vol_exec")]
    volume_executed: Decimal,
    cost: Decimal,
    fee: Decimal,
    #[serde(rename = "avg_price")]
    average_price: Decimal,
    stop_price:Decimal,
    limit_price: Decimal,
    misc: String,
    oflags: Option<String>,
    time_in_force: Option<String>, //TODO can this be a duration
    cancel_reason: Option<String>,
    rate_count: Option<i64>
}

pub type OpenOrderPair = (String, OpenOrder);

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
#[serde(rename_all = "lowercase")]
pub struct OrderStatusChange {
    status: OrderStatus
}

pub type OrderStatusChangePair = (String, OrderStatusChange);

/// Bit of a joke Kraken
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Clone)]
#[serde(untagged)]
pub enum OpenOrderOrStatusChange {
    OpenOrder(OpenOrderPair),
    StatusChange(OrderStatusChange)
}

pub type OpenOrders = Vec<OpenOrderOrStatusChange>;
use derive_getters::Getters;
use display_json::{DebugAsJson, DisplayAsJsonPretty};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;
use strum::Display as EnumDisplay;

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct Ping {
    #[serde(rename = "reqid")]
    request_id: i64,
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct Pong {
    #[serde(rename = "reqid")]
    request_id: i64,
}

#[serde(rename_all = "snake_case")]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Clone)]
pub enum SystemStatusCode {
    Online,
    Maintenance,
    CancelOnly,
    LimitOnly,
    PostOnly,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct SystemStatus {
    connection_id: i64,
    status: SystemStatusCode,
    version: String,
}

// Name 	Type 	Description
// event 	string 	subscribe
// reqid 	integer 	Optional - client originated ID reflected in response message
// pair 	array 	Optional - Array of currency pairs. Format of each pair is "A/B", where A and B are ISO 4217-A3 for standardized assets and popular unique symbol if not standardized.
// subscription 	object
// depth 	integer 	Optional - depth associated with book subscription in number of levels each side, default 10. Valid Options are: 10, 25, 100, 500, 1000
// interval 	integer 	Optional - Time interval associated with ohlc subscription in minutes. Default 1. Valid Interval values: 1|5|15|30|60|240|1440|10080|21600
// name 	string 	book|ohlc|openOrders|ownTrades|spread|ticker|trade|*, * for all available channels depending on the connected environment
// ratecounter 	boolean 	Optional - whether to send rate-limit counter in updates (supported only for openOrders subscriptions; default = false)
// snapshot 	boolean 	Optional - whether to send historical feed data snapshot upon subscription (supported only for ownTrades subscriptions; default = true)
// token 	string 	Optional - base64-encoded authentication token for private-data endpoints

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone)]
#[repr(u32)]
pub enum Interval {
    OneMin = 1,
    FiveMin = 5,
    FifteenMin = 15,
    ThirtyMin = 30,
    OneHour = 60,
    FourHour = 240,
    OneDay = 1440,
    OneWeek = 10080,
    FifteenDay = 21600,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone)]
#[repr(u16)]
pub enum Depth {
    Ten = 10,
    TwentyFive = 25,
    OneHundred = 100,
    FiveHundred = 500,
    OneThousand = 1000,
}

#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct SubscribeInfo {
    depth: Option<Depth>,
    interval: Option<Interval>,
    name: String,
    ratecounter: Option<bool>,
    snapshot: Option<bool>,
    token: Option<String>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct Subscribe {
    #[serde(rename = "reqid")]
    request_id: i64,
    pair: Vec<String>, // TODO ISO 4217-A3 currency enum?
    subscription: SubscriptionInfo,
}

#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct UnSubscribeInfo {
    depth: Option<u16>,
    interval: Option<Interval>,
    name: String,
    token: Option<String>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct UnSubscribe {
    #[serde(rename = "reqid")]
    request_id: i64,
    pair: Vec<String>, // TODO ISO 4217-A3 currency enum?
    subscription: UnSubscribeInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Status {
    Subscribed,
    Unsubscribed,
}

#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct Subscription {
    depth: Option<Depth>,
    interval: Option<Interval>,
    max_rate_count: Option<i64>,
    name: String, //TODO is there an enum that could be used here
    token: Option<String>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct SubscriptionStatus {
    channel_name: String,
    #[serde(rename = "reqid")]
    request_id: i64,
    pair: Vec<String>, // TODO ISO 4217-A3 currency enum?
    status: Status,
    subscription: Subscription,
}

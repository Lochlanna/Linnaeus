use derive_getters::Getters;
use display_json::{DebugAsJson, DisplayAsJsonPretty};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;

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

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SystemStatusCode {
    Online,
    Maintenance,
    CancelOnly,
    LimitOnly,
    PostOnly,
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SystemStatus {
    connection_id: i64,
    status: SystemStatusCode,
    version: String,
}


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
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeInfo {
    depth: Option<Depth>,
    interval: Option<Interval>,
    name: String,
    ratecounter: Option<bool>,
    snapshot: Option<bool>,
    token: Option<String>,
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Subscribe {
    #[serde(rename = "reqid")]
    request_id: i64,
    pair: Vec<String>, // TODO ISO 4217-A3 currency enum?
    subscription: SubscribeInfo,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UnSubscribeInfo {
    depth: Option<u16>,
    interval: Option<Interval>,
    name: String,
    token: Option<String>,
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
#[serde(rename_all = "camelCase")]
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
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    depth: Option<Depth>,
    interval: Option<Interval>,
    max_rate_count: Option<i64>,
    name: String, //TODO is there an enum that could be used here
    token: Option<String>,
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionStatus {
    channel_name: String,
    #[serde(rename = "reqid")]
    request_id: i64,
    pair: Vec<String>, // TODO ISO 4217-A3 currency enum?
    status: Status,
    subscription: Subscription,
}

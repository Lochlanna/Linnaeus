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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SystemStatus {
    #[serde(rename = "connectionID")]
    connection_id: i64,
    status: SystemStatusCode,
    version: String,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy)]
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

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy)]
#[repr(u16)]
pub enum Depth {
    Ten = 10,
    TwentyFive = 25,
    OneHundred = 100,
    FiveHundred = 500,
    OneThousand = 1000,
}


//TODO write custom serailzie deserialize on super::Channel and ditch this one
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SubscribableChannel {
    Book,
    #[serde(rename = "ohlc")]
    OHLC,
    OpenOrders,
    OwnTrades,
    Spread,
    Ticker,
    Trade
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeInfo {
    depth: Option<Depth>,
    interval: Option<Interval>,
    name: SubscribableChannel,
    ratecounter: Option<bool>,
    snapshot: Option<bool>,
    token: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Subscribe {
    #[serde(rename = "reqid")]
    request_id: Option<i64>,
    pair: Option<Vec<String>>, // TODO ISO 4217-A3 currency enum?
    subscription: SubscribeInfo,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UnSubscribeInfo {
    depth: Option<u16>,
    interval: Option<Interval>,
    name: SubscribableChannel,
    token: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UnSubscribe {
    #[serde(rename = "reqid")]
    request_id: Option<i64>,
    pair: Option<Vec<String>>, // TODO ISO 4217-A3 currency enum?
    subscription: UnSubscribeInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Subscribed,
    Unsubscribed,
    Error
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
    #[serde(rename = "channelID")]
    channel_id: Option<i64>,
    error_message: Option<String>,
    channel_name: crate::messages::Channel,
    #[serde(rename = "reqid")]
    request_id: Option<i64>,
    pair: Option<String>, // TODO ISO 4217-A3 currency enum?
    status: Status,
    subscription: Subscription,
}


#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use pretty_assertions::assert_str_eq;
    use crate::messages::*;
    use crate::test_utils;


    #[test]
    fn ping() {
        let j = test_utils::load_test_json("general/ping")
            .expect("couldn't load test json from file");
        let message: Message = serde_json::from_str(&j)
            .expect("failed to deserialize test json to message");
        let Message::Event(event) = message else {
            panic!("expected event");
        };
        let Event::Ping(ping) = event else {
            panic!("expected ping event")
        };
        assert_eq!(ping.request_id, 42)
    }

    #[test]
    fn pong() {
        let j = test_utils::load_test_json("general/pong")
            .expect("couldn't load test json from file");
        let message: Message = serde_json::from_str(&j)
            .expect("failed to deserialize test json to message");
        let Message::Event(event) = message else {
            panic!("expected event");
        };
        let Event::Pong(pong) = event else {
            panic!("expected pong event")
        };
        assert_eq!(pong.request_id, 42)
    }

    #[test]
    fn heartbeat() {
        let j = test_utils::load_test_json("general/heartbeat")
            .expect("couldn't load test json from file");
        let message: Message = serde_json::from_str(&j)
            .expect("failed to deserialize test json to message");
        let Message::Event(event) = message else {
            panic!("expected event");
        };
        let Event::Heartbeat = event else {
            panic!("expected heartbeat event")
        };
    }

    #[test]
    fn system_status() {
        let j = test_utils::load_test_json("general/system_status")
            .expect("couldn't load test json from file");
        let message: Message = serde_json::from_str(&j)
            .expect("failed to deserialize test json to message");
        let Message::Event(event) = message else {
            panic!("expected event");
        };
        let Event::SystemStatus(system_status) = event else {
            panic!("expected system status event")
        };
        assert_eq!(system_status.connection_id, 8628615390848610000)
    }

    #[test]
    fn subscribe_ohlc() {
        let expected_json = test_utils::load_test_json("general/subscribe/subscribe_ohlc")
            .expect("couldn't load test json from file");
        let subscribe_message = Subscribe {
            request_id: None,
            pair: Some(vec!["XBT/EUR".to_string()]),
            subscription: SubscribeInfo {
                depth: None,
                interval: Some(Interval::FiveMin),
                name: SubscribableChannel::OHLC,
                ratecounter: None,
                snapshot: None,
                token: None
            }
        };
        let subscribe_event = Event::Subscribe(subscribe_message);
        let produced_json = serde_json::to_string_pretty(&subscribe_event).expect("couldn't serialise subscription");
        assert_str_eq!(produced_json, expected_json)
    }

    #[test]
    fn subscribe_own_trades() {
        let expected_json = test_utils::load_test_json("general/subscribe/subscribe_own_trades")
            .expect("couldn't load test json from file");
        let subscribe_message = Subscribe {
            request_id: None,
            pair: None,
            subscription: SubscribeInfo {
                depth: None,
                interval: None,
                name: SubscribableChannel::OwnTrades,
                ratecounter: None,
                snapshot: None,
                token: Some("WW91ciBhdXRoZW50aWNhdGlvbiB0b2tlbiBnb2VzIGhlcmUu".to_string())
            }
        };
        let subscribe_event = Event::Subscribe(subscribe_message);
        let produced_json = serde_json::to_string_pretty(&subscribe_event).expect("couldn't serialise subscription");
        assert_str_eq!(produced_json, expected_json)
    }

    #[test]
    fn subscribe_ticker() {
        let expected_json = test_utils::load_test_json("general/subscribe/subscribe_ticker")
            .expect("couldn't load test json from file");
        let subscribe_message = Subscribe {
            request_id: None,
            pair: Some(vec!["XBT/USD".to_string(), "XBT/EUR".to_string()]),
            subscription: SubscribeInfo {
                depth: None,
                interval: None,
                name: SubscribableChannel::Ticker,
                ratecounter: None,
                snapshot: None,
                token: None
            }
        };
        let subscribe_event = Event::Subscribe(subscribe_message);
        let produced_json = serde_json::to_string_pretty(&subscribe_event).expect("couldn't serialise subscription");
        assert_str_eq!(produced_json, expected_json)
    }

    #[test]
    fn unsubscribe_own_trades() {
        let expected_json = test_utils::load_test_json("general/unsubscribe/unsubscribe_own_trades")
            .expect("couldn't load test json from file");
        let unsubscribe_message = UnSubscribe {
            request_id: None,
            pair: None,
            subscription: UnSubscribeInfo {
                depth: None,
                interval: None,
                name: SubscribableChannel::OwnTrades,
                token: Some("WW91ciBhdXRoZW50aWNhdGlvbiB0b2tlbiBnb2VzIGhlcmUu".to_string())
            }
        };
        let unsubscribe_event = Event::Unsubscribe(unsubscribe_message);
        let produced_json = serde_json::to_string_pretty(&unsubscribe_event).expect("couldn't serialise subscription");
        assert_str_eq!(produced_json, expected_json)
    }

    #[test]
    fn unsubscribe_ticker() {
        let expected_json = test_utils::load_test_json("general/unsubscribe/unsubscribe_ticker")
            .expect("couldn't load test json from file");
        let unsubscribe_message = UnSubscribe {
            request_id: None,
            pair: Some(vec!["XBT/EUR".to_string(), "XBT/USD".to_string()]),
            subscription: UnSubscribeInfo {
                depth: None,
                interval: None,
                name: SubscribableChannel::Ticker,
                token: None
            }
        };
        let unsubscribe_event = Event::Unsubscribe(unsubscribe_message);
        let produced_json = serde_json::to_string_pretty(&unsubscribe_event).expect("couldn't serialise subscription");
        assert_str_eq!(produced_json, expected_json)
    }

    #[test]
    fn subscription_status_ohlc() {
        let j = test_utils::load_test_json("general/subscription_status/subscription_status_ohlc")
            .expect("couldn't load test json from file");
        let message: Message = serde_json::from_str(&j)
            .expect("failed to deserialize test json to message");
        let Message::Event(event) = message else {
            panic!("expected event");
        };
        let Event::SubscriptionStatus(sub_status) = event else {
            panic!("expected subscription status event")
        };
        assert!(matches!(sub_status.channel_name, Channel::OHLC(Interval::FiveMin)));
        assert_str_eq!(sub_status.pair.expect("expected a pair"), "XBT/EUR");
        assert!(matches!(sub_status.status, Status::Unsubscribed))
    }

    #[ignore = "own trades not channel yet not supported"]
    #[test]
    fn subscription_status_own_trades() {
        let j = test_utils::load_test_json("general/subscription_status/subscription_status_own_trades")
            .expect("couldn't load test json from file");
        let message: Message = serde_json::from_str(&j)
            .expect("failed to deserialize test json to message");
        let Message::Event(event) = message else {
            panic!("expected event");
        };
        let Event::SubscriptionStatus(sub_status) = event else {
            panic!("expected subscription status event")
        };
        // assert!(matches!(sub_status.channel_name, Channel::O));
        assert!(matches!(sub_status.status, Status::Subscribed))
    }

    #[test]
    fn subscription_status_ticker() {
        let j = test_utils::load_test_json("general/subscription_status/subscription_status_ticker")
            .expect("couldn't load test json from file");
        let message: Message = serde_json::from_str(&j)
            .expect("failed to deserialize test json to message");
        let Message::Event(event) = message else {
            panic!("expected event");
        };
        let Event::SubscriptionStatus(sub_status) = event else {
            panic!("expected subscription status event")
        };
        assert!(matches!(sub_status.channel_name, Channel::Ticker));
        assert_str_eq!(sub_status.pair.expect("expected a pair"), "XBT/EUR");
        assert!(matches!(sub_status.status, Status::Subscribed))
    }

    #[ignore = "variable depths not supported"]
    #[test]
    fn subscription_status_error() {
        let j = test_utils::load_test_json("general/subscription_status/subscription_status_error")
            .expect("couldn't load test json from file");
        let message: Message = serde_json::from_str(&j)
            .expect("failed to deserialize test json to message");
        let Message::Event(event) = message else {
            panic!("expected event");
        };
        let Event::SubscriptionStatus(sub_status) = event else {
            panic!("expected subscription status event")
        };
        assert!(matches!(sub_status.channel_name, Channel::Ticker));
        assert_str_eq!(sub_status.pair.expect("expected a pair"), "XBT/EUR");
        assert_str_eq!(sub_status.error_message.expect("expected an error message"), "Subscription depth not supported");
        assert!(matches!(sub_status.status, Status::Error));
        assert!(matches!(sub_status.subscription.depth.expect("expected a depth") as u16, 42));
    }
}
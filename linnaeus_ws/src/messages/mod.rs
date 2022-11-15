pub mod general_messages;
pub mod private_messages;
pub mod public_messages;

use derive_getters::Getters;
use display_json::{DebugAsJson, DisplayAsJsonPretty};
use serde::de::Error as DeError;
use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use strum::Display as DisplayEnum;

use crate::messages::private_messages::{OpenOrders, OwnTrades};
use general_messages::*;
use public_messages::*;

//TODO use this everywhere
pub type Pair = String;

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Clone)]
#[serde(untagged)]
pub enum Message {
    ChannelMessage(ChannelMessageWrapper),
    Event(Event),
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Clone)]
#[serde(tag = "event")]
#[serde(rename_all = "camelCase")]
pub enum Event {
    Ping(Ping),
    Pong(Pong),
    Heartbeat,
    SystemStatus(SystemStatus),
    Subscribe(Subscribe),
    Unsubscribe(UnSubscribe),
    SubscriptionStatus(SubscriptionStatus),
}

#[derive(
    Serialize,
    Deserialize,
    DebugAsJson,
    DisplayAsJsonPretty,
    Clone,
    PartialOrd,
    PartialEq,
    Ord,
    Eq,
    Hash,
)]
pub enum EventType {
    Ping,
    Pong,
    Heartbeat,
    SystemStatus,
    Subscribe,
    Unsubscribe,
    SubscriptionStatus,
}

impl From<&Event> for EventType {
    fn from(event: &Event) -> Self {
        match event {
            Event::Ping(_) => Self::Ping,
            Event::Pong(_) => Self::Pong,
            Event::Heartbeat => Self::Heartbeat,
            Event::SystemStatus(_) => Self::SystemStatus,
            Event::Subscribe(_) => Self::Subscribe,
            Event::Unsubscribe(_) => Self::Unsubscribe,
            Event::SubscriptionStatus(_) => Self::SubscriptionStatus,
        }
    }
}

impl Event {
    pub fn get_request_id(&self) -> Option<i64> {
        match self {
            Event::Ping(p) => Some(p.request_id().clone()),
            Event::Pong(p) => Some(p.request_id().clone()),
            Event::Subscribe(s) => s.request_id().clone(),
            Event::Unsubscribe(u) => u.request_id().clone(),
            Event::SubscriptionStatus(s) => s.request_id().clone(),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, DisplayEnum, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum Channel {
    Ticker,
    OHLC(Interval),
    Trade,
    Spread,
    Book(Depth),
    OwnTrades,
    OpenOrders,
}

impl From<&SubscribeInfo> for Channel {
    fn from(subscribe_info: &SubscribeInfo) -> Self {
        match subscribe_info.name() {
            SubscribableChannel::Book => Self::Book(subscribe_info.depth().unwrap_or_default()),
            SubscribableChannel::OHLC => Self::OHLC(subscribe_info.interval().unwrap_or_default()),
            SubscribableChannel::OpenOrders => Self::OpenOrders,
            SubscribableChannel::OwnTrades => Self::OwnTrades,
            SubscribableChannel::Spread => Self::Spread,
            SubscribableChannel::Ticker => Self::Ticker,
            SubscribableChannel::Trade => Self::Trade,
        }
    }
}

impl Channel {
    pub(crate) fn generate_identifier(&self, pair: &Pair) -> u64 {
        let mut hasher = ahash::AHasher::default();
        self.hash(&mut hasher);
        pair.hash(&mut hasher);
        hasher.finish()
    }

    pub(crate) fn generate_identifier_no_pair(&self) -> u64 {
        let mut hasher = ahash::AHasher::default();
        self.hash(&mut hasher);
        let no_pair: Option<Pair> = None;
        no_pair.hash(&mut hasher);
        hasher.finish()
    }
}

impl Serialize for Channel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Channel::Ticker => serializer.serialize_str("ticker"),
            Channel::OHLC(interval) => {
                serializer.serialize_str(format!("ohlc-{}", *interval as u32).as_str())
            }
            Channel::Trade => serializer.serialize_str("trade"),
            Channel::Spread => serializer.serialize_str("spread"),
            Channel::Book(depth) => {
                serializer.serialize_str(format!("book-{}", *depth as u16).as_str())
            }
            Channel::OwnTrades => serializer.serialize_str("ownTrades"),
            Channel::OpenOrders => serializer.serialize_str("openOrders"),
        }
    }
}

impl<'de> Deserialize<'de> for Channel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StringVisitor {}

        impl<'de> Visitor<'de> for StringVisitor {
            type Value = Channel;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("ticker|ohlc-{}|trade|spread|book-{}")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Channel::from_str(v).or_else(|e| Err(DeError::custom(e)))
            }
        }

        deserializer.deserialize_str(StringVisitor {})
    }
}

impl FromStr for Channel {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("input empty");
        }
        let mut split = s.splitn(2, '-');
        let Some(name) = split.next() else {
            return Err("no valid strings");
        };

        match name {
            "ticker" => return Ok(Channel::Ticker),
            "trade" => return Ok(Channel::Trade),
            "spread" => return Ok(Channel::Spread),
            "ownTrades" => return Ok(Channel::OwnTrades),
            "openOrders" => return Ok(Channel::OpenOrders),
            _ => {}
        }

        let Some(value) = split.next() else {
            return Err("unknown channel");
        };

        match name {
            "ohlc" => {
                let Ok(interval) = serde_json::from_str::<Interval>(value) else {
                    return Err("invalid value for ohlc interval");
                };
                Ok(Channel::OHLC(interval))
            }
            "book" => {
                let Ok(depth) = serde_json::from_str::<Depth>(value) else {
                    return Err("invalid value for book depth");
                };
                Ok(Channel::Book(depth))
            }
            _ => Err("unknown channel"),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub enum ChannelMessage {
    Ticker(Ticker),
    OHLC(OHLC),
    Trade(Trades),
    Spread(Spread),
    Book(Book),
    OwnTrades(OwnTrades),
    OpenOrders(OpenOrders),
}

impl ChannelMessage {
    fn new(
        channel: &Channel,
        data: serde_json::Value,
    ) -> Result<ChannelMessage, serde_json::Error> {
        let channel_message = match channel {
            Channel::Ticker => ChannelMessage::Ticker(serde_json::from_value(data)?),
            Channel::OHLC(_) => ChannelMessage::OHLC(serde_json::from_value(data)?),
            Channel::Trade => ChannelMessage::Trade(serde_json::from_value(data)?),
            Channel::Spread => ChannelMessage::Spread(serde_json::from_value(data)?),
            Channel::Book(_) => ChannelMessage::Book(serde_json::from_value(data)?),
            Channel::OwnTrades => ChannelMessage::OwnTrades(serde_json::from_value(data)?),
            Channel::OpenOrders => ChannelMessage::OpenOrders(serde_json::from_value(data)?),
        };
        Ok(channel_message)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Getters)]
pub struct Sequence {
    sequence: i64,
}

#[derive(Serialize, DebugAsJson, Clone, Getters, DisplayAsJsonPretty)]
pub struct ChannelMessageWrapper {
    id: i64,
    message: ChannelMessage,
    channel: Channel,
    pair: Option<String>,
    sequence: Option<Sequence>,
}

impl ChannelMessageWrapper {
    pub(crate) fn get_channel_identifier(&self) -> u64 {
        match &self.pair {
            None => self.channel.generate_identifier_no_pair(),
            Some(pair) => self.channel.generate_identifier(pair),
        }
    }
}

//This is nasty. Kraken why you like this
impl<'de> Deserialize<'de> for ChannelMessageWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ChannelMessageWrapperVisitor {}

        impl<'de> Visitor<'de> for ChannelMessageWrapperVisitor {
            type Value = ChannelMessageWrapper;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("[i64, ChannelMessage..., String, String]")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let min_num_objects = match seq.size_hint() {
                    None => 1,
                    Some(n) => {
                        if n <= 3 {
                            return Err(DeError::invalid_length(n, &self));
                        }
                        n - 3
                    }
                };

                let id: i64 = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(0, &self))?;
                let channel: Channel;
                let mut messages = Vec::with_capacity(min_num_objects);
                loop {
                    let val: Value = seq
                        .next_element()?
                        .ok_or_else(|| DeError::invalid_length(1, &self))?;

                    if val.is_string() {
                        channel = match Channel::from_str(val.as_str().unwrap()) {
                            Ok(c) => c,
                            Err(e) => {
                                return Err(DeError::custom(format!(
                                    "invalid value for channel, input was {} -> {}",
                                    val.as_str().unwrap(),
                                    e
                                )))
                            }
                        };
                        break;
                    } else if val.is_object() || val.is_array() {
                        messages.push(val);
                    } else {
                        return match val {
                            Value::Null => Err(DeError::custom(
                                "unexpected value. Expected channel name or json object got null",
                            )),
                            Value::Bool(_) => Err(DeError::custom(
                                "unexpected value. Expected channel name or json object got bool",
                            )),
                            Value::Number(_) => Err(DeError::custom(
                                "unexpected value. Expected channel name or json object got number",
                            )),
                            Value::String(_) => Err(DeError::custom(
                                "unexpected value. Expected channel name or json object got string",
                            )),
                            Value::Array(_) => Err(DeError::custom(
                                "unexpected value. Expected channel name or json object got array",
                            )),
                            Value::Object(_) => Err(DeError::custom(
                                "unexpected value. Expected channel name or json object got object",
                            )),
                        };
                    }
                }
                if messages.is_empty() {
                    return Err(DeError::custom("no data contained in message"));
                }
                let message: Value;
                if messages.len() == 1 {
                    message = messages.remove(0);
                } else {
                    message = Value::Array(messages)
                }

                let message = match ChannelMessage::new(&channel, message) {
                    Ok(m) => m,
                    Err(e) => {
                        return Err(DeError::custom(format!(
                            "message inner object cannot be deserialized as {} -> {}",
                            channel, e
                        )))
                    }
                };

                //TODO may need to do some magic to determine if there is a pair here...
                let pair: String = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(3, &self))?;

                let sequence = seq.next_element::<Sequence>().ok().unwrap_or(None);
                Ok(ChannelMessageWrapper {
                    id,
                    message,
                    channel,
                    pair: Some(pair),
                    sequence,
                })
            }
        }

        deserializer.deserialize_seq(ChannelMessageWrapperVisitor {})
    }
}

#[cfg(test)]
mod wrapper_message_tests {
    use super::*;
    use crate::test_utils;
    use anyhow::bail;
    use pretty_assertions::assert_eq;
    use pretty_assertions::assert_str_eq;

    #[test]
    fn deserialize_channel_message() -> anyhow::Result<()> {
        let j = test_utils::load_test_json("public/ticker")?;
        let channel_message: ChannelMessageWrapper = serde_json::from_str(&j)?;
        assert_str_eq!(
            channel_message.pair().as_ref().expect("didnt' have a pair"),
            "XBT/USD"
        );
        let ChannelMessage::Ticker(ticker) = channel_message.message() else {
            bail!("expected ticker type");
        };
        assert_eq!(*ticker.ask().whole_lot_volume(), 1);

        let j = test_utils::load_test_json("public/ohlc-5")?;
        let channel_message: ChannelMessageWrapper = serde_json::from_str(&j)?;
        assert_str_eq!(
            channel_message.pair().as_ref().expect("didnt' have a pair"),
            "XBT/USD"
        );
        let ChannelMessage::OHLC(ohlc) = channel_message.message() else {
            bail!("expected ohlc type");
        };
        assert_eq!(*ohlc.count(), 2);
        match channel_message.channel() {
            Channel::OHLC(interval) => {
                assert!(matches!(interval, general_messages::Interval::FiveMin))
            }
            _ => panic!("invalid interval. expected 5"),
        }
        Ok(())
    }

    #[test]
    fn deserialize_any_message() -> anyhow::Result<()> {
        let j = test_utils::load_test_json("public/ticker")?;
        let channel_message: Message = serde_json::from_str(&j)?;
        match channel_message {
            Message::ChannelMessage(_) => {}
            Message::Event(_) => bail!("expected channel message not event"),
        }

        let j = test_utils::load_test_json("general/ping")?;
        let channel_message: Message = serde_json::from_str(&j)?;
        match channel_message {
            Message::ChannelMessage(_) => bail!("expected event got channel message"),
            Message::Event(_) => {}
        }
        Ok(())
    }

    #[test]
    fn multiple_objects_in_message() -> anyhow::Result<()> {
        let j = test_utils::load_test_json("public/ticker")?;
        let channel_message: Message = serde_json::from_str(&j)?;
        match channel_message {
            Message::ChannelMessage(_) => {}
            Message::Event(_) => bail!("expected channel message not event"),
        }

        let j = test_utils::load_test_json("general/ping")?;
        let channel_message: Message = serde_json::from_str(&j)?;
        match channel_message {
            Message::ChannelMessage(_) => bail!("expected event got channel message"),
            Message::Event(_) => {}
        }
        Ok(())
    }
}

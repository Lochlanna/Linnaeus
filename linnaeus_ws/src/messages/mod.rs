pub mod general_messages;
pub mod public_messages;

use display_json::{DebugAsJson, DisplayAsJsonPretty};
use serde::de::Error as DeError;
use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::Formatter;
use std::str::FromStr;
use derive_getters::Getters;
use strum::Display as DisplayEnum;

use general_messages::*;
use public_messages::*;


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

#[derive(Serialize, Deserialize, Debug, Clone, DisplayEnum)]
#[serde(rename_all = "camelCase")]
pub enum Channel {
    Ticker,
    OHLC(Interval),
    Trade,
    Spread,
    Book(Depth)
}

impl FromStr for Channel {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("input empty")
        }
        let mut split = s.splitn(2, '-');
        let Some(name) = split.next() else {
            return Err("no valid strings")
        };

        match name {
            "ticker" => return Ok(Channel::Ticker),
            "trade" => return Ok(Channel::Trade),
            "spread" => return Ok(Channel::Spread),
            _ => {}
        }

        let Some(value) = split.next() else {
            return Err("unknown channel")
        };

        match name {
            "ohlc" => {
                let Ok(interval) = serde_json::from_str::<Interval>(value) else {
                    return Err("invalid value for ohlc interval")
                };
                Ok(Channel::OHLC(interval))
            }
            "book" => {
                let Ok(depth) = serde_json::from_str::<Depth>(value) else {
                    return Err("invalid value for book depth")
                };
                Ok(Channel::Book(depth))
            }
            _ => Err("unknown channel")
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChannelMessage {
    Ticker(Ticker),
    OHLC(OHLC),
    Trade(Trades),
    Spread(Spreads),
    Book(Book)
}

impl ChannelMessage {
    fn new(channel: &Channel, data: serde_json::Value) -> Result<ChannelMessage, serde_json::Error> {
        let channel_message = match channel {
            Channel::Ticker => ChannelMessage::Ticker(serde_json::from_value(data)?),
            Channel::OHLC(_) => ChannelMessage::OHLC(serde_json::from_value(data)?),
            Channel::Trade => ChannelMessage::Trade(serde_json::from_value(data)?),
            Channel::Spread => ChannelMessage::Spread(serde_json::from_value(data)?),
            Channel::Book(_) => ChannelMessage::Book(serde_json::from_value(data)?),
        };
        Ok(channel_message)
    }
}

#[derive(Serialize, Debug, Clone, Getters)]
pub struct ChannelMessageWrapper {
    id: i64,
    message: ChannelMessage,
    channel: Channel,
    pair: String,
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
                formatter.write_str("[i32, ChannelMessage, String, String]")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: SeqAccess<'de>,
            {
                let id: i64 = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(0, &self))?;
                let message: serde_json::Value = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(1, &self))?;
                let channel: &str = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(2, &self))?;
                let pair: String = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(3, &self))?;
                let channel= match Channel::from_str(channel) {
                    Ok(c) => c,
                    Err(e) => return Err(DeError::custom(format!("invalid value for channel, input was {} -> {}", channel, e)))
                };
                let message = match ChannelMessage::new(&channel, message) {
                    Ok(m) => m,
                    Err(e) => return Err(DeError::custom(format!("message inner object cannot be deserialized as {} -> {}", channel, e)))
                };
                Ok(ChannelMessageWrapper {
                    id,
                    message,
                    channel,
                    pair,
                })
            }
        }

        deserializer.deserialize_seq(ChannelMessageWrapperVisitor {})
    }
}
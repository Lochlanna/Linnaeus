use display_json::{DebugAsJson, DisplayAsJsonPretty};
use serde::de::Error as DeError;
use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::Formatter;
use std::str::FromStr;
use strum::Display as DisplayEnum;

mod general_messages;
mod public_messages;

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
    Ping(general_messages::Ping),
    Pong(general_messages::Pong),
    Heartbeat,
    SystemStatus(general_messages::SystemStatus),
    Subscribe(general_messages::Subscribe),
    Unsubscribe(general_messages::UnSubscribe),
    SubscriptionStatus(general_messages::SubscriptionStatus),
}

#[derive(Serialize, Deserialize, Debug, Clone, DisplayEnum)]
#[serde(rename_all = "camelCase")]
pub enum Channel {
    Ticker,
    OHLC(general_messages::Interval),
    Trade,
    Spread,
    Book(general_messages::Depth)
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
                let Ok(interval) = serde_json::from_str::<general_messages::Interval>(value) else {
                    return Err("invalid value for ohlc interval")
                };
                Ok(Channel::OHLC(interval))
            }
            "book" => {
                let Ok(depth) = serde_json::from_str::<general_messages::Depth>(value) else {
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
    Ticker(public_messages::Ticker),
    OHLC(public_messages::OHLC),
    Trade(public_messages::Trades),
    Spread(public_messages::Spreads),
    Book(public_messages::Book)
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

#[derive(Serialize, Debug, Clone)]
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

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use anyhow::bail;
    use super::*;
    use pretty_assertions::assert_eq;
    use pretty_assertions::assert_str_eq;

    fn load_test_json(name: &str) -> std::io::Result<String> {
        std::fs::read_to_string(format!("test_json/{}.json", name))
    }

    #[test]
    fn deserialize_channel_message() -> anyhow::Result<()> {
        let j = load_test_json("ticker")?;
        let channel_message: ChannelMessageWrapper = serde_json::from_str(&j)?;
        assert_str_eq!(channel_message.pair, "XBT/USD");
        let ChannelMessage::Ticker(ticker) = channel_message.message else {
            bail!("expected ticker type");
        };
        assert_eq!(*ticker.ask().whole_lot_volume(), 1);

        let j = load_test_json("ohlc-5")?;
        let channel_message: ChannelMessageWrapper = serde_json::from_str(&j)?;
        assert_str_eq!(channel_message.pair, "XBT/USD");
        let ChannelMessage::OHLC(ohlc) = channel_message.message else {
            bail!("expected ohlc type");
        };
        assert_eq!(*ohlc.count(), 2);
        match channel_message.channel {
            Channel::OHLC(interval) => assert!(matches!(interval, general_messages::Interval::FiveMin)),
            _ => panic!("invalid interval. expected 5")
        }
        Ok(())
    }

    #[test]
    fn deserialize_any_message() -> anyhow::Result<()> {
        let j = load_test_json("ticker")?;
        let channel_message: Message = serde_json::from_str(&j)?;
        match channel_message {
            Message::ChannelMessage(_) => {},
            Message::Event(_) => bail!("expected channel message not event")
        }

        let j = load_test_json("ping")?;
        let channel_message: Message = serde_json::from_str(&j)?;
        match channel_message {
            Message::ChannelMessage(_) => bail!("expected event got channel message"),
            Message::Event(_) => { }
        }
        Ok(())
    }
}

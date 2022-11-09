use display_json::{DebugAsJson, DisplayAsJsonPretty};
use serde::de::Error as DeError;
use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::Formatter;
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
    #[serde(rename = "ohlc-1")]
    OHLCOneMin,
    #[serde(rename = "ohlc-5")]
    OHLCFiveMin,
    #[serde(rename = "ohlc-15")]
    OHLCFifteenMin,
    #[serde(rename = "ohlc-30")]
    OHLCThirtyMin,
    #[serde(rename = "ohlc-60")]
    OHLCOneHour,
    #[serde(rename = "ohlc-240")]
    OHLCFourHour,
    #[serde(rename = "ohlc-1440")]
    OHLCOneDay,
    #[serde(rename = "ohlc-10080")]
    OHLCOneWeek,
    #[serde(rename = "ohlc-21600")]
    OHLCFifteenDay,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChannelMessage {
    Ticker(public_messages::Ticker),
    OHLC(public_messages::OHLC),
}

impl ChannelMessage {
    fn new(channel: &Channel, data: serde_json::Value) -> Result<ChannelMessage, serde_json::Error> {
        let channel_message = match channel {
            Channel::Ticker => {
                let ticker: public_messages::Ticker = serde_json::from_value(data)?;
                ChannelMessage::Ticker(ticker)
            }
            Channel::OHLCOneMin |
            Channel::OHLCFiveMin |
            Channel::OHLCFifteenMin |
            Channel::OHLCThirtyMin |
            Channel::OHLCOneHour |
            Channel::OHLCFourHour |
            Channel::OHLCOneDay |
            Channel::OHLCOneWeek |
            Channel::OHLCFifteenDay => {
                let ohlc: public_messages::OHLC = serde_json::from_value(data)?;
                ChannelMessage::OHLC(ohlc)
            }
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
                let channel: Channel = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(2, &self))?;
                let pair: String = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(3, &self))?;
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
            Message::Event(e) => { }
        }
        Ok(())
    }
}

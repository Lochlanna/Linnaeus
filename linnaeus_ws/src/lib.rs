use display_json::{DebugAsJson, DisplayAsJsonPretty};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

mod general_messages;
mod public_messages;

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Clone)]
#[serde(untagged)]
pub enum Message {
    ChannelMessage,
    Event,
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Clone)]
#[serde(tag = "event")]
#[serde(rename_all = "camelCase")]
pub enum Event {
    Ping,
    Pong,
    Heartbeat,
    SystemStatus,
    Subscribe,
    Unsubscribe,
    SubscriptionStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChannelNames {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChannelMessage {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelMessageWrapper {
    channel_id: i64,
    data: serde_json::value::Value,
    channel_name: ChannelNames,
    pair: String,
}

impl ChannelMessageWrapper {
    pub fn to_inner(self) -> Result<ChannelMessage, serde_json::Error> {
        todo!()
    }
    /// Could be more memory efficient but you should check it first
    pub fn to_inner_unchecked<T: DeserializeOwned>(self) -> Result<T, serde_json::Error> {
        serde_json::from_value(self.data)
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

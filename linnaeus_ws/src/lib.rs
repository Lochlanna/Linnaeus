#[cfg(test)]
mod test_utils;
pub mod messages;


use std::collections::BTreeMap;
use display_json::{DebugAsJson, DisplayAsJsonPretty};
use serde::{Serialize, Deserialize};
use tokio::sync::broadcast;


#[derive(Debug, PartialOrd, PartialEq, Ord, Eq)]
struct SubscriptionKey {
    channel: messages::Channel,
    pair: Option<messages::Pair>
}

impl SubscriptionKey {
    fn new(channel: messages::Channel, pair: messages::Pair) -> Self {
        SubscriptionKey {
            channel,
            pair: Some(pair)
        }
    }

    fn without_pair(channel: messages::Channel) -> Self {
        SubscriptionKey {
            channel,
            pair: None
        }
    }
}


/// This struct splits messages which need to be sorted by currency pair and ones that dont
#[derive(Debug, Default)]
pub struct SubscriptionManager {
    currency_subscriptions: BTreeMap<messages::Pair, BTreeMap<messages::Channel, broadcast::Sender<messages::ChannelMessage>>>,
    other: BTreeMap<messages::Channel, broadcast::Sender<messages::ChannelMessage>>,
}


#[derive(DebugAsJson, DisplayAsJsonPretty, Serialize, Deserialize)]
pub struct LinnaeusWebsocket {
    websocket_url: url::Url,
    token: String,
    #[serde(skip)]
    subscriptions: BTreeMap<SubscriptionKey, broadcast::Sender<messages::ChannelMessage>>
}






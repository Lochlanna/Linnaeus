pub mod api;
#[cfg(test)]
mod test_helpers;

use std::sync::Arc;
use display_json::{DebugAsJson, DisplayAsJsonPretty};
use linnaeus_request::KrakenKeyPair;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};

pub use linnaeus_ws as ws;
use linnaeus_ws::error::LinnaeusWebsocketError;
use linnaeus_ws::LinnaeusWebsocket;

static KEY_ROTATION_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(DebugAsJson, DisplayAsJsonPretty, Serialize, Deserialize)]
pub struct Linnaeus {
    #[serde(skip)]
    client: Client,
    keys: Vec<KrakenKeyPair>,
    base_url: String,
    ws_url: String,
    #[serde(skip)]
    ws_client: Option<Arc<LinnaeusWebsocket>>
}

impl Linnaeus {
    pub fn new(keys: Vec<KrakenKeyPair>, base_url: &str, ws_url: &str) -> Self {
        //TODO check that keys isn't empty
        Self {
            client: Client::new(),
            keys,
            base_url: String::from(base_url),
            ws_url: String::from(ws_url),
            ws_client: None,
        }
    }

    pub async fn get_websocket_client(&mut self) -> Result<Arc<LinnaeusWebsocket>, LinnaeusWebsocketError> {
        match &self.ws_client {
            None => {
                let client = LinnaeusWebsocket::new(&self.ws_url).await;
                match &client {
                    Ok(client) => self.ws_client = Some(client.clone()),
                    _ => {}
                }
                client
            }
            Some(client) => Ok(client.clone())
        }
    }
}

impl linnaeus_request::RequestClient for Linnaeus {
    fn get_client(&self) -> &Client {
        &self.client
    }

    fn get_keys(&self) -> &KrakenKeyPair {
        let index = KEY_ROTATION_COUNTER.fetch_add(1, Ordering::SeqCst);
        let key = self
            .keys
            .get(index % self.keys.len())
            .expect("key list changed. This shouldn't be possible");
        key
    }

    fn get_base_url(&self) -> &str {
        &self.base_url
    }
}

impl linnaeus_request::RequestHelpers for Linnaeus {}

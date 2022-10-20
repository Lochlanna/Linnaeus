pub mod api;
#[cfg(test)]
mod test_helpers;

use display_json::{DebugAsJson, DisplayAsJsonPretty};
use linnaeus_request::KrakenKeyPair;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};
use thiserror::Error;

static KEY_ROTATION_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(DebugAsJson, DisplayAsJsonPretty, Serialize, Deserialize)]
pub struct Linnaeus {
    #[serde(skip)]
    client: Client,
    keys: Vec<KrakenKeyPair>,
    base_url: String,
    ws_url: String,
}

impl Linnaeus {
    pub fn new(keys: Vec<KrakenKeyPair>, base_url: &str, ws_url: &str) -> Self {
        //TODO check that keys isn't empty
        Self {
            client: Client::new(),
            keys,
            base_url: String::from(base_url),
            ws_url: String::from(ws_url),
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

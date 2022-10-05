pub mod api;
#[cfg(test)]
mod test_helpers;

use display_json::{DebugAsJson, DisplayAsJsonPretty};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(DebugAsJson, DisplayAsJsonPretty, Serialize, Deserialize)]
pub struct Linnaeus {
    #[serde(skip)]
    client: Client,
    api_key: String,
    api_secret: String,
    passphrase: String,
    base_url: String,
    ws_url: String,
}

impl Linnaeus {
    pub fn new(
        api_key: &str,
        api_secret: &str,
        passphrase: &str,
        base_url: &str,
        ws_url: &str,
    ) -> Self {
        Self {
            client: Client::new(),
            api_key: String::from(api_key),
            api_secret: String::from(api_secret),
            passphrase: String::from(passphrase),
            base_url: String::from(base_url),
            ws_url: String::from(ws_url),
        }
    }
}

impl linnaeus_request::RequestClient for Linnaeus {
    fn get_client(&self) -> &Client {
        &self.client
    }

    fn get_api_key(&self) -> &str {
        &self.api_key
    }

    fn get_api_secret(&self) -> &str {
        &self.api_secret
    }

    fn get_base_url(&self) -> &str {
        &self.base_url
    }

    fn get_passphrase(&self) -> &str {
        &self.passphrase
    }
}

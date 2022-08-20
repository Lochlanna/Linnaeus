pub mod error;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use serde_encrypt::serialize::impls::BincodeSerializer;
use serde_encrypt::shared_key::SharedKey;
use serde_encrypt::traits::SerdeEncryptSharedKey;
use std::fs;
use reqwest::header::{HeaderMap, HeaderName};

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    apiKey: String,
    last_nonce: Option<u128>,
    //TODO support OTP
}

impl SerdeEncryptSharedKey for Auth {
    type S = BincodeSerializer<Self>;
}

impl Auth {
    fn apiKey(&self) -> &str {
        self.apiKey.as_str()
    }
    fn next_nonce(&mut self) -> u128 {
        match &mut self.last_nonce {
            None => SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis(),
            Some(nonce) => {
                *nonce += 1;
                *nonce
            }
        }
    }
    fn new(apikey: &str) -> Self {
        Auth {
            apiKey: apikey.to_string(),
            last_nonce:None,
        }
    }
    fn new_with_nonce(apikey: &str, nonce:u128) -> Self {
        Auth {
            apiKey: apikey.to_string(),
            last_nonce:Some(nonce),
        }
    }
    fn save_to_disk(&self, path: &std::path::Path, key: &SharedKey) -> anyhow::Result<()> {
        let encrypted = self.encrypt(key)?;
        let encrypted_serialized: Vec<u8> = encrypted.serialize();
        fs::write(path, encrypted_serialized)?;
        Ok(())
    }
}

trait ToKrakenRequest {
    fn as_request(&self, auth: &mut Auth, client: &reqwest::Client) -> reqwest::RequestBuilder;
    fn get_kraken_signature(&self, auth: &mut Auth) {
        reqwest::
    }
}
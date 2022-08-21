pub mod error;
pub mod market;

use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use serde_encrypt::serialize::impls::BincodeSerializer;
use serde_encrypt::shared_key::SharedKey;
use serde_encrypt::traits::SerdeEncryptSharedKey;
use std::fs;
use anyhow::bail;
use sha2::{Sha256, Sha512, Digest};
use hmac::{Hmac, Mac};
use reqwest::Client;
use crate::KrakenError;

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
    pub fn apiKey(&self) -> &str {
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
    pub fn new(apikey: &str) -> Self {
        Auth {
            apiKey: apikey.to_string(),
            last_nonce:None,
        }
    }
    pub fn new_with_nonce(apikey: &str, nonce:u128) -> Self {
        Auth {
            apiKey: apikey.to_string(),
            last_nonce:Some(nonce),
        }
    }
    pub fn save_to_disk(&self, path: &std::path::Path, key: &SharedKey) -> anyhow::Result<()> {
        let encrypted = self.encrypt(key)?;
        let encrypted_serialized: Vec<u8> = encrypted.serialize();
        fs::write(path, encrypted_serialized)?;
        Ok(())
    }
}

pub trait KrakenType {
    fn kraken_path() -> &'static str;
    fn http_type() -> http::Method;
    fn authenticated_request() -> bool;
}

#[derive(Debug, Serialize, Clone)]
struct KrakenRequest<T: KrakenType + Serialize + Clone> {
    #[serde(flatten)]
    inner: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    nonce: Option<u128>
}

impl<T: KrakenType + Serialize + Clone> KrakenRequest<T> {
    fn new(inner: T, nonce: u128, auth: &mut Auth, client: &Client) -> anyhow::Result<reqwest::RequestBuilder> {
        let path = T::kraken_path().to_string();

        let (mut data, nonce) = if T::authenticated_request() {
            let nonce = Some(auth.next_nonce());
            let data = KrakenRequest::new_data(inner,nonce);
            (data, nonce)
        } else {
            (KrakenRequest::new_data(inner,None), None)
        };

        let mut req = client.post(&path).form(&data);

        if T::authenticated_request() {
            let nonce = match nonce {
                None => bail!("Couldn't get nonce on authenticated request"),
                Some(n) => n
            };
            let decoded_secret = base64::decode(auth.apiKey.as_bytes())?;

            let mut sha = Sha256::new();
            sha.update(nonce.to_string());
            sha.update(serde_urlencoded::to_string(data.clone())?.as_str());
            let sha_sum = sha.finalize().as_slice().to_vec();

            type HmacSha512 = Hmac<Sha512>;
            let mut hmac = HmacSha512::new_from_slice(&decoded_secret)?;
            hmac.update(path.as_bytes());
            hmac.update(&sha_sum);
            let mac_sum = hmac.finalize().into_bytes();

            let encoded = base64::encode(mac_sum);

            req = req.header("API-Key", auth.apiKey()).header("API-Sign", encoded);
        }

        Ok(req)
    }
    fn new_data(inner: T, nonce: Option<u128>) -> Self {
        Self { inner, nonce }
    }
}

#[derive(Debug, Deserialize)]
pub enum KrakenResponse<T> {
    error(Vec<KrakenError>),
    result(T)
}

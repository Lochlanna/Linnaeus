pub mod error;
pub mod market;
pub mod enums;

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
use serde::de::DeserializeOwned;
use crate::KrakenError;
use async_trait::async_trait;

const KRAKEN_BASE_URL: &str = "https://api.kraken.com/0";

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    api_key: String,
    last_nonce: Option<u128>,
    //TODO support OTP
}

impl SerdeEncryptSharedKey for Auth {
    type S = BincodeSerializer<Self>;
}

impl Auth {
    pub fn api_key(&self) -> &str {
        self.api_key.as_str()
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
            api_key: apikey.to_string(),
            last_nonce:None,
        }
    }
    pub fn new_with_nonce(apikey: &str, nonce:u128) -> Self {
        Auth {
            api_key: apikey.to_string(),
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
struct KrakenRequestContainer<'a, T: KrakenType + Serialize> {
    #[serde(flatten)]
    inner: &'a T,
    #[serde(skip_serializing_if = "Option::is_none")]
    nonce: Option<u128>
}

#[derive(Debug, Deserialize)]
pub struct KrakenResponseContainer<T> {
    error: Vec<KrakenError>,
    result: T
}

#[async_trait]
pub trait KrakenRequest {
    type R: DeserializeOwned + Send;
    async fn new_request(&self, auth: &mut Auth, client: &Client) -> Result<Self::R, anyhow::Error> where Self: KrakenType + Serialize + Send + Sized {
        let path = KRAKEN_BASE_URL.to_string() + Self::kraken_path();
        let (data, nonce) = if Self::authenticated_request() {
            let nonce = Some(auth.next_nonce());
            let data = KrakenRequestContainer::new(self, nonce);
            (data, nonce)
        } else {
            (KrakenRequestContainer::new(self, None), None)
        };

        let mut req = client.request(Self::http_type(), &path).form(&data);

        if Self::authenticated_request() {
            let nonce = match nonce {
                None => bail!("Couldn't get nonce on authenticated request"),
                Some(n) => n
            };
            let decoded_secret = base64::decode(auth.api_key.as_bytes())?;

            let mut sha = Sha256::new();
            sha.update(nonce.to_string());
            sha.update(serde_urlencoded::to_string(data)?.as_str());
            let sha_sum = sha.finalize().as_slice().to_vec();

            let mut hmac = Hmac::<Sha512>::new_from_slice(&decoded_secret)?;
            hmac.update(path.as_bytes());
            hmac.update(&sha_sum);
            let mac_sum = hmac.finalize().into_bytes();

            let encoded = base64::encode(mac_sum);

            req = req.header("API-Key", auth.api_key()).header("API-Sign", encoded);
        }

        let resp: KrakenResponseContainer<Self::R> = req.send().await?.json().await?;

        if resp.error.is_empty() {
            Ok(resp.result)
        } else {
            //TODO it's possible that we could have more than one error here...
            Err(anyhow::Error::new(resp.error[0].clone()))
        }
    }
}

impl<'a, T: KrakenType + Serialize> KrakenRequestContainer<'a, T> {
    fn new(inner: &'a T, nonce: Option<u128>) -> Self {
        Self { inner, nonce }
    }
}



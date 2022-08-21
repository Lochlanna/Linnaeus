pub mod error;
pub mod market;

use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use serde_encrypt::serialize::impls::BincodeSerializer;
use serde_encrypt::shared_key::SharedKey;
use serde_encrypt::traits::SerdeEncryptSharedKey;
use std::fs;
use sha2::{Sha256, Sha512, Digest};
use hmac::{Hmac, Mac};
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

pub trait KrakenPath {
    fn kraken_path() -> &'static str;
}

pub trait AuthenticatedKrakenRequest {
    fn get_reqwest(&self, auth: &mut Auth, client: &reqwest::Client) -> anyhow::Result<reqwest::RequestBuilder> where Self: crate::utils::ToBTree + KrakenPath {
        let path = Self::kraken_path().to_string();
        let nonce = auth.next_nonce();
        let mut data = self.to_b_tree();
        data.insert("nonce".to_string(), crate::utils::PrimitiveValue::from(nonce.to_string()));
        let decoded_secret = base64::decode(auth.apiKey.as_bytes()).expect("TODO");

        let mut sha = Sha256::new();
        sha.update(nonce.to_string());
        sha.update(serde_urlencoded::to_string(data.clone())?.as_str());
        let sha_sum = sha.finalize().as_slice().to_vec();

        type HmacSha512 = Hmac<Sha512>;
        let mut hmac = HmacSha512::new_from_slice(&decoded_secret).expect("TODO");
        hmac.update(path.as_bytes());
        hmac.update(&sha_sum);
        let mac_sum = hmac.finalize().into_bytes();

        let encoded = base64::encode(mac_sum);

        let req = client.post(path).form(&data).header("API-Key", auth.apiKey()).header("API-Sign", encoded);

        Ok(req)
    }
}

pub trait PublicKrakenRequest {
    fn get_reqwest(&self, client: &reqwest::Client) -> anyhow::Result<reqwest::RequestBuilder> where Self: KrakenPath + Serialize {
        let path = Self::kraken_path().to_string();
        let req = client.post(path).form(self);
        Ok(req)
    }
}

#[derive(Debug, Deserialize)]
pub enum KrakenResponse<T> {
    error(Vec<KrakenError>),
    result(T)
}

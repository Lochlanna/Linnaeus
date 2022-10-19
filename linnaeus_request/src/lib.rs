pub mod error;

use std::fmt::Display;
use chrono::{TimeZone, Utc};
use display_json::{DebugAsJson, DisplayAsJsonPretty};
use error::KrakenErrors;
use hmac::{Hmac, Mac};
use log::{info, trace};
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Sha512, Digest};
use strum::Display;
use serde_with::{skip_serializing_none, serde_as};
use crate::error::{KrakenErrorMessage, RequestError};

#[derive(Serialize)]
struct Empty {}

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct KrakenRequest<T: serde::Serialize> {
    pub nonce: u64,
    #[serde(flatten)]
    pub payload: T
}

impl<T> Display for KrakenRequest<T> where T: Serialize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(kr_str) => f.write_str(&kr_str),
            Err(_) => Err(std::fmt::Error{})
        }
    }
}

#[derive(Serialize)]
struct Payload<'a, T: Serialize> {
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signature: Option<String>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<&'a T>,
}

#[derive(Debug, Display)]
pub enum EndpointSecurityType {
    None,
    Private,
}

impl EndpointSecurityType {
    #[allow(dead_code)]
    fn is_secure(&self) -> bool {
        matches!(self, EndpointSecurityType::Private)
    }
}

pub trait RequestClient {
    fn get_client(&self) -> &reqwest::Client;
    fn get_api_key(&self) -> &str;
    fn get_api_private_key(&self) -> &str;
    fn get_base_url(&self) -> &str;
    fn get_next_nonce(&self) -> u64 {
        let from = Utc.ymd(2022, 10, 17).and_hms(11, 11, 11);
         let diff = Utc::now() - from;
        diff.num_nanoseconds().expect("oh no...") as u64
    }
}

pub trait RequestHelpers: RequestClient {
    fn generate_signature<T: Serialize>(
        &self,
        data: &KrakenRequest<T>,
        path: &str,
    ) -> Result<String, error::SignatureGenerationError> {
        let form_data = serde_urlencoded::to_string(&data)?;

        let message = format!("{}{}", data.nonce, form_data);
        trace!("signature message is {}", message);
        //SHA256(nonce + POST data)
        let mut sha256_hasher = Sha256::new();
        sha256_hasher.update(message.as_bytes());
        let inner_message = sha256_hasher.finalize();

        let hmac_key = base64::decode(self.get_api_private_key())?;
        let mut mac = match Hmac::<Sha512>::new_from_slice(&hmac_key) {
            Ok(mac) => mac,
            Err(_) => {
                return Err(error::SignatureGenerationError::InvalidSecret);
            }
        };
        mac.update(path.as_bytes());
        mac.update(&inner_message[..]);
        let result = mac.finalize();
        let result = base64::encode(result.into_bytes());
        Ok(result)
    }

    fn generate_req_no_payload(
        &self,
        path: &str,
        method: http::Method,
        security_type: EndpointSecurityType,
    ) -> Result<RequestBuilder, error::RequestError> {
        self.internal_generate_req::<Empty, Empty>(path, method, security_type, None, None)
    }

    fn generate_req_with_body<T: Serialize>(
        &self,
        path: &str,
        method: http::Method,
        security_type: EndpointSecurityType,
        body: &T,
    ) -> Result<RequestBuilder, error::RequestError> {
        self.internal_generate_req::<T, Empty>(path, method, security_type, Some(body), None)
    }

    fn generate_req<T: Serialize, Q: Serialize>(
        &self,
        path: &str,
        method: http::Method,
        security_type: EndpointSecurityType,
        body: &T,
        query: &Q
    ) -> Result<RequestBuilder, error::RequestError> {
        self.internal_generate_req(path, method, security_type, Some(body), Some(query))
    }

    fn generate_req_with_query<Q: Serialize>(
        &self,
        path: &str,
        method: http::Method,
        security_type: EndpointSecurityType,
        query: &Q
    ) -> Result<RequestBuilder, error::RequestError> {
        self.internal_generate_req::<Empty, Q>(path, method, security_type, None, Some(query))
    }

    fn internal_generate_req<T: Serialize, Q: Serialize>(
        &self,
        path: &str,
        method: http::Method,
        security_type: EndpointSecurityType,
        data: Option<&T>,
        query: Option<&Q>,
    ) -> Result<RequestBuilder, error::RequestError> {
        let url = self.get_base_url().to_string() + path;
        let mut req = self
            .get_client()
            .request(method, &url)
            .header(http::header::USER_AGENT, "Linnaeus");

        if security_type.is_secure() {
            let payload_with_nonce = KrakenRequest {
                payload: data,
                nonce: self.get_next_nonce()
            };
            let signature = self.generate_signature(&payload_with_nonce, path)?;
            trace!("request signature is {}", signature);
            req = req
                .header("API-Key", self.get_api_key())
                .header("API-Sign", signature).form(&payload_with_nonce);
        } else if let Some(payload) = data {
            req = req.form(payload)
        }

        if let Some(query) = query {
            req = req.query(query)
        }

        trace!(
            "Generated request for {}: {:?} with security type {}",
            path,
            req,
            security_type
        );
        Ok(req)
    }
}

#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Clone)]
struct Response<O> {
    error: Vec<String>,
    result: Option<O>,
}

impl<O> TryFrom<Response<O>> for KrakenErrors {
    type Error = error::RequestError;

    fn try_from(value: Response<O>) -> Result<Self, Self::Error> {
        let mut errors:Vec<error::KrakenError> = Vec::with_capacity(value.error.len());
        for err in value.error {
            errors.push(err.as_str().try_into()?);
        }
        Ok(KrakenErrors {
            errors,
        })
    }
}

async fn deserialize_response<O>(resp: reqwest::Response) -> Result<O, error::RequestError>
where
    O: DeserializeOwned,
{
    if resp.status().is_success() {
        trace!("Got success response for request to {}", resp.url());
        let resp_bytes = resp.bytes().await?;
        let resp: Response<O> = match serde_json::from_slice(&resp_bytes) {
            Ok(resp) => resp,
            Err(e) => return Err(error::RequestError::DeserializationError(e, String::from_utf8(resp_bytes.into()).unwrap_or("Non UTF-8 Json".into())))
        };
        match resp.result {
            Some(result) => Ok(result),
            None => {
                let errors: KrakenErrors = resp.try_into()?;
                Err(errors.into())
            }
        }
    } else {
        let status_code = resp.status().as_u16();
        let resp_body = resp.text().await?;
        Err(error::RequestError::Other(format!(
            "Got non 200 status code ({}) on request with body -> {}",
            status_code, resp_body
        )))
    }
}

#[inline]
async fn execute_request<O>(linnaeus_client: &(impl RequestClient + RequestHelpers), req: RequestBuilder) -> Result<O, error::RequestError> where O: DeserializeOwned {
    let req = req.build()?;
    let resp = linnaeus_client.get_client().execute(req).await?;
    deserialize_response(resp).await
}

pub async fn do_request_with_body<I, O>(
    linnaeus_client: &(impl RequestClient + RequestHelpers),
    url: &str,
    method: http::Method,
    security_type: EndpointSecurityType,
    body: &I,
) -> Result<O, error::RequestError>
where
    I: Serialize,
    O: DeserializeOwned,
{
    let req = linnaeus_client.generate_req_with_body(url, method, security_type, body)?;
    execute_request(linnaeus_client, req).await
}

pub async fn do_request_with_query<Q, O>(
    linnaeus_client: &(impl RequestClient + RequestHelpers),
    url: &str,
    method: http::Method,
    security_type: EndpointSecurityType,
    query: &Q,
) -> Result<O, error::RequestError>
    where
        Q: Serialize,
        O: DeserializeOwned,
{
    let req = linnaeus_client.generate_req_with_query(url, method, security_type, query)?;
    execute_request(linnaeus_client, req).await
}

pub async fn do_request<I, Q, O>(
    linnaeus_client: &(impl RequestClient + RequestHelpers),
    url: &str,
    method: http::Method,
    security_type: EndpointSecurityType,
    body: &I,
    query: &Q
) -> Result<O, error::RequestError>
    where
        I: Serialize,
        Q: Serialize,
        O: DeserializeOwned,
{
    let req = linnaeus_client.generate_req(url, method, security_type, body, query)?;
    execute_request(linnaeus_client, req).await
}

pub async fn do_request_no_params<O>(
    linnaeus_client: &(impl RequestClient + RequestHelpers),
    url: &str,
    method: http::Method,
    security_type: EndpointSecurityType,
) -> Result<O, error::RequestError>
where
    O: DeserializeOwned,
{
    let req = linnaeus_client.generate_req_no_payload(url, method, security_type)?;
    execute_request(linnaeus_client, req).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use reqwest::Client;
    use serde::Serialize;

    struct MockClient {
        client: Client,
    }

    impl MockClient {
        fn new() -> Self {
            Self {
                client: Client::new(),
            }
        }
    }

    impl RequestClient for MockClient {
        fn get_client(&self) -> &Client {
            &self.client
        }

        fn get_api_key(&self) -> &str {
            "21b33a403f265ba5c8382b3a8bafd254" // Not a real API key
        }

        fn get_api_private_key(&self) -> &str {
            "kQH5HW/8p1uGOVjbgWA7FunAmGO8lsSUXNsu3eow76sz84Q18fWxnyRzBHCd3pd5nE9qa99HAZtuZuj6F1huXg=="
            // Not a real secret
        }

        fn get_base_url(&self) -> &str {
            "https://base_url.com/"
        }
    }

    impl RequestHelpers for MockClient {}

    #[derive(Serialize)]
    struct TestStructure {
        a:u8,
        b:String,
    }

    #[derive(Serialize)]
    struct SignTestStructure {
        #[serde(rename = "ordertype")]
        order_type: String,
        pair:String,
        price:u32,
        #[serde(rename = "type")]
        transaction_type:String,
        volume:f32
    }

    impl Default for SignTestStructure {
        fn default() -> Self {
            Self {
                order_type: "limit".to_string(),
                pair: "XBTUSD".to_string(),
                price: 37500,
                transaction_type: "buy".to_string(),
                volume: 1.25
            }
        }
    }

    #[test]
    fn test_signature_generation() -> Result<()> {
        let mock = MockClient::new();

        let test_input = KrakenRequest {
            payload: SignTestStructure::default(),
            nonce: 1616492376594
        };
        let url_encoded = serde_urlencoded::to_string(&test_input)?;
        assert_str_eq!("nonce=1616492376594&ordertype=limit&pair=XBTUSD&price=37500&type=buy&volume=1.25", url_encoded);
        println!("form:{}", url_encoded);
        let res = mock.generate_signature(
            &test_input,
            "/0/private/AddOrder"
        )?;
        assert_eq!(res, "4/dpxb3iT4tp/ZCVEwSnEsLxx0bqyhLpdfOpc6fn7OR8+UClSV5n9E6aSS8MPtnRfp32bAb0nmbRn6H8ndwLUQ==");
        Ok(())
    }

    fn get_body(req: &reqwest::Request) -> String {
        assert!(req.body().is_some());
        let body_bytes = req
            .body()
            .unwrap()
            .as_bytes()
            .expect("Couldn't get the request body as bytes");
        let body_str: String =
            String::from_utf8(Vec::from(body_bytes)).expect("Couldn't encode body as utf-8");
        body_str
    }

    #[test]
    fn test_gen_no_sec() -> Result<()> {
        let mock = MockClient::new();
        let test_struct = TestStructure {
            a: 3,
            b: "hello".to_string(),
        };
        let res = mock
            .generate_req_with_body(
                "somepath",
                http::Method::GET,
                EndpointSecurityType::None,
                &test_struct,
            )?
            .build()?;

        assert!(!res.headers().contains_key("API-Key"));
        assert!(!res.headers().contains_key("API-Sign"));

        let body = get_body(&res);
        let body_map: std::collections::HashMap<String, String> = serde_urlencoded::from_str(&body)?;
        assert_eq!(body_map.len(), 2);
        assert!(body_map.contains_key("a"));
        assert!(body_map.contains_key("b"));
        assert!(!body_map.contains_key("nonce"));
        assert_str_eq!(body_map["a"], "3");
        assert_str_eq!(body_map["b"], "hello");

        Ok(())
    }

    #[test]
    fn test_gen_with_sec() -> Result<()> {
        let mock = MockClient::new();
        let test_struct = TestStructure {
            a: 3,
            b: "hello".to_string(),
        };
        let res = mock
            .generate_req_with_body(
                "somepath",
                http::Method::GET,
                EndpointSecurityType::Private,
                &test_struct,
            )?
            .build()?;

        assert!(res.headers().contains_key("API-key"));
        assert!(res.headers().contains_key("API-Sign"));

        assert_str_eq!(
            res.headers()
                .get("API-Key")
                .expect("Couldn't get access key").to_str()?,
            mock.get_api_key()
        );

        let body = get_body(&res);
        let body_map: std::collections::HashMap<String, String> = serde_urlencoded::from_str(&body)?;
        assert_eq!(body_map.len(),3);
        assert!(body_map.contains_key("a"));
        assert!(body_map.contains_key("b"));
        assert!(body_map.contains_key("nonce"));
        assert_str_eq!(body_map["a"], "3");
        assert_str_eq!(body_map["b"], "hello");

        Ok(())
    }
}

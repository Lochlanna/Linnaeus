pub mod error;

use display_json::{DebugAsJson, DisplayAsJsonPretty};
use error::KrakenErrors;
use hmac::{Hmac, Mac};
use log::trace;
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use strum::Display;

#[inline(always)]
fn nanos_to_seconds(nanos: i64) -> f64 {
    (nanos as f64) / 1e-9
}

#[derive(Serialize)]
struct Empty {}

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
    fn get_passphrase(&self) -> &str;
    fn get_base_url(&self) -> &str;
}

pub trait RequestHelpers: RequestClient {
    fn generate_signature<T: Serialize>(
        &self,
        data: Option<&T>,
        method: &http::Method,
        path: &str,
        timestamp: f64,
    ) -> Result<String, error::SignatureGenerationError> {
        let json_data = match data {
            Some(data) => serde_json::to_string(data)?,
            None => String::new(),
        };
        let message = format!("{}{}{}{}", timestamp, method, path, json_data);
        let hmac_key = base64::decode(self.get_api_private_key())?;
        let mut mac = match Hmac::<Sha256>::new_from_slice(&hmac_key) {
            Ok(mac) => mac,
            Err(_) => {
                return Err(error::SignatureGenerationError::InvalidSecret);
            }
        };
        mac.update(message.as_bytes());
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
            .request(method.clone(), &url)
            .header(http::header::USER_AGENT, "Linnaeus");

        if security_type.is_secure() {
            let timestamp = chrono::Utc::now();
            let timestamp_seconds = nanos_to_seconds(timestamp.timestamp_nanos());
            let signature = self.generate_signature(data, &method, &url, timestamp_seconds)?;
            req = req
                .header("CB-ACCESS-KEY", self.get_api_key())
                .header("CB-ACCESS-SIGN", signature)
                .header("CB-ACCESS-TIMESTAMP", timestamp_seconds.to_string())
                .header("CB-ACCESS-PASSPHRASE", self.get_passphrase());
        }

        if let Some(query) = query {
            req = req.query(query)
        }
        if let Some(payload) = data {
            req = req.form(payload)
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

impl<O> TryFrom<Response<O>> for error::KrakenErrors {
    type Error = error::RequestError;

    fn try_from(value: Response<O>) -> Result<Self, Self::Error> {
        let mut errors:Vec<error::KrakenError> = Vec::with_capacity(value.error.len());
        for err in value.error {
            errors.push(err.as_str().try_into()?);
        }
        Ok(error::KrakenErrors {
            errors,
        })
    }
}

async fn deserialize_response<O>(resp: reqwest::Response) -> Result<O, error::RequestError>
where
    O: DeserializeOwned,
{
    if resp.status().is_success() {
        let resp_body = resp.text().await?;
        let resp: Response<O> = match serde_json::from_str(&resp_body) {
            Ok(resp) => resp,
            Err(err) => {
                return Err(error::RequestError::DeserializationError(
                    err,
                    "Failed to deserialize success body".to_string(),
                ))
            }
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
    use chrono::{TimeZone, Utc};
    use pretty_assertions::assert_eq;
    use reqwest::Client;
    use serde::Serialize;

    struct MockClient {
        client: reqwest::Client,
    }

    impl MockClient {
        fn new() -> Self {
            Self {
                client: reqwest::Client::new(),
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
            "SbSF/fI5QLNBzl4tqgrFNmgolkgxixR0M1iw2XDWRtJV9lARYR5418hhvDEV3wwpmJ8zXNkok7oBHWjpasTXaA=="
            // Not a real secret
        }

        fn get_base_url(&self) -> &str {
            "https://base_url.com/"
        }

        fn get_passphrase(&self) -> &str {
            "lbhoaxk52vp" // Not a real passphrase
        }
    }

    impl RequestHelpers for MockClient {}
    #[derive(Serialize)]
    struct TestStructure {
        a: u8,
        b: String,
    }

    #[test]
    fn test_signature_generation() -> Result<()> {
        let mock = MockClient::new();

        let test_struct = TestStructure {
            a: 3,
            b: "hello".to_string(),
        };
        println!("json:{}", serde_json::to_string(&test_struct)?);
        let res = mock.generate_signature(
            Some(&test_struct),
            &http::method::Method::POST,
            "/test/path",
            12345.6,
        )?;
        assert_eq!(res, "JYoMt3NelQ/3/h7f9UpqTUAiHrYoyBrdkOS0+zuvTf8=");
        Ok(())
    }

    fn verify_body(req: &reqwest::Request, expecting: &str) {
        assert!(req.body().is_some());
        let body_bytes = req
            .body()
            .unwrap()
            .as_bytes()
            .expect("Couldn't get the request body as bytes");
        let body_str: String =
            String::from_utf8(Vec::from(body_bytes)).expect("Couldn't encode body as utf-8");
        assert_eq!(body_str, expecting);
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

        assert!(!res.headers().contains_key("CB-ACCESS-KEY"));
        assert!(!res.headers().contains_key("CB-ACCESS-SIGN"));
        assert!(!res.headers().contains_key("CB-ACCESS-TIMESTAMP"));
        assert!(!res.headers().contains_key("CB-ACCESS-PASSPHRASE"));

        verify_body(&res, "a=3&b=hello");

        Ok(())
    }

    #[inline(always)]
    fn seconds_to_nanos(seconds: f64) -> i64 {
        ((seconds as f64) * 1e-9) as i64
    }

    fn get_time_stamp_from_headers(req: &reqwest::Request) -> chrono::DateTime<Utc> {
        let ts_bytes = req
            .headers()
            .get("CB-ACCESS-TIMESTAMP")
            .expect("Couldn't get timestamp header")
            .as_bytes();
        let ts_str = String::from_utf8(Vec::from(ts_bytes)).expect("timestamp header wasnt' utf-8");

        let seconds: f64 = ts_str.parse().expect("couldn't parse timestamp as f64");
        let nanos = seconds_to_nanos(seconds);
        Utc.timestamp_nanos(nanos)
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

        assert!(res.headers().contains_key("CB-ACCESS-KEY"));
        assert!(res.headers().contains_key("CB-ACCESS-SIGN"));
        assert!(res.headers().contains_key("CB-ACCESS-TIMESTAMP"));
        assert!(res.headers().contains_key("CB-ACCESS-PASSPHRASE"));

        assert_eq!(
            res.headers()
                .get("CB-ACCESS-KEY")
                .expect("Couldn't get access key"),
            mock.get_api_key()
        );
        assert!(res.headers().get("CB-ACCESS-SIGN").is_some());

        let header_ts = get_time_stamp_from_headers(&res);
        let diff = chrono::Utc::now() - header_ts;
        assert!(diff < chrono::Duration::seconds(1));

        assert_eq!(
            res.headers()
                .get("CB-ACCESS-PASSPHRASE")
                .expect("Couldn't get passphrase"),
            mock.get_passphrase()
        );

        verify_body(&res, "a=3&b=hello");

        Ok(())
    }
}

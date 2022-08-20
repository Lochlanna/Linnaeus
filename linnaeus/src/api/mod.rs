use chrono::Utc;
use derive_getters::Getters;
use display_json::{DebugAsJson, DisplayAsJsonPretty};
use linnaeus_request::{
    do_request_no_params, error, EndpointSecurityType, RequestClient, RequestHelpers,
};
use serde::{Deserialize, Serialize};

use serde_with::serde_as;

pub mod market_data;
pub mod user_data;
pub mod user_funding;
pub mod user_staking;
pub mod user_trading;

#[serde_as]
#[derive(Serialize, Deserialize, DebugAsJson, DisplayAsJsonPretty, Getters, Clone)]
pub struct WebsocketToken {
    token: String,
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    #[serde(rename = "expires")]
    valid_lifetime: chrono::Duration,
    #[serde(default = "chrono::Utc::now")]
    created_at: chrono::DateTime<Utc>,
}

impl WebsocketToken {
    pub fn expiry_time(&self) -> chrono::DateTime<Utc> {
        let expires = self.created_at + self.valid_lifetime;
        return expires;
    }
    pub fn valid(&self) -> bool {
        Utc::now() < self.expiry_time()
    }
}

pub async fn authenticate_websocket(
    client: &(impl RequestClient + RequestHelpers),
) -> Result<WebsocketToken, error::RequestError> {
    do_request_no_params(
        client,
        "/0/private/GetWebSocketsToken",
        http::Method::POST,
        EndpointSecurityType::Private,
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::setup;
    use anyhow::Result;
    use log::info;

    #[tokio::test]
    async fn test_websocket_token() -> Result<()> {
        let bin = setup();
        let token = authenticate_websocket(&bin).await?;
        assert!(token.valid());
        info!("Got a websocket token {}", token);
        Ok(())
    }
}

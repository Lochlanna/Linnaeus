#[cfg(test)]
mod tests;
mod structs;

pub use structs::*;

use linnaeus_request::*;

pub async fn account_balances(
    client: &(impl RequestClient + RequestHelpers),
) -> Result<AccountBalances, error::RequestError> {
    do_request_no_params(
        client,
        "/0/private/Balance",
        http::Method::POST,
        EndpointSecurityType::Private,
    )
        .await
}

pub async fn trade_balances(
    client: &(impl RequestClient + RequestHelpers),
    params: &TradeBalancesParams
) -> Result<TradeBalances, error::RequestError> {
    do_request_with_body(
        client,
        "/0/private/TradeBalance",
        http::Method::POST,
        EndpointSecurityType::Private,
        params
    )
        .await
}
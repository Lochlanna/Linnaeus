mod structs;
#[cfg(test)]
mod tests;

use std::collections::HashMap;

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
    params: &TradeBalancesParams,
) -> Result<TradeBalances, error::RequestError> {
    do_request_with_body(
        client,
        "/0/private/TradeBalance",
        http::Method::POST,
        EndpointSecurityType::Private,
        params,
    )
    .await
}

pub async fn open_orders(
    client: &(impl RequestClient + RequestHelpers),
    params: &OpenOrdersParams,
) -> Result<HashMap<String, OrderBase>, error::RequestError> {
    let wrapper: OpenOrdersWrapper = do_request_with_body(
        client,
        "/0/private/OpenOrders",
        http::Method::POST,
        EndpointSecurityType::Private,
        params,
    )
    .await?;
    Ok(wrapper.open)
}

pub async fn closed_orders(
    client: &(impl RequestClient + RequestHelpers),
    params: &ClosedOrdersParams,
) -> Result<ClosedOrders, error::RequestError> {
    do_request_with_body(
        client,
        "/0/private/ClosedOrders",
        http::Method::POST,
        EndpointSecurityType::Private,
        params,
    )
    .await
}

pub async fn query_orders(
    client: &(impl RequestClient + RequestHelpers),
    params: &QueryOrderParams,
) -> Result<Orders, error::RequestError> {
    do_request_with_body(
        client,
        "/0/private/QueryOrders",
        http::Method::POST,
        EndpointSecurityType::Private,
        params,
    )
    .await
}

pub async fn trade_history(
    client: &(impl RequestClient + RequestHelpers),
    params: &TradeHistoryParams,
) -> Result<TradeHistory, error::RequestError> {
    do_request_with_body(
        client,
        "/0/private/TradesHistory",
        http::Method::POST,
        EndpointSecurityType::Private,
        params,
    )
    .await
}

pub async fn query_trade_info(
    client: &(impl RequestClient + RequestHelpers),
    params: &QueryTradeInfoParams,
) -> Result<TradeInfo, error::RequestError> {
    do_request_with_body(
        client,
        "/0/private/QueryTrades",
        http::Method::POST,
        EndpointSecurityType::Private,
        params,
    )
    .await
}

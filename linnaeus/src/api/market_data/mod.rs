mod structs;
#[cfg(test)]
mod tests;

pub use structs::*;

use linnaeus_request::*;

pub async fn server_time(
    client: &(impl RequestClient + RequestHelpers),
) -> Result<ServerTime, error::RequestError> {
    do_request_no_params(
        client,
        "/public/Time",
        http::Method::GET,
        EndpointSecurityType::None,
    )
    .await
}

pub async fn system_status(
    client: &(impl RequestClient + RequestHelpers),
) -> Result<SystemStatus, error::RequestError> {
    do_request_no_params(
        client,
        "/public/SystemStatus",
        http::Method::GET,
        EndpointSecurityType::None,
    )
    .await
}

pub async fn asset_info(
    client: &(impl RequestClient + RequestHelpers),
    params: &AssetInfoParams
) -> Result<AssetInfo, error::RequestError> {
    do_request_with_query(
        client,
        "/public/Assets",
        http::Method::GET,
        EndpointSecurityType::None,
        params
    )
    .await
}

pub async fn all_asset_info(
    client: &(impl RequestClient + RequestHelpers),
) -> Result<AssetInfo, error::RequestError> {
    do_request_no_params(
        client,
        "/public/Assets",
        http::Method::GET,
        EndpointSecurityType::None,
    )
    .await
}


pub async fn tradable_asset_pairs(
    client: &(impl RequestClient + RequestHelpers),
    params: &TradableAssetPairsParams
) -> Result<TradingAssetPairs, error::RequestError> {
    do_request_with_query(
        client,
        "/public/AssetPairs",
        http::Method::GET,
        EndpointSecurityType::None,
        params
    )
        .await
}

pub async fn all_tradable_asset_pairs(
    client: &(impl RequestClient + RequestHelpers),
) -> Result<TradingAssetPairs, error::RequestError> {
    do_request_no_params(
        client,
        "/public/AssetPairs",
        http::Method::GET,
        EndpointSecurityType::None,
    )
        .await
}

pub async fn ticker_information(
    client: &(impl RequestClient + RequestHelpers),
    params: &TickerInfoParams
) -> Result<MultiTickerInformation, error::RequestError> {
    do_request_with_query(
        client,
        "/public/Ticker",
        http::Method::GET,
        EndpointSecurityType::None,
        params
    )
        .await
}

pub async fn ohlc(
    client: &(impl RequestClient + RequestHelpers),
    params: &OHLCDataParams
) -> Result<OHLCData, error::RequestError> {
    do_request_with_query(
        client,
        "/public/OHLC",
        http::Method::GET,
        EndpointSecurityType::None,
        params
    )
        .await
}

pub async fn recent_trades(
    client: &(impl RequestClient + RequestHelpers),
    params: &RecentTradesParams
) -> Result<RecentTrades, error::RequestError> {
    do_request_with_query(
        client,
        "/public/Trades",
        http::Method::GET,
        EndpointSecurityType::None,
        params
    )
        .await
}
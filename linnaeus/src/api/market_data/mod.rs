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
    do_request(
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

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
        http::Method::GET,
        EndpointSecurityType::Private,
    )
        .await
}
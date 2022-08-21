pub mod kraken;

use std::collections::BTreeMap;
use thiserror::Error;
use crate::kraken::error::KrakenError;


#[derive(Error, Debug)]
pub enum LinnaeusError {
    #[error("Error from Kraken")]
    KrakenError(kraken::error::KrakenError)
}

#[derive(Debug)]
struct Config {
}

#[derive(Debug)]
struct Linnaeus {

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::kraken::market;
    use crate::kraken::KrakenType;
    #[test]
    fn it_works() {
        println!("{}", market::Time::kraken_path());
        println!("{}", market::Time::http_type());
        println!("{}", market::Time::authenticated_request());
    }
}

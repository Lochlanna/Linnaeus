pub mod kraken;
pub mod utils;

use std::collections::BTreeMap;
use thiserror::Error;
use linnaeus_derive::Kraken;
use crate::kraken::error::KrakenError;
use crate::utils::PrimitiveValue;


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
    #[test]
    fn it_works() {

    }
}

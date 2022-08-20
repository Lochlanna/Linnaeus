mod kraken;

use thiserror::Error;



#[derive(Error, Debug)]
pub enum LinnaeusError {
    #[error("Error from Kraken")]
    KrakenError(kraken::error::KrakenError)
}

struct Config {

}

struct Linnaeus {

}


#[cfg(test)]
mod tests {
    use super::*;
}

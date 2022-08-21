use crate::kraken::*;
use linnaeus_derive::Kraken;


#[derive(Debug, Serialize, Deserialize, Kraken)]
#[kraken_path(url = "/0/public/Time")]
pub struct Time {}

#[derive(Debug, Deserialize)]
pub struct TimeResult{
    unixtime: u128,
    rfc1123: String
}
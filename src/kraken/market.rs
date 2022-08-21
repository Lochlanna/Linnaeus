use crate::kraken::*;
use linnaeus_derive::kraken;


#[kraken(POST,"/hello/world", AUTH)]
pub struct Time {}

#[derive(Debug, Deserialize)]
pub struct TimeResult{
    unixtime: u128,
    rfc1123: String
}
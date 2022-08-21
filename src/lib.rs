pub mod kraken;

use thiserror::Error;
use crate::kraken::error::KrakenError;


#[derive(Error, Debug)]
pub enum LinnaeusError {
    #[error("Error from Kraken")]
    KrakenError(KrakenError)
}

#[derive(Debug)]
struct Config {
}

#[derive(Debug)]
struct Linnaeus {

}


#[cfg(test)]
mod tests {
    use reqwest::Client;
    use crate::kraken::market;
    use crate::kraken::*;

    #[tokio::test]
    async fn it_works() {
        let t = market::Time{};
        let mut a = Auth::new("hello world");
        let c = Client::new();
        let resp = match t.new_request(&mut a, &c).await {
            Ok(r) => r,
            Err(e) => {
                println!("Error was {}", e);
                return;
            }
        };

        println!("system time:{}", serde_json::to_string(&resp).unwrap());
        let s = market::SystemStatus{};
        let resp = match s.new_request(&mut a, &c).await {
            Ok(r) => r,
            Err(e) => {
                println!("Error was {}", e);
                return;
            }
        };

        println!("system status:{}", serde_json::to_string(&resp).unwrap());

        let ai = market::AssetInfo::new(crate::kraken::enums::Asset::BTC, None);
        let resp = match ai.new_request(&mut a, &c).await {
            Ok(r) => r,
            Err(e) => {
                println!("Error was {}", e);
                return;
            }
        };

        println!("BTC info is:{}", serde_json::to_string(&resp).unwrap());

    }
}

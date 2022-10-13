pub mod user_staking;
pub mod user_funding;
pub mod user_trading;
pub mod user_data;
pub mod market_data;


pub fn concat_strings_serializer<S>(i: &[String],serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
    let out = i.join(",");
    serializer.serialize_str(&out)
}
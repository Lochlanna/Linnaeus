use thiserror::Error;

#[derive(Error, Debug)]
pub enum LinnaeusWebsocketError {
    #[error("Failed to serialize message -> {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("websocket error -> {0}")]
    Websocket(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("url parsing error error -> {0}")]
    UrlParse(#[from] url::ParseError),
    #[error("Invalid websocket url -> {reason}")]
    Url{reason: &'static str},
    #[error("Kraken is not online")]
    KrakenOffline
}

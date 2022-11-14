use thiserror::Error;

#[derive(Error, Debug)]
pub enum LinnaeusWebsocketError {
    #[error("Failed to serialize message -> {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("websocket error -> {0}")]
    Websocket(#[from] tokio_tungstenite::tungstenite::Error)
}

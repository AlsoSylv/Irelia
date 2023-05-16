#![feature(lazy_cell)]

#[cfg(feature = "in_game")]
pub mod in_game;
#[cfg(feature = "rest")]
pub mod rest;
pub mod utils;
#[cfg(feature = "ws")]
pub mod ws;

/// Different error types that can produced, most are from other crates
#[derive(Debug)]
pub enum Error {
    #[cfg(any(feature = "in_game", feature = "rest"))]
    HyperHttpError(hyper::http::Error),
    #[cfg(any(feature = "in_game", feature = "rest"))]
    HyperError(hyper::Error),
    SerdeJsonError(serde_json::Error),
    #[cfg(feature = "ws")]
    WebsocketError(tokio_tungstenite::tungstenite::Error),
    LCUProcessNotRunning,
    PortNotFound,
    AuthTokenNotFound,
}

#![feature(lazy_cell, test)]

//! Irelia is an async set of bindings to the LCU API
//!
//! Features are broken down as follows:
//! - in_game: Allows connections to the in_game API, return types are auto generated
//! - rest: Allows connections to the LCU rest API, providing basic get/post functionality
//! - ws: Allows connections to the LCU websocket API, providing all functionality needed
//!
//! Irelia is currently nightly only, as it relies on the lazy_cell feature internally

pub use irelia_encoder;

#[cfg(feature = "in_game")]
pub mod in_game;
#[cfg(feature = "rest")]
pub mod rest;
pub mod utils;
#[cfg(feature = "ws")]
pub mod ws;

/// Errors that can be produced by the LCU API
///
/// This contians errors from serde_json, hyper and tungstenite
#[derive(Debug)]
pub enum LCUError {
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

#[cfg(feature = "tauri")]
impl ToString for LCUError {
    fn to_string(&self) -> String {
        match self {
            #[cfg(any(feature = "in_game", feature = "rest"))]
            LCUError::HyperHttpError(err) => err.to_string(),
            #[cfg(any(feature = "in_game", feature = "rest"))]
            LCUError::HyperError(err) => err.to_string(),
            LCUError::SerdeJsonError(err) => err.to_string(),
            #[cfg(feature = "ws")]
            LCUError::WebsocketError(err) => err.to_string(),
            LCUError::LCUProcessNotRunning => String::from("LCU Process is not running!"),
            LCUError::PortNotFound => String::from("Port Not Found!"),
            LCUError::AuthTokenNotFound => String::from("Auth Token Not Found!"),
        }
    }
}

#[cfg(feature = "tauri")]
impl serde::Serialize for LCUError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

#[cfg(any(feature = "in_game", feature = "rest"))]
/// Struct that represents any connection to the in game or rest APIs, this client has to be constructed and then passed to the clients
///
/// # Example
/// ```rs
/// use irelia::{RequestClient, rest::LCUClient};
///
/// fn main() {
///     let client = RequestClient::new();
///     
///     let lcu_client = LCUClient::new(&client);
/// }
/// ```
pub struct RequestClient {
    client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
}

#![warn(clippy::pedantic)]

//! Irelia is an async set of bindings to the LCU API
//!
//! Features are broken down as follows:
//! - `in_game`: Allows connections to the `in_game` API, return types are auto generated
//! - `rest`: Allows connections to the LCU `rest` API, providing basic get/post functionality
//! - `ws`: Allows connections to the LCU websocket API, providing all functionality needed
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
/// This contains errors from `serde_json`, `hyper` and `tungstenite`
#[derive(Debug)]
pub enum LCUError {
    #[cfg(any(feature = "in_game", feature = "rest"))]
    HyperHttpError(hyper::http::Error),
    #[cfg(any(feature = "in_game", feature = "rest"))]
    HyperClientError(hyper_util::client::legacy::Error),
    #[cfg(any(feature = "in_game", feature = "rest"))]
    HyperError(hyper::Error),
    SerdeJsonError(serde_json::Error),
    #[cfg(feature = "ws")]
    WebsocketError(tokio_tungstenite::tungstenite::Error),
    StdIo(std::io::Error),
    LCUProcessNotRunning,
    PortNotFound,
    AuthTokenNotFound,
    LockFileNotFound,
}

impl std::fmt::Display for LCUError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error = match self {
            #[cfg(any(feature = "in_game", feature = "rest"))]
            LCUError::HyperHttpError(err) => err.to_string(),
            #[cfg(any(feature = "in_game", feature = "rest"))]
            LCUError::HyperError(err) => err.to_string(),
            #[cfg(any(feature = "in_game", feature = "rest"))]
            LCUError::HyperClientError(err) => err.to_string(),
            LCUError::SerdeJsonError(err) => err.to_string(),
            #[cfg(feature = "ws")]
            LCUError::WebsocketError(err) => err.to_string(),
            LCUError::StdIo(err) => err.to_string(),
            LCUError::LCUProcessNotRunning => String::from("LCU Process is not running!"),
            LCUError::PortNotFound => String::from("Port Not Found!"),
            LCUError::AuthTokenNotFound => String::from("Auth Token Not Found!"),
            LCUError::LockFileNotFound => {
                String::from("Unable to the lock file, but the process was running!")
            }
        };
        f.write_fmt(format_args!("LCU Error: {error}"))
    }
}

impl std::error::Error for LCUError {}

#[cfg(feature = "tauri")]
impl serde::Serialize for LCUError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(any(feature = "rest", feature = "in_game"))]
pub use utils::requests::RequestClient;

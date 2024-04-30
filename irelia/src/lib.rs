#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

//! Irelia is an async set of bindings to the LCU API
//!
//! Features are broken down as follows:
//! - `in_game`: Allows connections to the `in_game` API, return types are auto generated
//! - `rest`: Allows connections to the LCU `rest` API, providing basic get/post functionality
//! - `ws`: Allows connections to the LCU websocket API, providing all functionality needed
pub use irelia_encoder;

#[cfg(feature = "in_game")]
pub mod in_game;
#[cfg(feature = "replay")]
pub mod replay;
#[cfg(feature = "rest")]
pub mod rest;
pub mod utils;
#[cfg(feature = "ws")]
pub mod ws;

/// Errors that can be produced by the LCU API
///
/// This contains errors from `serde_json`, `hyper` and `tungstenite`
#[derive(Debug)]
pub enum Error {
    #[cfg(any(feature = "in_game", feature = "rest"))]
    HyperHttpError(hyper::http::Error),
    #[cfg(any(feature = "in_game", feature = "rest"))]
    HyperClientError(hyper_util::client::legacy::Error),
    #[cfg(any(feature = "in_game", feature = "rest"))]
    HyperError(hyper::Error),
    #[cfg(feature = "ws")]
    WebsocketError(tokio_tungstenite::tungstenite::Error),
    #[cfg(any(feature = "ws", feature = "rest"))]
    StdIo(std::io::Error),
    SerdeJsonError(serde_json::Error),
    LCUProcessNotRunning,
    PortNotFound,
    AuthTokenNotFound,
    LockFileNotFound,
}

#[cfg(any(feature = "in_game", feature = "rest"))]
impl From<hyper::http::Error> for Error {
    fn from(value: hyper::http::Error) -> Self {
        Self::HyperHttpError(value)
    }
}

#[cfg(any(feature = "in_game", feature = "rest"))]
impl From<hyper_util::client::legacy::Error> for Error {
    fn from(value: hyper_util::client::legacy::Error) -> Self {
        Self::HyperClientError(value)
    }
}

#[cfg(any(feature = "in_game", feature = "rest"))]
impl From<hyper::Error> for Error {
    fn from(value: hyper::Error) -> Self {
        Self::HyperError(value)
    }
}

#[cfg(feature = "ws")]
impl From<tokio_tungstenite::tungstenite::Error> for Error {
    fn from(value: tokio_tungstenite::tungstenite::Error) -> Self {
        Self::WebsocketError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJsonError(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::StdIo(value)
    }
}


impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error: std::borrow::Cow<'_, str> = match self {
            #[cfg(any(feature = "in_game", feature = "rest"))]
            Error::HyperHttpError(err) => err.to_string().into(),
            #[cfg(any(feature = "in_game", feature = "rest"))]
            Error::HyperError(err) => err.to_string().into(),
            #[cfg(any(feature = "in_game", feature = "rest"))]
            Error::HyperClientError(err) => err.to_string().into(),
            Error::SerdeJsonError(err) => err.to_string().into(),
            #[cfg(feature = "ws")]
            Error::WebsocketError(err) => err.to_string().into(),
            Error::StdIo(err) => err.to_string().into(),
            Error::LCUProcessNotRunning => "LCU Process is not running!".into(),
            Error::PortNotFound => "Port Not Found!".into(),
            Error::AuthTokenNotFound => "Auth Token Not Found!".into(),
            Error::LockFileNotFound => {
                "Unable to the lock file, but the process was running!".into()
            }
        };
        f.write_str(&error)
    }
}

impl std::error::Error for Error {}

#[cfg(feature = "tauri")]
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(any(feature = "rest", feature = "in_game"))]
pub use utils::requests::RequestClient;

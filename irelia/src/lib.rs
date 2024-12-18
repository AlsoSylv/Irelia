#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![forbid(unsafe_code)]

//! Irelia is an async set of bindings to the LCU API
//!
//! Features are broken down as follows:
//! - `in_game`: Allows connections to the `in_game` API, return types are auto generated
//! - `rest`: Allows connections to the LCU `rest` API, providing basic get/post functionality
//! - `ws`: Allows connections to the LCU websocket API, providing all functionality needed
//! - `replay`: Allows connections to the `replay` API, also enables the in game API

#[cfg(feature = "in_game")]
pub mod in_game;
#[cfg(feature = "replay")]
pub mod replay;
#[cfg(feature = "rest")]
pub mod rest;
pub(crate) mod utils;
#[cfg(feature = "ws")]
pub mod ws;
#[cfg(any(feature = "ws", feature = "rest"))]
pub use utils::process_info;

#[cfg(any(feature = "rest", feature = "in_game"))]
pub use error::Error;

#[cfg(any(feature = "rest", feature = "in_game"))]
pub use utils::requests::RequestClient;

#[cfg(any(feature = "rest", feature = "in_game"))]
mod error {
    use crate::process_info;

    /// Errors that can be produced by the LCU API
    ///
    /// This contains errors from `serde_json`, `hyper` and `tungstenite`
    #[derive(Debug)]
    pub enum Error {
        HyperHttpError(hyper::http::Error),
        HyperClientError(hyper_util::client::legacy::Error),
        HyperError(hyper::Error),
        RequestError(hyper::StatusCode),
        RmpSerdeEncode(rmp_serde::encode::Error),
        RmpSerdeDecode(rmp_serde::decode::Error),
        #[cfg(feature = "rest")]
        ProcessInfoError(process_info::Error),
    }

    impl From<hyper::http::Error> for Error {
        fn from(value: hyper::http::Error) -> Self {
            Self::HyperHttpError(value)
        }
    }

    impl From<hyper_util::client::legacy::Error> for Error {
        fn from(value: hyper_util::client::legacy::Error) -> Self {
            Self::HyperClientError(value)
        }
    }

    impl From<hyper::Error> for Error {
        fn from(value: hyper::Error) -> Self {
            Self::HyperError(value)
        }
    }

    impl From<rmp_serde::encode::Error> for Error {
        fn from(value: rmp_serde::encode::Error) -> Self {
            Self::RmpSerdeEncode(value)
        }
    }

    impl From<rmp_serde::decode::Error> for Error {
        fn from(value: rmp_serde::decode::Error) -> Self {
            Self::RmpSerdeDecode(value)
        }
    }

    #[cfg(feature = "rest")]
    impl From<process_info::Error> for Error {
        fn from(value: process_info::Error) -> Self {
            Self::ProcessInfoError(value)
        }
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let error: std::borrow::Cow<'_, str> = match self {
                Self::HyperHttpError(err) => err.to_string().into(),
                Self::HyperError(err) => err.to_string().into(),
                Self::HyperClientError(err) => err.to_string().into(),
                Self::RequestError(code) => code.as_str().into(),
                #[cfg(feature = "rest")]
                Self::ProcessInfoError(err) => err.reason().into(),
                Self::RmpSerdeEncode(err) => err.to_string().into(),
                Self::RmpSerdeDecode(err) => err.to_string().into(),
            };
            f.write_str(&error)
        }
    }

    impl std::error::Error for Error {}

    impl serde::Serialize for Error {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(&self.to_string())
        }
    }

    #[cfg(feature = "rest")]
    impl From<hyper::header::InvalidHeaderValue> for Error {
        fn from(value: hyper::header::InvalidHeaderValue) -> Self {
            Self::HyperHttpError(hyper::http::Error::from(value))
        }
    }
}

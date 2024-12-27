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
#[cfg(any(feature = "rest", feature = "ws", feature = "in_game"))]
pub(crate) mod tls;
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
    /// Errors that can be produced by the LCU API
    ///
    /// This contains errors from `serde_json`, `hyper` and `tungstenite`
    #[derive(Debug)]
    pub enum Error {
        /// http error, re-exported by hyper
        HyperHttpError(hyper::http::Error),
        /// Client error from `hyper_util`
        HyperClientError(hyper_util::client::legacy::Error),
        /// Hyper error
        HyperError(hyper::Error),
        /// Error with the request, contains a status code
        RequestError(hyper::StatusCode),
        /// Encode error
        RmpSerdeEncode(rmp_serde::encode::Error),
        /// Decode error
        RmpSerdeDecode(rmp_serde::decode::Error),
        /// Error getting process info (only possible with the `rest` feature enabled)
        #[cfg(feature = "rest")]
        ProcessInfoError(crate::process_info::Error),
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
    impl From<crate::process_info::Error> for Error {
        fn from(value: crate::process_info::Error) -> Self {
            Self::ProcessInfoError(value)
        }
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::HyperHttpError(err) => err.fmt(f),
                Self::HyperError(err) => err.fmt(f),
                Self::HyperClientError(err) => err.fmt(f),
                Self::RequestError(code) => f.write_str(code.as_str()),
                #[cfg(feature = "rest")]
                Self::ProcessInfoError(err) => f.write_str(err.reason()),
                Self::RmpSerdeEncode(err) => err.fmt(f),
                Self::RmpSerdeDecode(err) => err.fmt(f),
            }
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

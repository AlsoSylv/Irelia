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
pub use utils::requests;

pub(crate) use error::Error;

#[cfg(any(feature = "rest", feature = "in_game"))]
pub mod error {
    use http::header::InvalidHeaderValue;

    pub trait HttpError: std::error::Error {
        fn invalid_header_value(invalid_header_value: InvalidHeaderValue) -> Self;
    }

    /// Errors that can be produced by the LCU API
    ///
    /// This contains errors from `serde_json`, `hyper` and `tungstenite`
    #[derive(Debug)]
    pub enum Error<E: HttpError> {
        HttpError(E),
        /// Error with the request, contains a status code
        RequestError(http::StatusCode),
        /// Encode error
        RmpSerdeEncode(rmp_serde::encode::Error),
        /// Decode error
        RmpSerdeDecode(rmp_serde::decode::Error),
        SerdeJsonError(serde_json::Error),
        /// Error getting process info (only possible with the `rest` feature enabled)
        #[cfg(feature = "rest")]
        ProcessInfoError(crate::process_info::Error),
    }

    pub enum SerdeError {
        RmpEncode(rmp_serde::encode::Error),
        RmpDecode(rmp_serde::decode::Error),
        Json(serde_json::Error),
    }

    impl From<rmp_serde::encode::Error> for SerdeError {
        fn from(value: rmp_serde::encode::Error) -> Self {
            Self::RmpEncode(value)
        }
    }

    impl From<rmp_serde::decode::Error> for SerdeError {
        fn from(value: rmp_serde::decode::Error) -> Self {
            Self::RmpDecode(value)
        }
    }

    impl From<serde_json::Error> for SerdeError {
        fn from(value: serde_json::Error) -> Self {
            Self::Json(value)
        }
    }

    impl<E: HttpError> From<SerdeError> for Error<E> {
        fn from(value: SerdeError) -> Self {
            match value {
                SerdeError::Json(err) => Self::SerdeJsonError(err),
                SerdeError::RmpDecode(err) => Self::RmpSerdeDecode(err),
                SerdeError::RmpEncode(err) => Self::RmpSerdeEncode(err),
            }
        }
    }

    impl<E: HttpError> From<InvalidHeaderValue> for Error<E> {
        fn from(value: InvalidHeaderValue) -> Self {
            Error::HttpError(E::invalid_header_value(value))
        }
    }

    impl<E: HttpError> From<E> for Error<E> {
        fn from(value: E) -> Self {
            Self::HttpError(value)
        }
    }

    impl<E: HttpError> From<rmp_serde::encode::Error> for Error<E> {
        fn from(value: rmp_serde::encode::Error) -> Self {
            Self::RmpSerdeEncode(value)
        }
    }

    impl<E: HttpError> From<rmp_serde::decode::Error> for Error<E> {
        fn from(value: rmp_serde::decode::Error) -> Self {
            Self::RmpSerdeDecode(value)
        }
    }

    impl<E: HttpError> From<serde_json::Error> for Error<E> {
        fn from(value: serde_json::Error) -> Self {
            Self::SerdeJsonError(value)
        }
    }

    #[cfg(feature = "rest")]
    impl<E: HttpError> From<crate::process_info::Error> for Error<E> {
        fn from(value: crate::process_info::Error) -> Self {
            Self::ProcessInfoError(value)
        }
    }

    impl<E: HttpError> std::fmt::Display for Error<E> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::HttpError(e) => std::fmt::Display::fmt(e, f),
                Self::RequestError(code) => f.write_str(code.as_str()),
                #[cfg(feature = "rest")]
                Self::ProcessInfoError(err) => f.write_str(err.reason()),
                Self::RmpSerdeEncode(err) => err.fmt(f),
                Self::RmpSerdeDecode(err) => err.fmt(f),
                Self::SerdeJsonError(err) => err.fmt(f),
            }
        }
    }

    impl<E: HttpError> std::error::Error for Error<E> {}

    impl<E: HttpError> serde::Serialize for Error<E> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(&self.to_string())
        }
    }
}

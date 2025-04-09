use http::header::InvalidHeaderValue;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
/// Enum of possible errors that will be passed to the `ErrorHandler`
pub enum Error {
    Tungstenite(Box<tungstenite::Error>),
    ProcessInfo(crate::process_info::Error),
    SerdeJson(serde_json::Error),
    Io(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tungstenite(e) => e.fmt(f),
            Self::ProcessInfo(e) => e.fmt(f),
            Self::SerdeJson(e) => e.fmt(f),
            Self::Io(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<tungstenite::Error> for Error {
    fn from(value: tungstenite::Error) -> Self {
        Self::Tungstenite(Box::new(value))
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJson(value)
    }
}

impl From<crate::process_info::Error> for Error {
    fn from(value: crate::process_info::Error) -> Self {
        Self::ProcessInfo(value)
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(value: InvalidHeaderValue) -> Self {
        Self::Tungstenite(Box::new(tungstenite::Error::HttpFormat(
            tungstenite::http::Error::from(value),
        )))
    }
}

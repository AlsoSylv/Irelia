use std::fmt::{Display, Formatter};

#[derive(Debug)]
/// Enum of possible errors that will be passed to the `ErrorHandler`
pub enum Error {
    Tungstenite(tungstenite::Error),
    ProcessInfo(crate::process_info::Error),
    SerdeJson(serde_json::Error),
    Io(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::Tungstenite(e) => e.to_string(),
            Self::ProcessInfo(e) => e.to_string(),
            Self::SerdeJson(e) => e.to_string(),
            Self::Io(e) => e.to_string(),
        };

        f.write_str(&string)
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
        Self::Tungstenite(value)
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

pub mod rest;
mod utils;
#[cfg(feature = "ws")]
pub mod ws;

#[derive(Debug, Clone, Copy)]
pub enum Errors {
    FailedParseJson,
    LCUStoppedRunning,
    InvalidRequest,
    InvalidBody,
    ProcessInfoError(ProcessInfoErrors),
}

#[derive(Debug, Clone, Copy)]
pub enum ProcessInfoErrors {
    LCUProcessNotRunning,
    PortNotFound,
    CannotLaunchTerminal,
    AuthTokenNotFound,
}

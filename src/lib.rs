pub mod rest;
mod utils;

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
    FailedParsePort,
    AuthTokenNotFound,
}

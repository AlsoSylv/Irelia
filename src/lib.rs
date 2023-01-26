#[cfg(feature = "in_game")]
pub mod in_game;
#[cfg(feature = "rest")]
pub mod rest;
mod utils;
#[cfg(feature = "ws")]
pub mod ws;

#[derive(Debug, Clone, Copy)]
pub enum Errors {
    FailedParseJson,
    LCUStoppedRunning,
    #[cfg(feature = "in_game")]
    LeagueStoppedRunning,
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

#[cfg(feature = "in_game")]
pub mod in_game;
#[cfg(feature = "rest")]
pub mod rest;
mod utils;
#[cfg(feature = "ws")]
pub mod ws;

#[derive(Debug, Clone, Copy)]
pub enum Error {
    FailedParseJson,
    LCUStoppedRunning,
    #[cfg(feature = "in_game")]
    LeagueStoppedRunning,
    InvalidRequest,
    InvalidBody,
    LCUProcessNotRunning,
    PortNotFound,
    CannotLaunchTerminal,
    AuthTokenNotFound,
}

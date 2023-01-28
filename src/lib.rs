//! Irelia is a wrapper around the LoL native APIs, with a focus on modularity and compile size
//! This crate has support for Windows, Linux, and MacOS, all of which have been tested to varying degrees


#[cfg(feature = "in_game")]
/// The in_game module has support for LoLs in game API, and returns all data as structs that much the 
/// Current spec release by Riot, more info can be found here: <https://developer.riotgames.com/docs/lol#game-client-api>
pub mod in_game;
#[cfg(feature = "rest")]
/// The rest module provides support for the LCU Rest API, and allows passing custom return types to each method
/// As long as they implement serde::Deserialize, more info can be found here: <https://hextechdocs.dev/getting-started-with-the-lcu-api/>
pub mod rest;
mod utils;
#[cfg(feature = "ws")]
/// The ws module provides support for the LCU Web Socket, and returns all data as `Value`
/// More info can be found here: <https://hextechdocs.dev/getting-started-with-the-lcu-websocket/>
/// 
/// This is a high level implementation of the LCU websocket, which managesthe event loop itself
/// Methods to subscribe, unsubscribe, and terminate the event loop are provided
pub mod ws;

/// Custom errors for the LCU
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

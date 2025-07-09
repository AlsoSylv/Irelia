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

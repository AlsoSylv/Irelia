//! This module uses sub-processes that are OS specific to get info
//! from the League of Legneds client well it is running, or error
//! if it is not. These sub-processes are derived from those found
//! on Hextech docs here, as well as personal testing on Linux.
//! <https://hextechdocs.dev/getting-started-with-the-lcu-websocket/>

use sysinfo::{ProcessExt, System, SystemExt};

use crate::Error;

use super::encoder::encode;

#[cfg(target_os = "windows")]
const TARGET_PROCESS_NAME: &str = "LeagueClientUx.exe";
#[cfg(target_os = "linux")]
const TARGET_PROCESS_NAME: &str = "LeagueClientUx.";
#[cfg(target_os = "macos")]
const TARGET_PROCESS_NAME: &str = "LeagueClientUx";

/// Gets the port and auth for the client via the process id
/// This is done to avoid needing to find the lock file, but
/// a fallback could be implemented in theory using the fact
/// that you can get the exe location, and go backwards.
pub(crate) fn get_running_client() -> Result<(String, String), Error> {
    let mut system = System::new();
    system.refresh_processes();

    let process = system
        .processes()
        .values()
        .find(|process| process.name() == TARGET_PROCESS_NAME)
        .map(|process| process.cmd().join(" "))
        .ok_or(Error::LCUProcessNotRunning)?;

    let port = process
        .split_whitespace()
        .find_map(|s| s.strip_prefix("--app-port="))
        .ok_or(Error::PortNotFound)?;
    let auth = process
        .split_whitespace()
        .find_map(|s| s.strip_prefix("--remoting-auth-token="))
        .ok_or(Error::AuthTokenNotFound)?;

    Ok((
        format!("127.0.0.1:{}", port),
        format!("Basic {}", encode(format!("riot:{}", auth))),
    ))
}
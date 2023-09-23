//! This module uses sub-processes that are OS specific to get info
//! from the League of Legneds client well it is running, or error
//! if it is not. These sub-processes are derived from those found
//! on Hextech docs here, as well as personal testing on Linux.
//! <https://hextechdocs.dev/getting-started-with-the-lcu-websocket/>

//! This module also contains a list of constants for the different names
//! of the processes for Linux, MacOS, and Windows

use irelia_encoder::Encoder;
use sysinfo::{ProcessExt, System, SystemExt};

use crate::LCUError;

#[cfg(target_os = "windows")]
const TARGET_PROCESS_NAME: &str = "LeagueClientUx.exe";
#[cfg(target_os = "linux")]
const TARGET_PROCESS_NAME: &str = "LeagueClientUx."; // Might be "LeagueClient.ex" instead, but I can't validate anything right now
#[cfg(target_os = "macos")]
const TARGET_PROCESS_NAME: &str = "LeagueClientUx";

/// Gets the port and auth for the client via the process id
/// This is done to avoid needing to find the lock file, but
/// a fallback could be implemented in theory using the fact
/// that you can get the exe location, and go backwards.
pub(crate) fn get_running_client() -> Result<(String, String), LCUError> {
    // Get the current list of processes
    let mut system = System::new();
    system.refresh_processes();

    /* 
        Iterate through all of the processes, using .values() because
        We don't need the PID. Try to find a process with the same name
        as the constant for that platform, ohterwise return an error.
    */ 
    let process = system
        .processes()
        .values()
        .find(|process| process.name() == TARGET_PROCESS_NAME)
        .map(|process| process.cmd().join(" "))
        .ok_or(LCUError::LCUProcessNotRunning)?;

    /*
        Look for a port to connect to the LCU with, otherwise return an error.
        If no port is found, but the LCU is running, something is probably wrong
        with the constant, or the code itself
     */
    let port = process
        .split_whitespace()
        .find_map(|s| s.strip_prefix("--app-port="))
        .ok_or(LCUError::PortNotFound)?;

    /*
        Look for an auth key to put inside the header, otherwise return an error.
        If no auth key is found, but the LCU is running, something is probably wrong
        with the constant, or the code itself
     */
    let auth = process
        .split_whitespace()
        .find_map(|s| s.strip_prefix("--remoting-auth-token="))
        .ok_or(LCUError::AuthTokenNotFound)?;

    // Create a new instance of the encoder
    const ENCODER: Encoder = Encoder::new();

    // Format the port and header so that they can be used properly
    Ok((
        format!("127.0.0.1:{}", port),
        format!("Basic {}", ENCODER.encode(format!("riot:{}", auth))),
    ))
}

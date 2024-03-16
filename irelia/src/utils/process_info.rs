//! This module uses sub-processes that are OS specific to get info
//! from the League of Legneds client well it is running, or error
//! if it is not. These sub-processes are derived from those found
//! on Hextech docs.
//! <https://hextechdocs.dev/getting-started-with-the-lcu-api/>

//! This module also contains a list of constants for the different names
//! of the processes for MacOS, and Windows

use irelia_encoder::Encoder;
use sysinfo::{ProcessRefreshKind, RefreshKind, System};

use crate::LCUError;

// Linux will be unplayable soon, so support has been removed
#[cfg(target_os = "windows")]
const CLIENT_PROCESS_NAME: &str = "LeagueClientUx.exe";
#[cfg(target_os = "macos")]
const CLIENT_PROCESS_NAME: &str = "LeagueClientUx";

#[cfg(target_os = "windows")]
const GAME_PROCESS_NAME: &str = "League of Legends.exe";
#[cfg(target_os = "macos")]
const GAME_PROCESS_NAME: &str = "League of Legends";

// Create a new instance of the encoder
const ENCODER: Encoder = Encoder::new();

/// Gets the port and auth for the client via the process id
/// This is done to avoid needing to find the lock file, but
/// a fallback could be implemented in theory using the fact
/// that you can get the exe location, and go backwards.
pub(crate) fn get_running_client() -> Result<(String, String), LCUError> {
    // Get the current list of processes
    let system = System::new_with_specifics(
        RefreshKind::new().with_processes(
            ProcessRefreshKind::new()
                .with_exe(sysinfo::UpdateKind::OnlyIfNotSet)
                .with_cmd(sysinfo::UpdateKind::OnlyIfNotSet),
        ),
    );

    // Is the client running, or is it the game?
    let mut client = false;

    /*
        Iterate through all of the processes, using .values() because
        We don't need the PID. Try to find a process with the same name
        as the constant for that platform, ohterwise return an error.
    */
    let process = system
        .processes()
        .values()
        .find(|process| {
            let name = process.name();
            if name == CLIENT_PROCESS_NAME {
                client = true;
                client
            } else {
                name == GAME_PROCESS_NAME
            }
        })
        .ok_or(LCUError::LCUProcessNotRunning)?;

    // Move these to an earlier scope to avoid an allocation and
    let lock_file: String;
    let port: &str;
    let auth: &str;

    if client {
        let cmd = process.cmd();

        // Assuming the order doesn't change (whihc I haven't seen it do)
        // We can avoid a second iteration over the cmd args
        let mut iter = cmd.iter();

        /*
           Look for an auth key to put inside the header, otherwise return an error.
           If no auth key is found, but the LCU is running, something is probably wrong
           with the constant, or the code itself
        */
        auth = iter
            .find_map(|s| s.strip_prefix("--remoting-auth-token="))
            .ok_or(LCUError::AuthTokenNotFound)?;
        /*
           Look for a port to connect to the LCU with, otherwise return an error.
           If no port is found, but the LCU is running, something is probably wrong
           with the constant, or the code itself
        */
        port = iter
            .find_map(|s| s.strip_prefix("--app-port="))
            .ok_or(LCUError::PortNotFound)?;
    } else {
        let path = process.exe().ok_or(LCUError::LockFileNotFound)?;
        let path = path
            .parent()
            .ok_or(LCUError::LockFileNotFound)?
            .parent()
            .ok_or(LCUError::LockFileNotFound)?;

        lock_file = std::fs::read_to_string(path.join("lockfile")).map_err(LCUError::StdIo)?;

        // Split the lock file on `:` which seperates the different fields
        let mut split = lock_file.split(':');

        // Get the 3rd field
        port = split.nth(2).ok_or(LCUError::PortNotFound)?;
        // We moved the cursor, so the fourth element is the very next one
        auth = split.next().ok_or(LCUError::AuthTokenNotFound)?;
    }

    // Format the port and header so that they can be used properly
    Ok((
        format!("127.0.0.1:{}", port),
        format!("Basic {}", ENCODER.encode(format!("riot:{}", auth))),
    ))
}

#[cfg(test)]
mod tests {
    use super::get_running_client;

    #[ignore = "This is only needed for testing, and doesn't need to be run all the time"]
    #[test]
    fn test_process_info() {
        let (port, pass) = get_running_client().unwrap();
        println!("{port} {pass}")
    }
}

//! Constants, as well as the schema for the lock file can be found here
//! <https://hextechdocs.dev/getting-started-with-the-lcu-api/>

//! This module also contains a list of constants for the different names
//! of the processes for `macOS`, and `Windows`

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

// These ONLY exist so that this will compile in a dev env
// Linux support will not be re-added unless someone comes
// up with a solution to running the game on Linux natively
#[cfg(debug_assertions)]
#[cfg(target_os = "linux")]
const CLIENT_PROCESS_NAME: &str = "";
#[cfg(debug_assertions)]
#[cfg(target_os = "linux")]
const GAME_PROCESS_NAME: &str = "";

/// const copy of the encoder
pub(crate) const ENCODER: Encoder = Encoder::new();

/// Gets the port and auth for the client via the process id
/// This is done to avoid needing to find the lock file, but
/// a fallback could be implemented in theory using the fact
/// that you can get the exe location, and go backwards.
///
/// # Errors
/// This will return an error if the LCU is truly not running,
/// or the lock file is inaccessibly for some reason.
/// If it returns an error for any other reason, this code
/// likely needs the client and game process names updated.
pub(crate) fn get_running_client() -> Result<(String, String), LCUError> {
    // Get the current list of processes
    let system = System::new_with_specifics(
        // This creates a new instance of `system` every time, so this only
        //  needs to be updated if it's not set
        RefreshKind::new().with_processes(
            ProcessRefreshKind::new()
                .with_exe(sysinfo::UpdateKind::OnlyIfNotSet)
                .with_cmd(sysinfo::UpdateKind::OnlyIfNotSet),
        ),
    );

    // Is the client running, or is it the game?
    let mut client = false;

    /*
        Iterate through all the processes, using .values() because
        We don't need the PID. Try to find a process with the same name
        as the constant for that platform, otherwise return an error.
    */
    let process = system
        .processes()
        .values()
        .find(|process| {
            // Get the name of the process
            let name = process.name();
            // If it matches the name of the client,
            // set the flag, and return it
            if name == CLIENT_PROCESS_NAME {
                client = true;
                client
                // Otherwise return if it matches the game name process
            } else {
                name == GAME_PROCESS_NAME
            }
        })
        .ok_or(LCUError::LCUProcessNotRunning)?;

    // Move these to an earlier scope to avoid an allocation
    // And deduplicate some code later on
    let lock_file: String;
    let port: &str;
    let auth: &str;

    if client {
        let cmd = process.cmd();

        // Assuming the order doesn't change (which I haven't seen it do)
        // We can avoid a second iteration over the cmd args
        let mut iter = cmd.iter();

        /*
           Look for an auth key to put inside the command line, otherwise return an error.
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
        // We have to walk back twice to get the path of the lock file relative to the path of the game
        // This can only be None on Linux according to the docs, so we should be fine everywhere else
        let path = process.exe().ok_or(LCUError::LockFileNotFound)?;
        let path = path
            .parent()
            .ok_or(LCUError::LockFileNotFound)?
            .parent()
            .ok_or(LCUError::LockFileNotFound)?;

        // Read the lock file, initialize the lock_file var with the value
        lock_file = std::fs::read_to_string(path.join("lockfile")).map_err(LCUError::StdIo)?;

        // Split the lock file on `:` which separates the different fields
        // Because lock_file is from a higher scope, we can split the string here
        // and return two string references later on
        let mut split = lock_file.split(':');

        // Get the 3rd field, which should be the port
        port = split.nth(2).ok_or(LCUError::PortNotFound)?;
        // We moved the cursor, so the fourth element is the very next one
        // Which should be the auth string
        auth = split.next().ok_or(LCUError::AuthTokenNotFound)?;
    }

    // Format the port and header so that they can be used as headers
    // For the LCU API
    Ok((
        format!("127.0.0.1:{port}"),
        format!("Basic {}", ENCODER.encode(format!("riot:{auth}"))),
    ))
}

#[cfg(test)]
mod tests {
    use super::get_running_client;

    #[ignore = "This is only needed for testing, and doesn't need to be run all the time"]
    #[test]
    fn test_process_info() {
        let (port, pass) = get_running_client().unwrap();
        println!("{port} {pass}");
    }
}

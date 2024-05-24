//! Constants, as well as the schema for the lock file can be found here
//! <https://hextechdocs.dev/getting-started-with-the-lcu-api/>

//! This module also contains a list of constants for the different names
//! of the processes for `macOS`, and `Windows`

use irelia_encoder::Encoder;
use sysinfo::{ProcessRefreshKind, RefreshKind, System};

use crate::Error;

// Linux will be unplayable soon, so support has been removed
#[cfg(target_os = "windows")]
pub const CLIENT_PROCESS_NAME: &str = "LeagueClientUx.exe";
#[cfg(target_os = "macos")]
pub const CLIENT_PROCESS_NAME: &str = "LeagueClientUx";

#[cfg(target_os = "windows")]
pub const GAME_PROCESS_NAME: &str = "League of Legends.exe";
#[cfg(target_os = "macos")]
pub const GAME_PROCESS_NAME: &str = "League of Legends";

// These ONLY exist so that this will compile in a dev env
// Linux support will not be re-added unless someone comes
// up with a solution to running the game on Linux natively
#[cfg(debug_assertions)]
#[cfg(target_os = "linux")]
pub(crate) const CLIENT_PROCESS_NAME: &str = "";
#[cfg(debug_assertions)]
#[cfg(target_os = "linux")]
pub(crate) const GAME_PROCESS_NAME: &str = "";

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
///
pub fn get_running_client(
    client_process_name: &str,
    game_process_name: &str,
    force_lock_file: bool,
) -> Result<(String, String), Error> {
    // If we always read the lock file, we never need to get the command line of the process
    let cmd = if force_lock_file {
        sysinfo::UpdateKind::Never
    } else {
        sysinfo::UpdateKind::OnlyIfNotSet
    };
    // No matter what, the path to the process is required
    let refresh_kind = ProcessRefreshKind::new()
        .with_exe(sysinfo::UpdateKind::OnlyIfNotSet)
        .with_cmd(cmd);

    // Get the current list of processes
    let system = System::new_with_specifics(
        // This creates a new instance of `system` every time, so this only
        //  needs to be updated if it's not set
        RefreshKind::new().with_processes(refresh_kind),
    );

    // Is the client running, or is it the game?
    let mut client = false;

    // Iterate through all the processes, using .values() because
    // We don't need the PID. Look for a process with the same name
    // as the constant for that platform, otherwise return an error.
    let process = system
        .processes()
        .values()
        .find(|process| {
            // Get the name of the process
            let name = process.name();
            // If it matches the name of the client,
            // set the flag, and return it
            if name == client_process_name {
                client = true;
                client
            } else {
                name == game_process_name
            }
        })
        .ok_or_else(|| Error::LCUProcessNotRunning)?;

    // Move these to an earlier scope to avoid an allocation
    // And deduplicate some code later on
    let lock_file: String;
    let port: &str;
    let auth: &str;

    if client && !force_lock_file {
        let cmd = process.cmd();
        // Assuming the order doesn't change (which I haven't seen it do)
        // we can avoid a second iteration over the cmd args
        let mut iter = cmd.iter();

        // Look for an auth key to put inside the command line, otherwise return an error.
        auth = iter
            .find_map(|s| s.strip_prefix("--remoting-auth-token="))
            .ok_or_else(|| Error::AuthTokenNotFound)?;

        // Look for a port to connect to the LCU with, otherwise return an error.
        port = iter
            .find_map(|s| s.strip_prefix("--app-port="))
            .ok_or_else(|| Error::PortNotFound)?;
    } else {
        // We have to walk back twice to get the path of the lock file relative to the path of the game
        // This can only be None on Linux according to the docs, so we should be fine everywhere else
        let path = process.exe().ok_or_else(|| Error::LockFileNotFound)?;
        let dir = path.parent().ok_or_else(|| Error::LockFileNotFound)?;
        // Sadly, we're relying on how the client structures things here
        // Walking back a whole folder in order to get the lock file
        let base_dir = if client {
            // If it IS the client, we're in the right dir
            dir
        } else {
            // Otherwise it is the game, and we need to go back once
            dir.parent().ok_or_else(|| Error::LockFileNotFound)?
        };

        // Read the lock file, putting the value in a higher scope
        lock_file = std::fs::read_to_string(base_dir.join("lockfile")).map_err(Error::StdIo)?;

        // Split the lock file on `:` which separates the different fields
        // Because lock_file is from a higher scope, we can split the string here
        // and return two string references later on
        let mut split = lock_file.split(':');

        // Get the 3rd field, which should be the port
        port = split.nth(2).ok_or_else(|| Error::PortNotFound)?;
        // We moved the cursor, so the fourth element is the very next one
        // Which should be the auth string
        auth = split.next().ok_or_else(|| Error::AuthTokenNotFound)?;
    }

    // The auth header has to be base64 encoded, so that's happens here
    let auth_header = ENCODER.encode(format!("riot:{auth}"));

    // Format the port and header so that they can be used as headers
    // For the LCU API
    Ok((format!("127.0.0.1:{port}"), format!("Basic {auth_header}",)))
}

#[cfg(test)]
mod tests {
    use super::{get_running_client, CLIENT_PROCESS_NAME, GAME_PROCESS_NAME};
    use sysinfo::{ProcessRefreshKind, RefreshKind, System};

    #[ignore = "This is only needed for testing, and doesn't need to be run all the time"]
    #[test]
    fn test_process_info() {
        let (port, pass) =
            get_running_client(GAME_PROCESS_NAME, CLIENT_PROCESS_NAME, false).unwrap();
        println!("{port} {pass}");
    }

    #[ignore = "This is only needed for testing, and doesn't need to be run all the time"]
    #[test]
    fn test_process_args() {
        // No matter what, the path to the process is required
        let refresh_kind = ProcessRefreshKind::new()
            .with_cwd(sysinfo::UpdateKind::OnlyIfNotSet)
            .with_root(sysinfo::UpdateKind::OnlyIfNotSet)
            .with_exe(sysinfo::UpdateKind::OnlyIfNotSet)
            .with_cmd(sysinfo::UpdateKind::OnlyIfNotSet);

        // Get the current list of processes
        let system = System::new_with_specifics(
            // This creates a new instance of `system` every time, so this only
            //  needs to be updated if it's not set
            RefreshKind::new().with_processes(refresh_kind),
        );

        let process = system
            .processes()
            .values()
            .find(|process| process.name() == GAME_PROCESS_NAME)
            .unwrap();

        println!("{:?}", process.exe());
        println!("{:?}", process.root());
        println!("{:?}", process.cmd());
        println!("{:?}", process.cwd());
        println!("{:?}", process.environ());

        let parent = process.parent().unwrap();

        let process = system.process(parent).unwrap();

        println!("{process:?}");
    }
}

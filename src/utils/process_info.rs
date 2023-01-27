use crate::Error;

use super::encoder::encode;

#[cfg(target_os = "linux")]
const TARGET_COMMAND: &str = "ps -Af | grep LeagueClientUx.";

#[cfg(target_os = "macos")]
const TARGET_COMMAND: &str = "ps -Af | grep LeagueClientUx";

/// Runs a wmic command on windows to get info on the current running clinet, if one exists
#[cfg(target_os = "windows")]
pub(crate) fn get_running_client() -> Result<String, Error> {
    std::process::Command::new("wmic")
        .args([
            "PROCESS",
            "WHERE",
            "name='LeagueClientUx.exe'",
            "GET",
            "commandline",
        ])
        .output()
        .map_or(Err(Error::CannotLaunchTerminal), |value| {
            // If the client is not running, the errors section will not be empty
            // And the output section will be nothing but \r
            if !value.stderr.is_empty() {
                Err(Error::LCUProcessNotRunning)
            } else {
                // This is already checked before hand, so it can be unwrapped
                Ok(String::from_utf8(value.stdout).unwrap())
            }
        })
}

/// Runes a PS command on a POSIX complient shell to get the info on the running
/// Client for that platform, assuming one exists, and that platform is supported
#[cfg(not(target_os = "windows"))]
pub(crate) fn get_running_client() -> Result<String, Error> {
    std::process::Command::new("/bin/sh")
        .args(["-c", TARGET_COMMAND])
        .output()
        .map_or(Err(Error::CannotLaunchTerminal), |value| {
            // This always returns a String, so this can be unwrapped
            let string = String::from_utf8(value.stdout).unwrap();
            // Checks that the client API is actautally running
            if !string.contains("--client-config-url=") {
                Err(Error::LCUProcessNotRunning)
            } else {
                Ok(string)
            }
        })
}

/// Uses string splitting in order to get the port and auth key from the currently running client
/// On windows this uses quotes, because it returns every arg wrapped in quotes
#[cfg(target_os = "windows")]
pub(crate) fn get_port_and_auth() -> Result<(String, String), Error> {
    let process = get_running_client()?;
    let Some(port) = process.split('"').find_map(|s| s.strip_prefix("--app-port=")) else {
		return Err(Error::PortNotFound);
	};
    let Some(auth) = process.split('"').find_map(|s| s.strip_prefix("--remoting-auth-token=")) else {
    	return Err(Error::AuthTokenNotFound);
  	};
    Ok((port.to_owned(), encode(format!("riot:{}", auth))))
}

/// Uses string splitting in order to get the port and auth key from the currently running client
/// On anything using a PS command this uses whitespace, as args are wrapped in whitespace instead
#[cfg(not(target_os = "windows"))]
pub(crate) fn get_port_and_auth() -> Result<(String, String), Error> {
    let process = get_running_client()?;
    let Some(port) = process.split_whitespace().find_map(|s| s.strip_prefix("--app-port=")) else {
      return Err(Error::PortNotFound);
    };
    let Some(auth) = process.split_whitespace().find_map(|s| s.strip_prefix("--remoting-auth-token=")) else {
      return Err(Error::AuthTokenNotFound);
    };
    Ok((port.to_owned(), encode(format!("riot:{}", auth))))
}

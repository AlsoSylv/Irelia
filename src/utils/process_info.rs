use crate::ProcessInfoErrors;

use super::encoder::encode;

#[cfg(target_os = "linux")]
const TARGET_PROCESS: &str = "LeagueClientUx.";

#[cfg(target_os = "macos")]
const TARGET_PROCESS: &str = "LeagueClientUx";

#[cfg(target_os = "windows")]
pub(crate) fn get_running_client() -> Result<String, ProcessInfoErrors> {
    std::process::Command::new("wmic")
        .args([
            "PROCESS",
            "WHERE",
            "name='LeagueClientUx.exe'",
            "GET",
            "commandline",
        ])
        .output()
        .map_or(Err(ProcessInfoErrors::CannotLaunchTerminal), |value| {
            if !value.stderr.is_empty() {
                Err(ProcessInfoErrors::LCUProcessNotRunning)
            } else {
                Ok(String::from_utf8(value.stdout).unwrap())
            }
        })
}

#[cfg(not(target_os = "windows"))]
pub(crate) fn get_running_client() -> Result<String, ProcessInfoErrors> {
    std::process::Command::new("/bin/sh")
        .args(["-c", TARGET_COMMAND])
        .output()
        .map_or(Err(ProcessInfoErrors::CannotLaunchTerminal), |value| {
            let string = String::from_utf8(value.stdout).unwrap();
            if !string.contains("--client-config-url=") {
                Err(ProcessInfoErrors::LCUProcessNotRunning)
            } else {
                Ok(string)
            }
        })
}

#[cfg(target_os = "windows")]
pub(crate) fn get_port_and_auth() -> Result<(String, String), ProcessInfoErrors> {
    let process = get_running_client()?;
    let Some(port) = process.split('"').find_map(|s| s.strip_prefix("--app-port=")) else {
		panic!("No port!")
	};
    let Some(auth) = process.split('"').find_map(|s| s.strip_prefix("--remoting-auth-token=")) else {
    	panic!("No auth!");
  	};
    Ok((port.to_owned(), encode(format!("riot:{}", auth))))
}

#[cfg(not(target_os = "windows"))]
pub(crate) fn get_port_and_auth() -> Result<(String, String), ProcessInfoErrors> {
    let process = get_running_client()?;
    let Some(port) = process.split_whitespace().find_map(|s| s.strip_prefix("--app-port=")) else {
      return Err(ProcessInfoErrors::PortNotFound);
    };
    let Some(auth) = process.split_whitespace().find_map(|s| s.strip_prefix("--remoting-auth-token=")) else {
      return Err(ProcessInfoErrors::AuthTokenNotFound);
    };
    Ok((port.to_owned(), encode(format!("riot:{}", auth))))
}

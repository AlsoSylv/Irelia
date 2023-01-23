use regex::Regex;
use sysinfo::{ProcessExt, System, SystemExt};

use crate::ProcessInfoErrors;

use super::encoder::encode;

#[cfg(target_os = "windows")]
const TARGET_PROCESS: &str = "LeagueClientUx.exe";
#[cfg(target_os = "linux")]
const TARGET_PROCESS: &str = "LeagueClientUx.";
#[cfg(target_os = "macos")]
const TARGET_PROCESS: &str = "LeagueClientUx";

pub(crate) fn get_running_client() -> Result<String, ProcessInfoErrors> {
    let mut sys = System::new_all();
    sys.refresh_all();
    sys.processes()
        .values()
        .find(|process| process.name() == TARGET_PROCESS)
        .map_or(Err(ProcessInfoErrors::LCUProcessNotRunning), |process| {
            Ok(process.cmd().join(" "))
        })
}

pub(crate) fn get_port_and_auth(process: String) -> Result<(String, String), ProcessInfoErrors> {
    let port_regex = Regex::new(r"--app-port=([0-9]*)").unwrap();
    let password_regex = Regex::new(r"--remoting-auth-token=([\w-]*)").unwrap();

    let port =
        port_regex
            .captures(&process)
            .map_or(Err(ProcessInfoErrors::PortNotFound), |capture| {
                capture
                    .get(1)
                    .map_or(Err(ProcessInfoErrors::PortNotFound), |matched| {
                        Ok(matched.as_str())
                        // I don't think this actually needs to be a u32, and having it as a String should be just fine
                        // .parse::<u32>()
                        // .map_or(Err(Pr), |port| Ok(port))
                    })
            })?;

    let password = password_regex.captures(&process).map_or(
        Err(ProcessInfoErrors::AuthTokenNotFound),
        |captures| {
            captures
                .get(1)
                .map_or(Err(ProcessInfoErrors::AuthTokenNotFound), |token| {
                    Ok(token.as_str())
                })
        },
    )?;

    Ok((port.to_owned(), encode(&format!("riot:{}", password))))
}

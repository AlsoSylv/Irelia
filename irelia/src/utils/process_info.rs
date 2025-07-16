//! Constants, as well as the schema for the lock file can be found here
//! <https://hextechdocs.dev/getting-started-with-the-lcu-api/>

//! This module also contains a list of constants for the different names
//! of the processes for `OSX`, and `Windows`

use irelia_encoder::Encoder;
use std::fmt::{Display, Formatter};
use std::io::Read;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::num::ParseIntError;
use std::path::Path;
use std::str::FromStr;
use sysinfo::{ProcessRefreshKind, RefreshKind, System};

// Linux is unplayable, the constants here are only defined so the docs build
#[cfg(target_os = "windows")]
pub const CLIENT_PROCESS_NAME: &str = "LeagueClientUx.exe";
#[cfg(target_os = "macos")]
pub const CLIENT_PROCESS_NAME: &str = "LeagueClientUx";

#[cfg(target_os = "windows")]
pub const GAME_PROCESS_NAME: &str = "League of Legends.exe";
#[cfg(target_os = "macos")]
pub const GAME_PROCESS_NAME: &str = "League of Legends";

/// const copy of the encoder
pub(crate) const ENCODER: Encoder = Encoder::new();

#[cfg(all(docsrs, target_os = "linux"))]
/// Constant for the game process name, `League of Legends.exe` on Windows and `League of Legends` on MacOS
pub const GAME_PROCESS_NAME: &str = "";
#[cfg(all(docsrs, target_os = "linux"))]
/// Constant for the client process name, `LeagueClientUx.exe` on Windows and `LeagueClientUx` on MacOS
pub const CLIENT_PROCESS_NAME: &str = "";

const NOT_RUNNING: Error = Error::new(
    ErrorKind::NotRunning,
    "neither the game or client process were running",
);

const PORT_NOT_FOUND: Error = Error::new(ErrorKind::PortNotFound, "port was not found");

const AUTH_NOT_FOUND: Error = Error::new(ErrorKind::AuthTokenNotFound, "auth token was not found");

const LOCK_FILE_NOT_FOUND: Error = Error::new(
    ErrorKind::LockFileNotFound,
    "Did not follow the typical install structure",
)
.set_lockfile_error(true);

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
/// # Panics
/// Panics if the lockfile length is greater than `usize::MAX`, but this should be impossible
pub fn get_running_client<T>(
    client_process_name: &str,
    game_process_name: &str,
    force_lock_file: bool,
    force_directory: Option<&Path>,
) -> Result<(SocketAddrV4, Result<T, T::Err>), Error>
where
    T: FromStr,
{
    const RIOT_PREFIX: &[u8] = b"riot:";
    const BASIC_PREFIX: &[u8] = b"Basic ";

    // If we always read the lock file, we never need to get the command line of the process
    let cmd = if force_lock_file {
        sysinfo::UpdateKind::Never
    } else {
        sysinfo::UpdateKind::OnlyIfNotSet
    };

    let exe = if force_directory.is_some() {
        sysinfo::UpdateKind::Never
    } else {
        sysinfo::UpdateKind::OnlyIfNotSet
    };

    // No matter what, the path to the process is required
    let refresh_kind = ProcessRefreshKind::nothing().with_exe(exe).with_cmd(cmd);

    // Get the current list of processes
    let system = System::new_with_specifics(
        // This creates a new instance of `system` every time, so this only
        //  needs to be updated if it's not set
        RefreshKind::nothing().with_processes(refresh_kind),
    );

    // Is the client running, or is it the game?
    let mut client = false;

    // Iterate through all the processes, using .values() because
    // We don't need the PID. Look for a process with the same name
    // as the constant for that platform, otherwise return an error.
    let process = if let Some(force_directory) = force_directory {
        Err(force_directory)
    } else {
        Ok(system
            .processes()
            .values()
            .find(|process| {
                client = process.name() == client_process_name;
                client || (process.name() == game_process_name)
            })
            .ok_or(NOT_RUNNING)?)
    };

    // The size of the lock file is typically 53kb, but I am overallocating to stay cautious
    let mut lock_file = [0; 60];
    let [port, auth] = match (process, client, force_lock_file) {
        (Ok(process), true, false) => pull_client_info(process)?,
        _ => read_lock_file(&mut lock_file, client, &process)?,
    };

    // Prevent the pre-encoded base64 string from allocating
    let pre_encoded_buffer_len = auth.len() + RIOT_PREFIX.len();
    // `22 + RIOT_PREFIX.len()` is 27, which is what I've observed to almost always be the length
    let buffer: &mut [u8] = if pre_encoded_buffer_len > 22 + RIOT_PREFIX.len() {
        &mut vec![0; pre_encoded_buffer_len].into_boxed_slice()
    } else {
        &mut [0; 22 + RIOT_PREFIX.len()]
    };

    buffer[..RIOT_PREFIX.len()].copy_from_slice(RIOT_PREFIX);
    buffer[RIOT_PREFIX.len()..auth.len() + RIOT_PREFIX.len()].copy_from_slice(auth.as_bytes());

    let auth_header_len = pre_encoded_buffer_len.div_ceil(3) * 4;
    // 27 / 3 * 4 = 36 + 6 for the "Basic " prefix
    let auth_header_buffer: &mut [u8] = if auth_header_len > 36 {
        &mut vec![b'='; auth_header_len + BASIC_PREFIX.len()].into_boxed_slice()
    } else {
        &mut [b'='; 36 + BASIC_PREFIX.len()]
    };

    auth_header_buffer[..BASIC_PREFIX.len()].copy_from_slice(BASIC_PREFIX);

    // The auth header has to be base64 encoded, so that's happens here
    ENCODER.internal_encode(buffer, &mut auth_header_buffer[BASIC_PREFIX.len()..]);

    let port: u16 = port.parse().map_err(|err: ParseIntError| {
        Error::new_string(ErrorKind::PortNotFound, err.to_string())
    })?;

    let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, port);

    let auth_header_buffer = std::str::from_utf8(auth_header_buffer)?;

    // Format the port and header so that they can be used as headers
    // For the LCU API
    let res = T::from_str(auth_header_buffer);
    Ok((addr, res))
}

fn pull_client_info(process: &sysinfo::Process) -> Result<[&str; 2], Error> {
    // The port and auth should always be ASCII, as they are a number and a B64 buffer
    let cmd = process.cmd().iter().filter_map(|os_str| os_str.to_str());
    // Use a variable in a higher scope to make sure that port and auth get initialized
    let mut scoped_auth = None;
    let mut scoped_port = None;

    // Iterate through the command args, updating the scoped values as we go
    for s in cmd {
        if scoped_auth.is_some() && scoped_port.is_some() {
            break;
        }

        if scoped_auth.is_none() {
            scoped_auth = s.strip_prefix("--remoting-auth-token=");
        }

        if scoped_port.is_none() {
            scoped_port = s.strip_prefix("--app-port=");
        }
    }

    // Check that we found a port and auth key, otherwise error
    Ok([
        scoped_port.ok_or(PORT_NOT_FOUND)?,
        scoped_auth.ok_or(AUTH_NOT_FOUND)?,
    ])
}

fn read_lock_file<'a>(
    lock_file: &'a mut [u8; 60],
    client: bool,
    process: &Result<&sysinfo::Process, &Path>,
) -> Result<[&'a str; 2], Error> {
    let dir = match process {
        Err(path) => *path,
        Ok(process) => {
            // We have to walk back twice to get the path of the lock file relative to the path of the game
            // This can only be None on Linux according to the docs, so we should be fine everywhere else
            let path = process.exe().ok_or(LOCK_FILE_NOT_FOUND)?;

            let mut dir = path.parent().ok_or(LOCK_FILE_NOT_FOUND)?;
            // Sadly, we're relying on how the client structures things here
            // Walking back a whole folder in order to get the lock file
            if !client {
                // If we're looking at the game and not the client, we need to walk back once more
                dir = dir.parent().ok_or(LOCK_FILE_NOT_FOUND)?;
            }

            dir
        }
    };

    let mut file = std::fs::File::open(dir.join("lockfile"))?;
    // This len shouldn't be more than a few bytes
    let len = file
        .metadata()?
        .len()
        .try_into()
        .expect("This file is always ~60 bytes");

    // Read the file initially
    let mut read = file.read(lock_file)?;

    // Make sure the entire file was read, though it is so small I can't imagine it wouldn't be
    while read != len {
        read += file.read(&mut lock_file[read..])?;
    }

    // Make sure that we're not over reading into 0's
    let lock_file = std::str::from_utf8(&lock_file[..len])?;

    // Split the lock file on `:` which separates the different fields
    // Because lock_file is from a higher scope, we can split the string here
    // and return two string references later on
    let mut split = lock_file.split(':');

    Ok([
        // Get the 3rd field, which should be the port
        split
            .nth(2)
            .ok_or(PORT_NOT_FOUND.set_lockfile_error(true))?,
        // We moved the cursor, so the fourth element is the very next one
        // Which should be the auth string
        split
            .next()
            .ok_or(AUTH_NOT_FOUND.set_lockfile_error(true))?,
    ])
}

#[derive(Debug, Clone)]
/// Error retaining to getting the auth key and url for the LCU
pub struct Error {
    kind: ErrorKind,
    message: std::borrow::Cow<'static, str>,
    lock_file: bool,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for Error {}

impl Error {
    const fn new(kind: ErrorKind, message: &'static str) -> Self {
        Self {
            kind,
            message: std::borrow::Cow::Borrowed(message),
            lock_file: false,
        }
    }

    const fn new_string(kind: ErrorKind, message: String) -> Self {
        Self {
            kind,
            message: std::borrow::Cow::Owned(message),
            lock_file: false,
        }
    }

    const fn set_lockfile_error(mut self, lock_fie_error: bool) -> Self {
        self.lock_file = lock_fie_error;
        self
    }

    #[must_use]
    pub const fn is_lockfile_error(&self) -> bool {
        self.lock_file
    }

    #[must_use]
    /// Returns true if it's an IO error, false otherwise
    pub const fn is_io_error(&self) -> bool {
        matches!(self.kind, ErrorKind::Io(_))
    }

    #[must_use]
    pub fn kind(&self) -> ErrorKind {
        self.kind.clone()
    }

    #[must_use]
    pub fn reason(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
/// What caused the error
pub enum ErrorKind {
    Io(std::io::ErrorKind),
    LockFileNotFound,
    AuthTokenNotFound,
    PortNotFound,
    NotRunning,
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self {
            kind: ErrorKind::Io(value.kind()),
            message: value.to_string().into(),
            lock_file: true,
        }
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(_: std::str::Utf8Error) -> Self {
        const {
            Self::new(
                ErrorKind::Io(std::io::ErrorKind::InvalidData),
                "stream did not contain valid UTF-8",
            )
            .set_lockfile_error(true)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CLIENT_PROCESS_NAME, GAME_PROCESS_NAME, get_running_client};
    use http::HeaderValue;
    use sysinfo::{ProcessRefreshKind, RefreshKind, System};

    #[ignore = "This is only needed for testing, and doesn't need to be run all the time"]
    #[test]
    fn test_process_info() {
        let (port, pass): (_, Result<HeaderValue, _>) =
            get_running_client(CLIENT_PROCESS_NAME, GAME_PROCESS_NAME, true, None).unwrap();
        println!("{port} {pass:?}");
    }

    #[ignore = "This is only needed for testing, and doesn't need to be run all the time"]
    #[test]
    fn test_process_args() {
        // No matter what, the path to the process is required
        let refresh_kind = ProcessRefreshKind::nothing()
            .with_cwd(sysinfo::UpdateKind::OnlyIfNotSet)
            .with_root(sysinfo::UpdateKind::OnlyIfNotSet)
            .with_exe(sysinfo::UpdateKind::OnlyIfNotSet)
            .with_cmd(sysinfo::UpdateKind::OnlyIfNotSet);

        // Get the current list of processes
        let system = System::new_with_specifics(
            // This creates a new instance of `system` every time, so this only
            //  needs to be updated if it's not set
            RefreshKind::nothing().with_processes(refresh_kind),
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

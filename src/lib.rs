//! Irelia is a wrapper around the LoL native APIs, with a focus on modularity and compile size
//! This crate has support for Windows, Linux, and MacOS, all of which have been tested to varying degrees

#[cfg(feature = "in_game")]
/// The in_game module has support for LoLs in game API, and returns all data as structs that much the
/// Current spec release by Riot, more info can be found here: <https://developer.riotgames.com/docs/lol#game-client-api>
pub mod in_game;
#[cfg(feature = "rest")]
/// The rest module provides support for the LCU Rest API, and allows passing custom return types to each method
/// As long as they implement serde::Deserialize, more info can be found here: <https://hextechdocs.dev/getting-started-with-the-lcu-api/>
pub mod rest;
mod utils;
#[cfg(feature = "ws")]
/// The ws module provides support for the LCU Web Socket, and returns all data as `Value`
/// More info can be found here: <https://hextechdocs.dev/getting-started-with-the-lcu-websocket/>
///
/// This is a high level implementation of the LCU websocket, which managesthe event loop itself
/// Methods to subscribe, unsubscribe, and terminate the event loop are provided
pub mod ws;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
/// Custom errors for the LCU
pub enum Error {
    /// Expected or input type are incorrect
    FailedParseJson = 10,
    /// The LCU stopped running
    LCUStoppedRunning = 11,
    #[cfg(feature = "in_game")]
    /// The game stopped running
    LeagueStoppedRunning = 12,
    /// The following request is invalid
    InvalidRequest = 13,
    /// The request body is invalid
    InvalidBody = 14,
    /// The LCU was never running
    LCUProcessNotRunning = 15,
    /// Could not locate port for the LCU
    PortNotFound = 16,
    /// The sub process could not be spawned
    CannotLaunchTerminal = 17,
    /// Auth token for the LCU could not be found
    AuthTokenNotFound = 18,
}

#[cfg(feature = "c")]
mod c_lcu {
    use std::ffi::{c_char, c_int, CStr, CString};

    use futures_util::Future;
    use tokio::runtime::Runtime;

    use crate::{rest::LCUClient, Error};

    #[repr(C)]
    pub struct LcuResponse {
        pub json: *const c_char,
        pub error: c_int,
    }

    #[repr(C)]
    pub struct NewLCU<'a> {
        pub client: *mut LCU<'a>,
        pub error: c_int,
    }

    pub struct LCU<'a> {
        client: LCUClient<'a>,
        rt: Runtime,
    }

    impl LCU<'_> {
        fn new<'a>() -> NewLCU<'a> {
            let client = LCUClient::new();
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Pain");

            match client {
                Ok(client) => NewLCU {
                    client: Box::into_raw(Box::new(LCU { client, rt })),
                    error: 0,
                },
                Err(err) => NewLCU {
                    client: std::ptr::null_mut(),
                    error: err as c_int,
                },
            }
        }
    }

    fn lcu_generic(
        fut: impl Future<Output = Result<Option<CString>, Error>>,
        rt: &Runtime,
    ) -> LcuResponse {
        let result = rt.block_on(fut);

        match result {
            Ok(result) => match result {
                Some(c_string) => LcuResponse {
                    json: c_string.as_ptr(),
                    error: 0,
                },
                None => LcuResponse {
                    json: std::ptr::null(),
                    error: 0,
                },
            },
            Err(err) => LcuResponse {
                json: std::ptr::null(),
                error: err as c_int,
            },
        }
    }

    #[no_mangle]
    pub extern "C" fn lcu_new<'a>() -> NewLCU<'a> {
        LCU::new()
    }

    #[no_mangle]
    pub unsafe extern "C" fn lcu_get<'a>(client: *mut LCU, endpoint: *const c_char) -> LcuResponse {
        let client = &*client;
        let endpoint = CStr::from_ptr(endpoint).to_string_lossy();
        let fut = client.client.get::<CString>(&endpoint);
        lcu_generic(fut, &client.rt)
    }

    #[no_mangle]
    pub unsafe extern "C" fn lcu_post<'a>(
        client: *mut LCU,
        endpoint: *const c_char,
        body: *const c_char,
    ) -> LcuResponse {
        let client = &*client;
        let endpoint = CStr::from_ptr(endpoint).to_string_lossy();
        let body = CStr::from_ptr(body).to_string_lossy();
        let fut = client.client.post::<_, CString>(&endpoint, &body);
        lcu_generic(fut, &client.rt)
    }

    #[no_mangle]
    pub unsafe extern "C" fn lcu_put<'a>(
        client: *mut LCU,
        endpoint: *const c_char,
        body: *const c_char,
    ) -> LcuResponse {
        let client = &*client;
        let endpoint = CStr::from_ptr(endpoint).to_string_lossy();
        let body = CStr::from_ptr(body).to_string_lossy();
        let fut = client.client.put::<_, CString>(&endpoint, &body);
        lcu_generic(fut, &client.rt)
    }

    #[no_mangle]
    pub unsafe extern "C" fn lcu_delete<'a>(client: *mut LCU, endpoint: *const c_char) -> LcuResponse {
        let client = &*client;
        let endpoint = CStr::from_ptr(endpoint).to_string_lossy();
        let fut = client.client.delete::<CString>(&endpoint);
        lcu_generic(fut, &client.rt)
    }

    #[no_mangle]
    pub unsafe extern "C" fn lcu_head<'a>(client: *mut LCU, endpoint: *const c_char) -> LcuResponse {
        let client = &*client;
        let endpoint = CStr::from_ptr(endpoint).to_string_lossy();
        let fut = client.client.head::<CString>(&endpoint);
        lcu_generic(fut, &client.rt)
    }

    #[no_mangle]
    pub extern "C" fn lcu_drop<'a>(client: *mut LCU) {
        let client = unsafe { Box::from_raw(client) };
        drop(client)
    }
}

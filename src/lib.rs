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
    FailedParseJson = 0,
    /// The LCU stopped running
    LCUStoppedRunning = 1,
    #[cfg(feature = "in_game")]
    /// The game stopped running
    LeagueStoppedRunning = 2,
    /// The following request is invalid
    InvalidRequest = 3,
    /// The request body is invalid
    InvalidBody = 4,
    /// The LCU was never running
    LCUProcessNotRunning = 5,
    /// Could not locate port for the LCU
    PortNotFound = 6,
    /// The sub process could not be spawned
    CannotLaunchTerminal = 7,
    /// Auth token for the LCU could not be found
    AuthTokenNotFound = 8,
}

#[cfg(feature = "c")]
mod c_lcu {
    use std::ffi::{c_char, CStr, CString};

    use futures_util::Future;
    use tokio::runtime::Runtime;

    use crate::{rest::LCUClient, Error};

    pub struct LCUC<'a> {
        client: LCUClient<'a>,
        rt: Runtime,
    }

    impl LCUC<'_> {
        fn new<'a>() -> Result<LCUC<'a>, Error> {
            let client = LCUClient::new()?;
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Pain");

            Ok(LCUC { client, rt })
        }
    }

    fn lcu_generic(
        fut: impl Future<Output = Result<Option<CString>, Error>>,
        rt: &Runtime,
    ) -> *const Result<*const c_char, Error> {
        let result = rt.block_on(fut);

        match result {
            Ok(result) => match result {
                Some(c_string) => &Ok(c_string.as_ptr()),
                None => &Ok(std::ptr::null()),
            },
            Err(err) => &Err(err),
        }
    }

    #[no_mangle]
    pub extern "C" fn lcu_new<'a>() -> *mut Result<LCUC<'a>, Error> {
        Box::into_raw(Box::new(LCUC::new()))
    }

    #[no_mangle]
    pub unsafe extern "C" fn lcu_get<'a>(
        client: *mut LCUC,
        endpoint: *mut c_char,
    ) -> *const Result<*const c_char, Error> {
        let client = &*client;
        let endpoint = CStr::from_ptr(endpoint).to_string_lossy();
        let fut = client.client.get::<CString>(&endpoint);
        lcu_generic(fut, &client.rt)
    }

    #[no_mangle]
    pub unsafe extern "C" fn lcu_post<'a>(
        client: *mut LCUC,
        endpoint: *mut c_char,
        body: *mut c_char,
    ) -> *const Result<*const c_char, Error> {
        let client = &*client;
        let endpoint = CStr::from_ptr(endpoint).to_string_lossy();
        let body = CStr::from_ptr(body).to_string_lossy();
        let fut = client.client.post::<_, CString>(&endpoint, &body);
        lcu_generic(fut, &client.rt)
    }

    #[no_mangle]
    pub unsafe extern "C" fn lcu_put<'a>(
        client: *mut LCUC,
        endpoint: *mut c_char,
        body: *mut c_char,
    ) -> *const Result<*const c_char, Error> {
        let client = &*client;
        let endpoint = CStr::from_ptr(endpoint).to_string_lossy();
        let body = CStr::from_ptr(body).to_string_lossy();
        let fut = client.client.put::<_, CString>(&endpoint, &body);
        lcu_generic(fut, &client.rt)
    }

    #[no_mangle]
    pub unsafe extern "C" fn lcu_delete<'a>(
        client: *mut LCUC,
        endpoint: *mut c_char,
    ) -> *const Result<*const c_char, Error> {
        let client = &*client;
        let endpoint = CStr::from_ptr(endpoint).to_string_lossy();
        let fut = client.client.delete::<CString>(&endpoint);
        lcu_generic(fut, &client.rt)
    }

    #[no_mangle]
    pub unsafe extern "C" fn lcu_head<'a>(
        client: *mut LCUC,
        endpoint: *mut c_char,
    ) -> *const Result<*const c_char, Error> {
        let client = &*client;
        let endpoint = CStr::from_ptr(endpoint).to_string_lossy();
        let fut = client.client.head::<CString>(&endpoint);
        lcu_generic(fut, &client.rt)
    }

    #[no_mangle]
    pub extern "C" fn lcu_drop<'a>(client: *mut LCUC) {
        let client = unsafe { Box::from_raw(client) };
        drop(client)
    }
}

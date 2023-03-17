use std::ffi::{c_char, c_int, CStr, CString};

use futures_util::StreamExt;
use tokio::runtime::Runtime;

use crate::ws::{EventType, LCUWebSocket};

#[derive(Debug)]
#[repr(C)]
/// Holds JSON from the response, error can
/// be 0 and json can be null if no event
/// has been sent back
pub struct LcuWsRes {
    pub json: *mut c_char,
    pub error: c_int,
}

#[repr(C)]
/// Handle to the LCU websocket
pub struct NewWS {
    pub client: *mut LcuWS,
    pub error: c_int,
}

/// Opaque type that stores the client
/// and the tokio runtime
pub struct LcuWS {
    client: LCUWebSocket,
    rt: Runtime,
}

#[repr(C)]
#[allow(dead_code)]
/// Event type you're requesting from the socket
pub enum EventTypeC {
    JsonApiEvent,
    LcdEvent,
    JsonApiEventCallback,
    LcdEventCallback,
}

#[repr(C)]
/// Event to send to the socket, endpoint is ignored
/// if you send JsonApiEvent and LcdEvent, and cannot
/// be null otherwise
pub struct Event {
    event: EventTypeC,
    endpoint: *const c_char,
}

impl Event {
    unsafe fn real_event_type(&self) -> EventType {
        match self.event {
            EventTypeC::JsonApiEvent => EventType::OnJsonApiEvent,
            EventTypeC::LcdEvent => EventType::OnLcdEvent,
            EventTypeC::JsonApiEventCallback => {
                let ptr = self.endpoint;
                let string = CStr::from_ptr(ptr);
                let str = string.to_str().unwrap().to_owned();
                EventType::OnJsonApiEventCallback(str)
            }
            EventTypeC::LcdEventCallback => {
                let ptr = self.endpoint;
                let string = CStr::from_ptr(ptr);
                let str = string.to_str().unwrap().to_owned();
                EventType::OnLcdEventCallback(str)
            }
        }
    }
}

impl NewWS {
    fn new() -> Self {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("PAIN");
        let client = rt.block_on(LCUWebSocket::new());
        match client {
            Ok(client) => Self {
                client: Box::into_raw(Box::new(LcuWS { client, rt })),
                error: 0,
            },
            Err(err) => Self {
                client: std::ptr::null_mut(),
                error: err as c_int,
            },
        }
    }
}

/// Creates a new handle for the web socket
#[no_mangle]
pub unsafe extern "C" fn new_ws() -> NewWS {
    NewWS::new()
}

/// Subscribes to a new web socket event
#[no_mangle]
pub unsafe extern "C" fn subscribe(client: *mut LcuWS, event: Event) {
    let client = &mut *client;
    client.client.subscribe(event.real_event_type());
}

/// Unsubscribes from a web socket event
#[no_mangle]
pub unsafe extern "C" fn unsubscribe(client: *mut LcuWS, event: Event) {
    let client = &mut *client;
    client.client.unsubscribe(event.real_event_type());
}

/// Requests to the event sent by the websocket, returns null
/// if there is no event or if there is an error
#[no_mangle]
pub unsafe extern "C" fn next(client: *mut LcuWS) -> LcuWsRes {
    let client = &mut *client;
    let val = client.rt.block_on(client.client.next());
    match val {
        Some(val) => match val {
            Ok(val) => {
                let str = val.to_string();
                let c_str = CString::new(str).unwrap();
                LcuWsRes {
                    json: c_str.into_raw(),
                    error: 0,
                }
            }
            Err(err) => LcuWsRes {
                json: std::ptr::null_mut(),
                error: err as c_int,
            },
        },
        None => LcuWsRes {
            json: std::ptr::null_mut(),
            error: 0,
        },
    }
}

/// Drops the web socket handle
#[no_mangle]
pub unsafe extern "C" fn drop_ws(client: NewWS) {
    drop(Box::from_raw(client.client))
}

/// Drops the web socket response
#[no_mangle]
pub unsafe extern "C" fn drop_ws_res(res: LcuWsRes) {
    drop(Box::from_raw(res.json));
}

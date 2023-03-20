use std::{
    ffi::{c_char, CStr, CString},
    ptr::NonNull,
};

use futures_util::StreamExt;

use crate::{
    ws::{EventType, LCUWebSocket},
    LcuResponse,
};

use super::runtime::RT;

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

/// Creates a new handle for the web socket
#[no_mangle]
pub unsafe extern "C" fn new_ws(
    client: *mut *mut LCUWebSocket,
    rt: Option<NonNull<RT>>,
) -> LcuResponse {
    let ws_client = rt.unwrap().as_ref().rt.block_on(LCUWebSocket::new());
    match ws_client {
        Ok(ws_client) => {
            *client = Box::into_raw(Box::new(ws_client));
            LcuResponse::Success
        }
        Err(err) => err,
    }
}

/// Subscribes to a new web socket event
#[no_mangle]
pub unsafe extern "C" fn subscribe(client: Option<NonNull<LCUWebSocket>>, event: Event) {
    client.unwrap().as_mut().subscribe(event.real_event_type());
}

/// Unsubscribes from a web socket event
#[no_mangle]
pub unsafe extern "C" fn unsubscribe(client: Option<NonNull<LCUWebSocket>>, event: Event) {
    client
        .unwrap()
        .as_mut()
        .unsubscribe(event.real_event_type());
}

/// Requests to the event sent by the websocket, returns null
/// if there is no event or if there is an error
#[no_mangle]
pub unsafe extern "C" fn next(
    client: Option<NonNull<LCUWebSocket>>,
    rt: Option<NonNull<RT>>,
    json: Option<NonNull<*mut c_char>>,
) -> LcuResponse {
    let client = client.unwrap().as_mut();
    let val = rt.unwrap().as_ref().rt.block_on(client.next());
    match val {
        Some(val) => match val {
            Ok(val) => {
                let str = val.to_string();
                let c_str = CString::new(str).unwrap();
                *json.unwrap().as_ptr() = c_str.into_raw();
                LcuResponse::Success
            }
            Err(err) => err,
        },
        None => {
            *json.unwrap().as_ptr() = std::ptr::null_mut();
            LcuResponse::Success
        }
    }
}

/// Drops the web socket handle
#[no_mangle]
pub unsafe extern "C" fn drop_ws(client: *mut *mut LCUWebSocket) {
    drop(Box::from_raw(*client))
}

/// Drops the web socket response
#[no_mangle]
pub unsafe extern "C" fn drop_ws_res(res: *mut *mut c_char) {
    drop(CString::from_raw(*res));
}

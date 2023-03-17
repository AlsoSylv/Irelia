use std::ffi::{c_char, c_int, CStr, CString};

use futures_util::StreamExt;
use tokio::runtime::Runtime;

use crate::ws::{EventType, LCUWebSocket};

#[derive(Debug)]
#[repr(C)]
pub struct LcuWsRes {
    pub json: *mut c_char,
    pub error: c_int,
}

#[repr(C)]
pub struct NewWS {
    pub client: *mut LcuWS,
    pub error: c_int,
}

pub struct LcuWS {
    client: LCUWebSocket,
    rt: Runtime,
}

#[repr(C)]
#[allow(dead_code)]
pub enum EventTypeC {
    OnJsonApiEvent,
    OnLcdEvent,
    OnJsonApiEventCallback,
    OnLcdEventCallback,
}

#[repr(C)]
pub struct Event {
    event: EventTypeC,
    endpoint: *const c_char,
}

impl Event {
    unsafe fn real_event_type(&self) -> EventType {
        unsafe {
            match self.event {
                EventTypeC::OnJsonApiEvent => EventType::OnJsonApiEvent,
                EventTypeC::OnLcdEvent => EventType::OnLcdEvent,
                EventTypeC::OnJsonApiEventCallback => {
                    let ptr = self.endpoint;
                    let string = CStr::from_ptr(ptr);
                    let str = string.to_str().unwrap().to_owned();
                    EventType::OnJsonApiEventCallback(str)
                }
                EventTypeC::OnLcdEventCallback => {
                    let ptr = self.endpoint;
                    let string = CStr::from_ptr(ptr);
                    let str = string.to_str().unwrap().to_owned();
                    EventType::OnLcdEventCallback(str)
                }
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

#[no_mangle]
pub unsafe extern "C" fn new_ws() -> NewWS {
    NewWS::new()
}

#[no_mangle]
pub unsafe extern "C" fn subscribe(client: *mut LcuWS, event: Event) {
    unsafe {
        let client = &mut *client;
        client.client.subscribe(event.real_event_type());
    }
}

#[no_mangle]
pub unsafe extern "C" fn unsubscribe(client: *mut LcuWS, event: Event) {
    unsafe {
        let client = &mut *client;
        client.client.unsubscribe(event.real_event_type());
    }
}

#[no_mangle]
pub unsafe extern "C" fn next(client: *mut LcuWS) -> LcuWsRes {
    unsafe {
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
}

#[no_mangle]
pub unsafe extern "C" fn drop_ws(client: *mut LcuWS) {
    drop(Box::from_raw(client))
}

#[no_mangle]
pub unsafe extern "C" fn drop_ws_res(res: LcuWsRes) {
    drop(Box::from_raw(res.json));
}

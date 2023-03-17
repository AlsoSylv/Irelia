use std::ffi::{c_char, c_int, CStr, CString};

use futures_util::Future;
use serde_json::Value;
use tokio::runtime::Runtime;

use crate::{rest::LCUClient, Error};

#[derive(Debug)]
#[repr(C)]
pub struct LcuResponse {
    pub json: *mut c_char,
    pub error: c_int,
}

#[repr(C)]
pub struct NewLCU<'a> {
    pub client: *mut Lcu<'a>,
    pub error: c_int,
}

pub struct Lcu<'a> {
    client: LCUClient<'a>,
    rt: Runtime,
}

impl NewLCU<'_> {
    fn new() -> Self {
        let client = LCUClient::new();
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Pain");

        match client {
            Ok(client) => Self {
                client: Box::into_raw(Box::new(Lcu { client, rt })),
                error: 0,
            },
            Err(err) => Self {
                client: std::ptr::null_mut(),
                error: err as c_int,
            },
        }
    }
}

fn lcu_generic(
    fut: impl Future<Output = Result<Option<Value>, Error>>,
    rt: &Runtime,
) -> LcuResponse {
    let result = rt.block_on(fut);

    match result {
        Ok(result) => match result {
            Some(json) => {
                let json_string = json.to_string();
                println!("{}", json_string);
                let json_c_string = CString::new(json_string).unwrap();
                println!("{:?}", json_c_string);
                LcuResponse {
                    json: json_c_string.into_raw(),
                    error: 0,
                }
            }
            None => LcuResponse {
                json: std::ptr::null_mut(),
                error: 0,
            },
        },
        Err(err) => LcuResponse {
            json: std::ptr::null_mut(),
            error: err as c_int,
        },
    }
}

#[no_mangle]
pub extern "C" fn lcu_new<'a>() -> NewLCU<'a> {
    NewLCU::new()
}

#[no_mangle]
pub unsafe extern "C" fn lcu_get(client: *mut Lcu, endpoint: *const c_char) -> LcuResponse {
    let client = &*client;
    let endpoint = CStr::from_ptr(endpoint).to_string_lossy();
    let fut = client.client.get::<Value>(&endpoint);
    lcu_generic(fut, &client.rt)
}

#[no_mangle]
pub unsafe extern "C" fn lcu_post(
    client: *mut Lcu,
    endpoint: *const c_char,
    body: *const c_char,
) -> LcuResponse {
    let client = &*client;
    let endpoint = CStr::from_ptr(endpoint).to_string_lossy();
    let body = CStr::from_ptr(body).to_string_lossy();
    let fut = client.client.post::<_, Value>(&endpoint, &body);
    lcu_generic(fut, &client.rt)
}

#[no_mangle]
pub unsafe extern "C" fn lcu_put(
    client: *mut Lcu,
    endpoint: *const c_char,
    body: *const c_char,
) -> LcuResponse {
    let client = &*client;
    let endpoint = CStr::from_ptr(endpoint).to_string_lossy();
    let body = CStr::from_ptr(body).to_string_lossy();
    let fut = client.client.put::<_, Value>(&endpoint, &body);
    lcu_generic(fut, &client.rt)
}

#[no_mangle]
pub unsafe extern "C" fn lcu_delete(client: *mut Lcu, endpoint: *const c_char) -> LcuResponse {
    let client = &*client;
    let endpoint = CStr::from_ptr(endpoint).to_string_lossy();
    let fut = client.client.delete::<Value>(&endpoint);
    lcu_generic(fut, &client.rt)
}

#[no_mangle]
pub unsafe extern "C" fn lcu_head(client: *mut Lcu, endpoint: *const c_char) -> LcuResponse {
    let client = &*client;
    let endpoint = CStr::from_ptr(endpoint).to_string_lossy();
    let fut = client.client.head::<Value>(&endpoint);
    lcu_generic(fut, &client.rt)
}

#[no_mangle]
pub extern "C" fn lcu_drop(client: *mut Lcu) {
    let client = unsafe { Box::from_raw(client) };
    drop(client)
}

#[no_mangle]
pub extern "C" fn lcu_drop_res(res: LcuResponse) {
    unsafe { CString::from_raw(res.json) };
}

mod tests {
    #[test]
    fn new_test() {
        use std::ffi::{CStr, CString};

        use crate::c::rest::{lcu_get, lcu_new};

        unsafe {
            let endpoint = CString::new("/lol-champ-select/v1/current-champion").unwrap();
            let lcu_handle = lcu_new();
            let get = lcu_get(lcu_handle.client, endpoint.as_ptr());
            println!("{:?}", get.json);
            if get.error > 0 {
                panic!("{}", get.error)
            } else {
                println!("{}", CStr::from_ptr(get.json).to_string_lossy());
            }
        }
    }
}

use std::{
    ffi::{c_char, CStr, CString},
    ptr::NonNull,
    str::FromStr,
};

use futures_util::Future;
use serde_json::Value;
use tokio::runtime::Runtime;

use crate::{rest::LCUClient, LcuResponse};

use super::runtime::RT;

fn lcu_generic(
    fut: impl Future<Output = Result<Option<Value>, LcuResponse>>,
    rt: &Runtime,
    c_json: *mut *mut c_char,
) -> LcuResponse {
    let result = rt.block_on(fut);

    match result {
        Ok(response) => unsafe {
            // SAFETY: c_json must be intilized if the value is success
            *c_json = response.map_or(std::ptr::null_mut(), json_to_cstring);
            LcuResponse::Success
        },
        Err(err) => err,
    }
}

fn json_to_cstring(json: Value) -> *mut i8 {
    let json_string = json.to_string();
    let json_c_string = CString::new(json_string).unwrap();
    json_c_string.into_raw()
}

/// Creates a new LCU handle
#[no_mangle]
pub unsafe extern "C" fn lcu_new(client: Option<NonNull<*mut LCUClient>>) -> LcuResponse {
    let lcu_client = LCUClient::new();
    match lcu_client {
        Ok(lcu_client) => {
            *client.unwrap().as_mut() = Box::into_raw(Box::new(lcu_client));
            LcuResponse::Success
        }
        Err(err) => err,
    }
}

/// Makes a get request to the LCU
#[no_mangle]
pub unsafe extern "C" fn lcu_get(
    client: Option<NonNull<LCUClient>>,
    rt: Option<NonNull<RT>>,
    endpoint: Option<NonNull<c_char>>,
    c_json: Option<NonNull<*mut c_char>>,
) -> LcuResponse {
    let client = client.unwrap().as_ref();
    let endpoint = CStr::from_ptr(endpoint.unwrap().as_ptr()).to_string_lossy();
    let fut = client.get(&endpoint);
    lcu_generic(fut, &rt.unwrap().as_ref().rt, c_json.unwrap().as_mut())
}

/// Makes a post request to the LCU
/// takes a string as a body that
/// must be json, else it will panic
#[no_mangle]
pub unsafe extern "C" fn lcu_post(
    client: Option<NonNull<LCUClient>>,
    rt: Option<NonNull<RT>>,
    endpoint: Option<NonNull<c_char>>,
    body: Option<NonNull<c_char>>,
    c_json: Option<NonNull<*mut c_char>>,
) -> LcuResponse {
    let client = client.unwrap().as_ref();
    let endpoint = CStr::from_ptr(endpoint.unwrap().as_ptr()).to_string_lossy();
    let body = CStr::from_ptr(body.unwrap().as_ptr()).to_string_lossy();
    let fut = client.post(&endpoint, Value::from_str(&body).unwrap());
    lcu_generic(fut, &rt.unwrap().as_ref().rt, c_json.unwrap().as_mut())
}

/// Makes a put request to the LCU
/// takes a string as a body that
/// must be json, else it will panic
#[no_mangle]
pub unsafe extern "C" fn lcu_put(
    client: Option<NonNull<LCUClient>>,
    rt: Option<NonNull<RT>>,
    endpoint: Option<NonNull<c_char>>,
    body: Option<NonNull<c_char>>,
    c_json: Option<NonNull<*mut c_char>>,
) -> LcuResponse {
    let client = client.unwrap().as_ref();
    let endpoint = CStr::from_ptr(endpoint.unwrap().as_ptr()).to_string_lossy();
    let body = CStr::from_ptr(body.unwrap().as_ptr()).to_string_lossy();
    let fut = client.put(&endpoint, Value::from_str(&body).unwrap());
    lcu_generic(fut, &rt.unwrap().as_ref().rt, c_json.unwrap().as_mut())
}

/// Makes a delete request to the LCU
#[no_mangle]
pub unsafe extern "C" fn lcu_delete(
    client: Option<NonNull<LCUClient>>,
    rt: Option<NonNull<RT>>,
    endpoint: Option<NonNull<c_char>>,
    c_json: Option<NonNull<*mut c_char>>,
) -> LcuResponse {
    let client = client.unwrap().as_ref();
    let endpoint = CStr::from_ptr(endpoint.unwrap().as_ptr()).to_string_lossy();
    let fut = client.delete(&endpoint);
    lcu_generic(fut, &rt.unwrap().as_ref().rt, c_json.unwrap().as_mut())
}

/// Makes a head request to the LCU
#[no_mangle]
pub unsafe extern "C" fn lcu_head(
    client: Option<NonNull<LCUClient>>,
    rt: Option<NonNull<RT>>,
    endpoint: Option<NonNull<c_char>>,
    c_json: Option<NonNull<*mut c_char>>,
) -> LcuResponse {
    let client = client.unwrap().as_ref();
    let endpoint = CStr::from_ptr(endpoint.unwrap().as_ptr()).to_string_lossy();
    let fut = client.head(&endpoint);
    lcu_generic(fut, &rt.unwrap().as_ref().rt, c_json.unwrap().as_mut())
}

/// Drops the client handle
#[no_mangle]
pub unsafe extern "C" fn lcu_drop(client: *mut *mut LCUClient) {
    drop(Box::from_raw(*client))
}

/// Drops the client response
#[no_mangle]
pub unsafe extern "C" fn lcu_drop_res(res: *mut *mut c_char) {
    drop(CString::from_raw(*res));
}

#[cfg(test)]
mod tests {
    #[test]
    fn new_test() {
        use std::{
            ffi::{CStr, CString},
            ptr::NonNull,
        };

        use crate::c::rest::{lcu_drop, lcu_drop_res, lcu_get, lcu_new};
        use crate::c::runtime::{drop_rt, new_rt};
        use crate::{rest::LCUClient, utils::request::HYPER_CLIENT};

        unsafe {
            let endpoint = CString::new("/lol-champ-select/v1/current-champion").unwrap();
            let rt = new_rt();
            let mut lcu_client: LCUClient = LCUClient {
                url: "".to_string(),
                client: &HYPER_CLIENT,
                auth_header: "".to_owned(),
            };
            let mut client: *mut LCUClient = &mut lcu_client;
            let a = lcu_new(NonNull::new(&mut client));
            if a as u8 > 0 {
                panic!()
            }
            println!("{:?}", *client);
            let mut c_json_owned = CString::default().into_raw();
            let c_json_ptr: *mut *mut i8 = &mut c_json_owned;
            let get = lcu_get(
                NonNull::new(&mut *client),
                NonNull::new(rt),
                NonNull::new(endpoint.into_raw()),
                NonNull::new(c_json_ptr),
            );
            if get as u8 > 0 {
                panic!("{}", get as u8)
            } else {
                println!("JSON: {}", CStr::from_ptr(*c_json_ptr).to_string_lossy());
                lcu_drop(&mut client);
                lcu_drop_res(c_json_ptr);
                drop_rt(rt);
            }
        }
    }
}

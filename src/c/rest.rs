use std::{
    ffi::{c_char, c_int, CStr, CString},
    ptr::{null_mut, NonNull},
};

use serde_json::Value;
use std::future::Future;
use tokio::task::JoinHandle;

use crate::{rest::LCUClient, LcuResponse};

use super::{runtime::RT, utils::json_to_cstring};

pub struct CFuture {
    pub fut: JoinHandle<()>,
}

/// SAFETY: The future cannot be null
#[no_mangle]
pub unsafe extern "C" fn is_finished(future: *mut CFuture) -> c_int {
    let future = &(*future).fut;
    if future.is_finished() {
        1
    } else {
        0
    }
}

/// SAFETY: The future here cannot be null
/// Dropping the future will abort it
#[no_mangle]
pub unsafe extern "C" fn drop_future(future: *mut *mut CFuture) {
    let fut = Box::from_raw(*future);
    fut.fut.abort();
    drop(fut);
    *future = std::ptr::null_mut();
}

/// SAFETY: This takes pointers to arrays, these arrays
/// must be the same size, and futures cannot be null.
/// This also takes a non-null tokio runtime used to
/// execture futures
#[no_mangle]
pub unsafe extern "C" fn await_future(
    // Futures returned by functions
    future: *mut CFuture,
    // Size of the future and json arrays
    rt: *mut RT,
) {
    // Get the runtime
    let rt = &(*rt).rt;
    // Construct the arrays that hold the futures and pointers
    let fut = &mut *future;
    // Run future
    rt.block_on(&mut fut.fut).unwrap();
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

/// SAFETY: None of these params can be null
/// the string passed to the function can be
/// null if there is no response or if there
/// is an error
#[no_mangle]
pub extern "C" fn lcu_get(
    client: *mut LCUClient<'static>,
    rt: *mut RT,
    endpoint: *const c_char,
    func: extern "C" fn(*mut c_char, LcuResponse),
) -> *mut CFuture {
    let client = unsafe { &*client };
    let endpoint = unsafe { CStr::from_ptr(endpoint) }.to_str().unwrap();
    let fut = client.get::<Value>(endpoint);
    generic_request(fut, rt, func)
}

/// SAFETY: None of these params can be null
/// the string passed to the function can be
/// null if there is no response or if there
/// is an error
#[no_mangle]
pub extern "C" fn lcu_post(
    client: *mut LCUClient<'static>,
    rt: *mut RT,
    endpoint: *const c_char,
    body: *mut c_char,
    func: extern "C" fn(*mut c_char, LcuResponse),
) -> *mut CFuture {
    let client = unsafe { &*client };
    let endpoint = unsafe { CStr::from_ptr(endpoint) }.to_str().unwrap();
    let c_str_body = unsafe { CStr::from_ptr(body) }.to_str().unwrap();
    let fut = client.post::<Value, _>(endpoint, c_str_body.into());
    generic_request(fut, rt, func)
}

/// SAFETY: None of these params can be null
/// the string passed to the function can be
/// null if there is no response or if there
/// is an error
#[no_mangle]
pub extern "C" fn lcu_put(
    client: *mut LCUClient<'static>,
    rt: *mut RT,
    endpoint: *const c_char,
    body: *mut c_char,
    func: extern "C" fn(*mut c_char, LcuResponse),
) -> *mut CFuture {
    let client = unsafe { &*client };
    let endpoint = unsafe { CStr::from_ptr(endpoint) }.to_str().unwrap();
    let c_str_body = unsafe { CStr::from_ptr(body) }.to_str().unwrap();
    let fut = client.put::<Value, _>(endpoint, c_str_body.into());
    generic_request(fut, rt, func)
}

/// SAFETY: None of these params can be null
/// the string passed to the function can be
/// null if there is no response or if there
/// is an error
#[no_mangle]
pub extern "C" fn lcu_delete(
    client: *mut LCUClient<'static>,
    rt: *mut RT,
    endpoint: *const c_char,
    func: extern "C" fn(*mut c_char, LcuResponse),
) -> *mut CFuture {
    let client = unsafe { &*client };
    let endpoint = unsafe { CStr::from_ptr(endpoint) }.to_str().unwrap();
    let fut = client.delete::<Value>(endpoint);
    generic_request(fut, rt, func)
}

/// SAFETY: None of these params can be null
/// the string passed to the function can be
/// null if there is no response or if there
/// is an error
#[no_mangle]
pub extern "C" fn lcu_head(
    client: *mut LCUClient<'static>,
    rt: *mut RT,
    endpoint: *const c_char,
    func: extern "C" fn(*mut c_char, LcuResponse),
) -> *mut CFuture {
    let client = unsafe { &*client };
    let endpoint = unsafe { CStr::from_ptr(endpoint) }.to_str().unwrap();
    let fut = client.head::<Value>(endpoint);
    generic_request(fut, rt, func)
}

fn generic_request(
    fut: impl Future<Output = Result<Option<Value>, LcuResponse>> + std::marker::Send + 'static,
    rt: *mut RT,
    func: extern "C" fn(*mut c_char, LcuResponse),
) -> *mut CFuture {
    let rt = &unsafe { &*rt }.rt;
    Box::into_raw(Box::new(CFuture {
        fut: rt.spawn(async move {
            let res = fut.await;
            match res {
                Ok(val) => func(
                    val.map_or_else(null_mut, json_to_cstring),
                    LcuResponse::Success,
                ),
                Err(err) => func(null_mut(), err),
            };
        }),
    }))
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
mod test {
    use std::ffi::{c_char, CStr};

    use crate::LcuResponse;

    #[allow(dead_code)]
    extern "C" fn printer(i: *mut c_char, a: LcuResponse) {
        println!("{:?}", a);
        println!("{:?}", unsafe { CStr::from_ptr(i) });
    }

    #[test]
    fn get_current_champ() {
        use std::{ffi::CString, ptr::NonNull};

        use crate::{
            c::{
                rest::{await_future, drop_future, is_finished},
                runtime::new_rt,
            },
            rest::LCUClient,
            utils::request::HYPER_CLIENT,
        };

        use super::{lcu_get, lcu_new};

        let rt = new_rt();
        let mut lcu_client: LCUClient = LCUClient {
            url: "".to_string(),
            client: &HYPER_CLIENT,
            auth_header: "".to_owned(),
        };
        let mut client: *mut LCUClient = &mut lcu_client;
        let a = unsafe { lcu_new(NonNull::new(&mut client)) };
        if a as u8 > 0 {
            panic!()
        }
        let mut fut = lcu_get(
            client,
            rt,
            CString::new("/lol-champ-select/v1/current-champion")
                .unwrap()
                .into_raw(),
            printer,
        );

        let mut fut_2 = lcu_get(
            client,
            rt,
            CString::new("/lol-champ-select/v1/current-champion")
                .unwrap()
                .into_raw(),
            printer,
        );

        loop {
            if unsafe { is_finished(fut) == 1 && is_finished(fut_2) == 1 } {
                break;
            }
        }

        unsafe { await_future(fut, rt) };
        unsafe { await_future(fut_2, rt) };

        unsafe {
            drop_future(&mut fut);
            drop_future(&mut fut_2);
        }
    }
}

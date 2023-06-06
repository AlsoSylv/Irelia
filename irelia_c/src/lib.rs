#![cfg_attr(not(test), no_std)]

use irelia::{rest::LCUClient, LCUError, RequestClient};
use serde_json::Value;
use tokio::{runtime::Runtime, task::JoinHandle};

extern crate alloc;

pub struct CRT {
    rt: Runtime,
}

pub struct CFuture {
    fut: JoinHandle<Result<Option<Value>, LCUError>>,
}

pub struct CRequestClient {
    client: RequestClient,
}

pub struct CLCUClient<'a> {
    client: LCUClient<'a>,
}

pub struct CLCUResponse {
    res: i8,
    error: Option<LCUError>,
}

mod rt {
    use crate::{utils::error_to_int, CFuture, CLCUResponse, CRT};
    use alloc::{ffi::CString, boxed::Box, string::ToString};
    use core::{
        ffi::c_char,
        ptr::{null, null_mut},
    };
    use tokio::runtime::Runtime;

    #[no_mangle]
    pub extern "C" fn new_rt() -> *const CRT {
        Box::leak(Box::new(CRT {
            rt: Runtime::new().unwrap(),
        }))
    }

    #[no_mangle]
    pub extern "C" fn block_on(
        fut: *mut CFuture,
        rt: *const CRT,
        res: *mut *mut c_char,
    ) -> *const CLCUResponse {
        assert!(fut != null_mut());
        assert!(rt != null());

        let rt = &unsafe { &*rt }.rt;
        let fut = unsafe { core::ptr::read(fut) }.fut;
        let endpoint_res = rt.block_on(fut).unwrap();

        match endpoint_res {
            Ok(response) => {
                match response {
                    Some(str) => unsafe {
                        *res = CString::new(str.to_string()).unwrap().into_raw()
                    },
                    None => unsafe { *res = null_mut() },
                };

                Box::leak(Box::new(CLCUResponse {
                    res: 0,
                    error: None,
                }))
            }
            Err(err) => {
                let response = error_to_int(&err);
                unsafe {
                    *res = null_mut();
                }

                Box::leak(Box::new(CLCUResponse {
                    res: response,
                    error: Some(err),
                }))
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn is_finished(fut: *mut CFuture) -> c_char {
        assert!(fut != null_mut());
        unsafe { &*fut }.fut.is_finished() as c_char
    }
}

mod request {
    use crate::CRequestClient;
    use irelia::RequestClient;
    use alloc::boxed::Box;

    #[no_mangle]
    pub extern "C" fn new_request_client() -> *const CRequestClient {
        Box::leak(Box::new(CRequestClient {
            client: RequestClient::new(),
        }))
    }
}

mod lcu {
    use crate::{utils::error_to_int, CFuture, CLCUClient, CLCUResponse, CRequestClient, CRT};
    use core::{
        ffi::{c_char, CStr},
        ptr::{null, null_mut},
    };
    use irelia::rest::LCUClient;
    use serde_json::Value;
    use alloc::{str::FromStr, boxed::Box};

    #[no_mangle]
    pub extern "C" fn new_lcu_client(
        client: *const CRequestClient,
        lcu_client: *mut *mut CLCUClient,
    ) -> *const CLCUResponse {
        assert!(client != null());

        let client_ref = unsafe { &*client };

        let client = LCUClient::new(&client_ref.client);

        match client {
            Ok(client) => {
                unsafe { *lcu_client = Box::leak(Box::new(CLCUClient { client })) };
                Box::leak(Box::new(CLCUResponse {
                    res: 0,
                    error: None,
                }))
            }

            Err(err) => {
                let error_code = error_to_int(&err);

                unsafe { *lcu_client = null_mut() };

                Box::leak(Box::new(CLCUResponse {
                    res: error_code,
                    error: Some(err),
                }))
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn lcu_get(
        client: *const CLCUClient<'static>,
        rt: *const CRT,
        endpoint: *const c_char,
    ) -> *mut CFuture {
        assert!(client != null());
        assert!(endpoint != null());
        assert!(rt != null());

        let client = &unsafe { &*client }.client;
        let endpoint = unsafe { CStr::from_ptr(endpoint) }.to_str().unwrap();
        let rt = &unsafe { &*rt }.rt;

        let fut = client.get::<Value>(endpoint);

        Box::leak(Box::new(CFuture { fut: rt.spawn(fut) }))
    }

    #[no_mangle]
    pub extern "C" fn lcu_post(
        client: *const CLCUClient<'static>,
        rt: *const CRT,
        endpoint: *const c_char,
        body: *const c_char,
    ) -> *mut CFuture {
        assert!(client != null());
        assert!(endpoint != null());
        assert!(rt != null());

        let client = &unsafe { &*client }.client;
        let endpoint = unsafe { CStr::from_ptr(endpoint) }.to_str().unwrap();
        let rt = &unsafe { &*rt }.rt;
        let fut = if body == null() {
            client.post(endpoint, None)
        } else {
            let body = unsafe { CStr::from_ptr(body) }.to_str().unwrap();
            client.post(endpoint, Some(Value::from_str(body).unwrap()))
        };

        Box::leak(Box::new(CFuture { fut: rt.spawn(fut) }))
    }
}

pub(crate) mod utils {
    use crate::CLCUResponse;
    use alloc::{ffi::CString, string::ToString};
    use core::{ffi::c_char, ptr::null};
    use irelia::LCUError;

    pub fn error_to_int(error: &LCUError) -> c_char {
        match error {
            LCUError::HyperHttpError(_) => 1,
            LCUError::HyperError(_) => 2,
            LCUError::SerdeJsonError(_) => 3,
            LCUError::LCUProcessNotRunning => 4,
            LCUError::PortNotFound => 5,
            LCUError::AuthTokenNotFound => 6,
        }
    }

    #[no_mangle]
    pub extern "C" fn get_response_code(res: *const CLCUResponse) -> c_char {
        assert!(res != null());

        let err = unsafe { &*res };
        err.res
    }

    #[no_mangle]
    pub extern "C" fn get_response_description(res: *const CLCUResponse) -> *const c_char {
        assert!(res != null());

        let err = unsafe { &*res };

        if let Some(err) = &err.error {
            CString::new(err.to_string()).unwrap().into_raw()
        } else {
            CString::new("Success").unwrap().into_raw()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn c_test() {
        let a = (inline_c::assert_c! {
            #include "bindings.h"
            #include <stdio.h>
            #include <stdlib.h>

            int main() {
                const struct RT* rt = new_rt();
                const struct RequestClient* rc = new_request_client();

                struct LCUClient* lc;
                const struct LCUResponse* res = new_lcu_client(rc, &lc);

                const char code = get_response_code(res);

                printf("%d \n", (int) code);

                if (code != 0) {
                    printf("%s \n", get_response_description(res));

                    return 1;
                } else {
                    Future* fut = lcu_get(lc, rt, "/lol-summoner/v1/current-summoner");

                    char* res;
                    const LCUResponse* get_res = block_on(fut, rt, &res);

                    const char code = get_response_code(get_res);

                    if (code != 0) {
                        printf("%s \n", get_response_description(get_res));

                        return 1;
                    } else {
                        printf("%s \n", res);
                    }
                }

                return 0;
            }
        })
        .success()
        .code(0);

        println!("{}", a.to_string())
    }
}

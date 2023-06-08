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
    use alloc::{boxed::Box, ffi::CString, string::ToString};
    use core::{
        ffi::c_char,
        ptr::{null, null_mut},
    };
    use tokio::runtime::Runtime;

    #[no_mangle]
    pub extern "C" fn new_rt() -> *mut CRT {
        Box::into_raw(Box::new(CRT {
            rt: Runtime::new().unwrap(),
        }))
    }

    #[no_mangle]
    pub extern "C" fn drop_rt(rt: *mut CRT) {
        assert!(rt != null_mut());
        unsafe { drop(Box::from_raw(rt)) };
    }

    #[no_mangle]
    pub extern "C" fn block_on(
        fut: *mut CFuture,
        rt: *const CRT,
        res: *mut *mut c_char,
    ) -> *mut CLCUResponse {
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
    use alloc::boxed::Box;
    use irelia::RequestClient;

    #[no_mangle]
    pub extern "C" fn new_request_client() -> *mut CRequestClient {
        Box::leak(Box::new(CRequestClient {
            client: RequestClient::new(),
        }))
    }

    #[no_mangle]
    pub extern "C" fn drop_request_client(client: *mut CRequestClient) {
        unsafe { drop(Box::from_raw(client)) }
    }
}

mod lcu {
    use crate::{utils::error_to_int, CFuture, CLCUClient, CLCUResponse, CRequestClient, CRT};
    use alloc::{boxed::Box, str::FromStr};
    use core::{
        ffi::{c_char, CStr},
        ptr::{null, null_mut},
    };
    use irelia::rest::LCUClient;
    use serde_json::Value;
    use tokio::runtime::Runtime;

    #[no_mangle]
    pub extern "C" fn new_lcu_client(
        client: *const CRequestClient,
        lcu_client: *mut *mut CLCUClient,
    ) -> *mut CLCUResponse {
        assert!(client != null());

        let client_ref = unsafe { &*client };

        let client = LCUClient::new(&client_ref.client);

        match client {
            Ok(client) => {
                unsafe { *lcu_client = Box::into_raw(Box::new(CLCUClient { client })) };
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
    pub extern "C" fn lcu_delete(
        client: *const CLCUClient<'static>,
        rt: *const CRT,
        endpoint: *const c_char,
    ) -> *mut CFuture {
        let (client, rt) = check_ptrs(client, rt, endpoint);

        make_request(client, rt, endpoint, RequestType::Delete)
    }

    #[no_mangle]
    pub extern "C" fn lcu_get(
        client: *const CLCUClient<'static>,
        rt: *const CRT,
        endpoint: *const c_char,
    ) -> *mut CFuture {
        let (client, rt) = check_ptrs(client, rt, endpoint);

        make_request(client, rt, endpoint, RequestType::Get)
    }

    #[no_mangle]
    pub extern "C" fn lcu_head(
        client: *const CLCUClient<'static>,
        rt: *const CRT,
        endpoint: *const c_char,
    ) -> *mut CFuture {
        let (client, rt) = check_ptrs(client, rt, endpoint);

        make_request(client, rt, endpoint, RequestType::Head)
    }

    #[no_mangle]
    pub extern "C" fn lcu_post(
        client: *const CLCUClient<'static>,
        rt: *const CRT,
        endpoint: *const c_char,
        body: *const c_char,
    ) -> *mut CFuture {
        let (client, rt) = check_ptrs(client, rt, endpoint);

        make_request(client, rt, endpoint, RequestType::Post(convert_body(body)))
    }

    #[no_mangle]
    pub extern "C" fn lcu_patch(
        client: *const CLCUClient<'static>,
        rt: *const CRT,
        endpoint: *const c_char,
        body: *const c_char,
    ) -> *mut CFuture {
        let (client, rt) = check_ptrs(client, rt, endpoint);

        make_request(client, rt, endpoint, RequestType::Patch(convert_body(body)))
    }

    #[no_mangle]
    pub extern "C" fn lcu_put(
        client: *const CLCUClient<'static>,
        rt: *const CRT,
        endpoint: *const c_char,
        body: *const c_char,
    ) -> *mut CFuture {
        let (client, rt) = check_ptrs(client, rt, endpoint);

        make_request(client, rt, endpoint, RequestType::Put(convert_body(body)))
    }

    #[no_mangle]
    pub extern "C" fn drop_lcu_client(lcu_client: *mut *mut CLCUClient) {
        unsafe { Box::from_raw(*lcu_client) };
    }

    fn check_ptrs(
        client: *const CLCUClient<'static>,
        rt: *const CRT,
        endpoint: *const c_char,
    ) -> (&'static LCUClient<'static>, &Runtime) {
        assert!(client != null());
        assert!(endpoint != null());
        assert!(rt != null());

        let client = &unsafe { &*client }.client;
        let rt = &unsafe { &*rt }.rt;

        (client, rt)
    }

    fn convert_body(body: *const c_char) -> Option<Value> {
        if body == null() {
            None
        } else {
            let body = unsafe { CStr::from_ptr(body) }.to_str().unwrap();
            Some(Value::from_str(body).unwrap())
        }
    }

    fn make_request(
        client: &'static LCUClient<'static>,
        rt: &Runtime,
        endpoint: *const c_char,
        request: RequestType,
    ) -> *mut CFuture {
        let endpoint = unsafe { CStr::from_ptr(endpoint) }.to_str().unwrap();

        Box::leak(Box::new(CFuture {
            fut: match request {
                RequestType::Delete => rt.spawn(client.delete(endpoint)),
                RequestType::Get => rt.spawn(client.get(endpoint)),
                RequestType::Head => rt.spawn(client.head(endpoint)),
                RequestType::Patch(body) => rt.spawn(client.patch(endpoint, body)),
                RequestType::Post(body) => rt.spawn(client.post(endpoint, body)),
                RequestType::Put(body) => rt.spawn(client.put(endpoint, body)),
            },
        }))
    }

    pub enum RequestType {
        Delete,
        Get,
        Head,
        Patch(Option<Value>),
        Post(Option<Value>),
        Put(Option<Value>),
    }
}

pub(crate) mod utils {
    use crate::{CFuture, CLCUResponse};
    use alloc::{boxed::Box, ffi::CString, string::ToString};
    use core::{ffi::c_char, ptr::null_mut};
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
    pub extern "C" fn get_response_code(res: *mut CLCUResponse) -> c_char {
        assert!(res != null_mut());

        let err = unsafe { &*res };
        err.res
    }

    #[no_mangle]
    pub extern "C" fn get_response_description(res: *mut CLCUResponse) -> *const c_char {
        assert!(res != null_mut());

        let err = unsafe { &*res };

        if let Some(err) = &err.error {
            CString::new(err.to_string()).unwrap().into_raw()
        } else {
            CString::new("Success").unwrap().into_raw()
        }
    }

    #[no_mangle]
    pub extern "C" fn drop_future(fut: *mut CFuture) {
        assert!(fut != null_mut());
        unsafe { drop(Box::from_raw(fut)) };
    }

    #[no_mangle]
    pub extern "C" fn drop_lcu_res(res: *mut CLCUResponse) {
        assert!(res != null_mut());
        unsafe { drop(Box::from_raw(res)) };
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
                struct RT* rt = new_rt();
                struct RequestClient* rc = new_request_client();
            
                struct LCUClient* lc;
                struct LCUResponse* res = new_lcu_client(rc, &lc);
            
                const char code = get_response_code(res);
            
                printf("%d \n", (int) code);
            
                if (code != 0) {
                    printf("%s \n", get_response_description(res));
                } else {
                    Future* fut = lcu_get(lc, rt, "/lol-summoner/v1/current-summoner");
            
                    char* res;
                    struct LCUResponse* get_res = block_on(fut, rt, &res);
            
                    drop_future(fut);
            
                    const char code = get_response_code(get_res);
            
                    if (code != 0) {
                        printf("%s \n", get_response_description(get_res));
                    } else {
                        printf("%s \n", res);
                    }
            
                    drop_lcu_res(get_res);
                }

                drop_lcu_client(&lc);
            
                drop_lcu_res(res);
                drop_rt(rt);
                drop_request_client(rc);
            
                return 0;
            }
        })
        .success()
        .code(0);

        println!("{}", a.to_string())
    }
}

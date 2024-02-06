#![cfg_attr(not(test), no_std)]

extern crate alloc;

mod types;

mod rt {
    use crate::{
        types::{CFuture, CLCUResponse, Crt},
        utils::error_to_int,
    };
    use alloc::{boxed::Box, ffi::CString, string::ToString};
    use core::{ffi::c_char, ptr::null_mut};
    use tokio::runtime::Runtime;

    #[no_mangle]
    pub extern "C" fn new_rt() -> *mut Crt {
        Box::into_raw(Box::new(Crt::new(Runtime::new().unwrap())))
    }

    #[no_mangle]
    pub extern "C" fn drop_rt(rt: *mut Crt) {
        assert!(!rt.is_null());
        unsafe { drop(Box::from_raw(rt)) };
    }

    #[no_mangle]
    pub extern "C" fn block_on(
        fut: *mut CFuture,
        rt: *const Crt,
        res: *mut *mut c_char,
    ) -> *mut CLCUResponse {
        assert!(!fut.is_null());
        assert!(!rt.is_null());

        let rt = unsafe { &*rt }.rt();
        let fut = unsafe { core::ptr::read(fut) }.take_fut();
        let endpoint_res = rt.block_on(fut).unwrap();

        match endpoint_res {
            Ok(response) => {
                match response {
                    Some(str) => unsafe {
                        *res = CString::new(str.to_string()).unwrap().into_raw()
                    },
                    None => unsafe { *res = null_mut() },
                };

                Box::leak(Box::new(CLCUResponse::new(0, None)))
            }
            Err(err) => {
                let response = error_to_int(&err);
                unsafe {
                    *res = null_mut();
                }

                Box::leak(Box::new(CLCUResponse::new(response, Some(err))))
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn is_finished(fut: *mut CFuture) -> c_char {
        assert!(!fut.is_null());
        unsafe { &*fut }.fut().is_finished() as c_char
    }
}

mod request {
    use crate::types::CRequestClient;
    use alloc::boxed::Box;
    use irelia::RequestClient;

    #[no_mangle]
    pub extern "C" fn new_request_client() -> *mut CRequestClient {
        Box::leak(Box::new(CRequestClient::new(RequestClient::new())))
    }

    #[no_mangle]
    pub extern "C" fn drop_request_client(client: *mut CRequestClient) {
        assert!(!client.is_null());
        unsafe { drop(Box::from_raw(client)) }
    }
}

mod lcu {
    use crate::{
        types::{CFuture, CLCUClient, CLCUResponse, CRequestClient, Crt},
        utils::error_to_int,
    };
    use alloc::{boxed::Box, str::FromStr};
    use core::{
        ffi::{c_char, CStr},
        ptr::null_mut,
    };
    use irelia::{rest::LCUClient, RequestClient};
    use serde_json::Value;
    use tokio::runtime::Runtime;

    #[no_mangle]
    pub extern "C" fn new_lcu_client(
        lcu_client: *mut *mut CLCUClient,
    ) -> *mut CLCUResponse {
        let client = LCUClient::new();

        match client {
            Ok(client) => {
                unsafe { *lcu_client = Box::leak(Box::new(CLCUClient::new(client))) };
                Box::leak(Box::new(CLCUResponse::new(0, None)))
            }

            Err(err) => {
                let error_code = error_to_int(&err);

                unsafe { *lcu_client = null_mut() };

                Box::leak(Box::new(CLCUResponse::new(error_code, Some(err))))
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn lcu_delete(
        client: *const CLCUClient,
        request_client: *const CRequestClient,
        rt: *const Crt,
        endpoint: *const c_char,
    ) -> *mut CFuture {
        let (client, rt, request_client) = check_ptrs(client, rt, request_client, endpoint);

        make_request(client, request_client, rt, endpoint, RequestType::Delete)
    }

    #[no_mangle]
    pub extern "C" fn lcu_get(
        client: *const CLCUClient,
        request_client: *const CRequestClient,
        rt: *const Crt,
        endpoint: *const c_char,
    ) -> *mut CFuture {
        let (client, rt, request_client) = check_ptrs(client, rt, request_client, endpoint);

        make_request(client, request_client, rt, endpoint, RequestType::Get)
    }

    #[no_mangle]
    pub extern "C" fn lcu_head(
        client: *const CLCUClient,
        request_client: *const CRequestClient,
        rt: *const Crt,
        endpoint: *const c_char,
    ) -> *mut CFuture {
        let (client, rt, request_client) = check_ptrs(client, rt, request_client, endpoint);

        make_request(client, request_client, rt, endpoint, RequestType::Head)
    }

    #[no_mangle]
    pub extern "C" fn lcu_post(
        client: *const CLCUClient,
        request_client: *const CRequestClient,
        rt: *const Crt,
        endpoint: *const c_char,
        body: *const c_char,
    ) -> *mut CFuture {
        let (client, rt, request_client) = check_ptrs(client, rt, request_client, endpoint);

        make_request(client, request_client, rt, endpoint, RequestType::Post(convert_body(body)))
    }

    #[no_mangle]
    pub extern "C" fn lcu_patch(
        client: *const CLCUClient,
        request_client: *const CRequestClient,
        rt: *const Crt,
        endpoint: *const c_char,
        body: *const c_char,
    ) -> *mut CFuture {
        let (client, rt, request_client) = check_ptrs(client, rt, request_client, endpoint);

        make_request(client, request_client, rt, endpoint, RequestType::Patch(convert_body(body)))
    }

    #[no_mangle]
    pub extern "C" fn lcu_put(
        client: *const CLCUClient,
        request_client: *const CRequestClient,
        rt: *const Crt,
        endpoint: *const c_char,
        body: *const c_char,
    ) -> *mut CFuture {
        let (client, rt, request_client) = check_ptrs(client, rt, request_client, endpoint);

        make_request(client, request_client, rt, endpoint, RequestType::Put(convert_body(body)))
    }

    #[no_mangle]
    pub extern "C" fn drop_lcu_client(lcu_client: *mut CLCUClient) {
        assert!(lcu_client.is_null());
        unsafe { drop(Box::from_raw(lcu_client)) };
    }

    fn check_ptrs(
        client: *const CLCUClient,
        rt: *const Crt,
        request_client: *const CRequestClient,
        endpoint: *const c_char,
    ) -> (&'static LCUClient, &'static Runtime, &'static RequestClient) {
        assert!(!client.is_null());
        assert!(!endpoint.is_null());
        assert!(!rt.is_null());
        assert!(!request_client.is_null());

        let client = &unsafe { &*client }.client();
        let rt = &unsafe { &*rt }.rt();
        let request_client = &unsafe { &*request_client }.client();

        (client, rt, request_client)
    }

    fn convert_body(body: *const c_char) -> Option<Value> {
        if body.is_null() {
            None
        } else {
            let body = unsafe { CStr::from_ptr(body) }.to_str().unwrap();
            Some(Value::from_str(body).unwrap())
        }
    }

    fn make_request(
        client: &'static LCUClient,
        request_client: &'static RequestClient,
        rt: &Runtime,
        endpoint: *const c_char,
        request: RequestType,
    ) -> *mut CFuture {
        let endpoint = unsafe { CStr::from_ptr(endpoint) }.to_str().unwrap();

        Box::leak(Box::new(CFuture::new(match request {
            RequestType::Delete => rt.spawn(client.delete(endpoint, request_client)),
            RequestType::Get => rt.spawn(client.get(endpoint, request_client)),
            RequestType::Head => rt.spawn(client.head(endpoint, request_client)),
            RequestType::Patch(body) => rt.spawn(client.patch(endpoint, body, request_client)),
            RequestType::Post(body) => rt.spawn(client.post(endpoint, body, request_client)),
            RequestType::Put(body) => rt.spawn(client.put(endpoint, body, request_client)),
        })))
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
    use crate::types::{CFuture, CLCUResponse};
    use alloc::{boxed::Box, ffi::CString, string::ToString};
    use core::ffi::c_char;
    use irelia::LCUError;

    pub fn error_to_int(error: &LCUError) -> c_char {
        match error {
            LCUError::HyperHttpError(_) => 1,
            LCUError::HyperClientError(_) => 2,
            LCUError::HyperError(_) => 3,
            LCUError::SerdeJsonError(_) => 4,
            LCUError::LCUProcessNotRunning => 5,
            LCUError::PortNotFound => 6,
            LCUError::AuthTokenNotFound => 7,
        }
    }

    #[no_mangle]
    pub extern "C" fn get_response_code(res: *mut CLCUResponse) -> c_char {
        assert!(!res.is_null());

        let err = unsafe { &*res };
        err.res()
    }

    #[no_mangle]
    pub extern "C" fn get_response_description(res: *mut CLCUResponse) -> *const c_char {
        assert!(!res.is_null());

        let err = unsafe { &*res };

        if let Some(err) = &err.error() {
            CString::new(err.to_string()).unwrap().into_raw()
        } else {
            CString::new("Success").unwrap().into_raw()
        }
    }

    #[no_mangle]
    pub extern "C" fn drop_future(fut: *mut CFuture) {
        assert!(!fut.is_null());
        unsafe { drop(Box::from_raw(fut)) };
    }

    #[no_mangle]
    pub extern "C" fn drop_lcu_res(res: *mut CLCUResponse) {
        assert!(!res.is_null());
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
                int return_val = 0;

                struct RT* rt = new_rt();
                struct RequestClient* rc = new_request_client();

                struct LCUClient* lc;
                struct LCUResponse* res = new_lcu_client(&lc);

                const char code = get_response_code(res);

                printf("%d \n", (int) code);

                if (code != 0) {
                    printf("%s \n", get_response_description(res));

                    return_val = 1;
                } else {
                    Future* fut = lcu_get(lc, rc, rt, "/lol-summoner/v1/current-summoner");

                    char* res;
                    struct LCUResponse* get_res = block_on(fut, rt, &res);

                    drop_future(fut);

                    const char code = get_response_code(get_res);

                    if (code != 0) {
                        printf("%s \n", get_response_description(get_res));

                        return_val = 1;
                    } else {
                        printf("%s \n", res);
                    }

                    drop_lcu_res(get_res);
                }

                if (lc != NULL) {
                    drop_lcu_client(lc);
                }

                drop_lcu_res(res);
                drop_rt(rt);
                drop_request_client(rc);

                return return_val;
            }
        })
        .success()
        .code(0);

        println!("{}", a.to_string())
    }
}

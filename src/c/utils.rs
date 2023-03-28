use std::ffi::CString;

use serde_json::Value;

pub(super) fn json_to_cstring(json: Value) -> *mut i8 {
    let json_string = json.to_string();
    let json_c_string = CString::new(json_string).unwrap();
    json_c_string.into_raw()
}

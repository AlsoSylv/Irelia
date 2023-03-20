use tokio::runtime::Runtime;

pub struct RT {
    pub rt: Runtime,
}

impl RT {
    fn new() -> Self {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("PAIN");
        RT { rt }
    }
}

#[no_mangle]
pub extern "C" fn new_rt() -> *mut RT {
    Box::into_raw(Box::new(RT::new()))
}

#[no_mangle]
pub unsafe extern "C" fn drop_rt(rt: *mut RT) {
    drop(Box::from_raw(rt));
}

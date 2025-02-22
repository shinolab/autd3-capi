use std::ffi::{CStr, CString, c_char};

use autd3capi_driver::{ConstPtr, libc};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDGetErr(src: ConstPtr, dst: *mut c_char) {
    unsafe {
        let src = Box::from_raw(src.0 as *mut String);
        let c_string = CString::new(src.as_str()).unwrap();
        let c_str: &CStr = c_string.as_c_str();
        libc::strcpy(dst, c_str.as_ptr());
    }
}

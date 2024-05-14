#[no_mangle]
pub unsafe extern "C" fn AUTDGetErr(src: ConstPtr, dst: *mut c_char) {
    let src = Box::from_raw(src as *mut String);
    let c_string: CString = CString::new(src.as_str()).unwrap();
    let c_str: &CStr = c_string.as_c_str();
    libc::strcpy(dst, c_str.as_ptr());
}

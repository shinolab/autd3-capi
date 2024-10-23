pub mod builder;
pub mod group;
pub mod timer;

use autd3::Controller;
use autd3capi_driver::{
    async_ffi::{FfiFuture, FutureExt},
    driver::firmware::{fpga::FPGAState, version::FirmwareVersion},
};
use driver::link::Link;

use std::ffi::c_char;

use autd3capi_driver::*;

#[repr(C)]
pub struct ResultController {
    pub result: ControllerPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl_ffi_result!(ResultController, ControllerPtr);

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerClose(cnt: ControllerPtr) -> FfiFuture<ResultStatus> {
    let cnt = take!(cnt, Controller<Box<dyn Link>>);
    async move { cnt.close().await.into() }.into_ffi()
}

#[repr(C)]
pub struct FPGAStateListPtr(pub *const libc::c_void);

impl_ptr!(FPGAStateListPtr, Vec<Option<FPGAState>>);

#[repr(C)]
pub struct ResultFPGAStateList {
    pub result: FPGAStateListPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl_ffi_result!(ResultFPGAStateList, FPGAStateListPtr);

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerFPGAState(
    mut cnt: ControllerPtr,
) -> FfiFuture<ResultFPGAStateList> {
    async move { cnt.fpga_state().await.into() }.into_ffi()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDControllerFPGAStateGet(p: FPGAStateListPtr, idx: u32) -> i16 {
    p[idx as usize].map_or(-1, |v| v.state() as i16)
}

#[no_mangle]
pub unsafe extern "C" fn AUTDControllerFPGAStateDelete(p: FPGAStateListPtr) {
    let _ = take!(p, Vec<Option<FPGAState>>);
}

#[repr(C)]
pub struct FirmwareVersionListPtr(pub *const libc::c_void);

impl_ptr!(FirmwareVersionListPtr, Vec<FirmwareVersion>);

#[repr(C)]
pub struct ResultFirmwareVersionList {
    pub result: FirmwareVersionListPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl_ffi_result!(ResultFirmwareVersionList, FirmwareVersionListPtr);

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerFirmwareVersionListPointer(
    mut cnt: ControllerPtr,
) -> FfiFuture<ResultFirmwareVersionList> {
    async move { cnt.firmware_version().await.into() }.into_ffi()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDControllerFirmwareVersionGet(
    p_info_list: FirmwareVersionListPtr,
    idx: u32,
    info: *mut c_char,
) {
    let info_str = std::ffi::CString::new(p_info_list[idx as usize].to_string()).unwrap();
    libc::strcpy(info, info_str.as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn AUTDControllerFirmwareVersionListPointerDelete(
    p_info_list: FirmwareVersionListPtr,
) {
    let _ = take!(p_info_list, Vec<FirmwareVersion>);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDFirmwareLatest(latest: *mut c_char) {
    let info_str = std::ffi::CString::new(FirmwareVersion::latest()).unwrap();
    libc::strcpy(latest, info_str.as_ptr());
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerSend(
    mut cnt: ControllerPtr,
    d: DatagramPtr,
) -> FfiFuture<ResultStatus> {
    async move { cnt.send(*Box::<DynamicDatagram>::from(d)).await.into() }.into_ffi()
}

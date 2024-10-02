pub mod builder;
pub mod group;

use autd3::{error::AUTDError, Controller};
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

impl From<Result<Controller<Box<dyn Link>>, AUTDError>> for ResultController {
    fn from(r: Result<Controller<Box<dyn Link>>, AUTDError>) -> Self {
        match r {
            Ok(v) => Self {
                result: ControllerPtr(Box::into_raw(Box::new(v)) as _),
                err_len: 0,
                err: ConstPtr(std::ptr::null_mut()),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: ControllerPtr(std::ptr::null()),
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: ConstPtr(Box::into_raw(Box::new(err)) as _),
                }
            }
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerClose(cnt: ControllerPtr) -> FfiFuture<ResultI32> {
    let cnt = take!(cnt, Controller<Box<dyn Link>>);
    async move {
        let r: ResultI32 = cnt.close().await.into();
        r
    }
    .into_ffi()
}

#[repr(C)]
pub struct FPGAStateListPtr(pub *const libc::c_void);

#[repr(C)]

pub struct ResultFPGAStateList {
    pub result: FPGAStateListPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl std::ops::Deref for FPGAStateListPtr {
    type Target = Vec<Option<FPGAState>>;

    fn deref(&self) -> &Self::Target {
        unsafe { (self.0 as *const Self::Target).as_ref().unwrap() }
    }
}

impl From<Result<Vec<Option<FPGAState>>, AUTDError>> for ResultFPGAStateList {
    fn from(r: Result<Vec<Option<FPGAState>>, AUTDError>) -> Self {
        match r {
            Ok(v) => Self {
                result: FPGAStateListPtr(Box::into_raw(Box::new(v)) as _),
                err_len: 0,
                err: ConstPtr(std::ptr::null_mut()),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: FPGAStateListPtr(std::ptr::null()),
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: ConstPtr(Box::into_raw(Box::new(err)) as _),
                }
            }
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerFPGAState(
    mut cnt: ControllerPtr,
) -> FfiFuture<ResultFPGAStateList> {
    async move {
        let r: ResultFPGAStateList = cnt.fpga_state().await.into();
        r
    }
    .into_ffi()
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

#[repr(C)]

pub struct ResultFirmwareVersionList {
    pub result: FirmwareVersionListPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl std::ops::Deref for FirmwareVersionListPtr {
    type Target = Vec<FirmwareVersion>;

    fn deref(&self) -> &Self::Target {
        unsafe { (self.0 as *const Self::Target).as_ref().unwrap() }
    }
}

impl From<Result<Vec<FirmwareVersion>, AUTDError>> for ResultFirmwareVersionList {
    fn from(r: Result<Vec<FirmwareVersion>, AUTDError>) -> Self {
        match r {
            Ok(v) => Self {
                result: FirmwareVersionListPtr(Box::into_raw(Box::new(v)) as _),
                err_len: 0,
                err: ConstPtr(std::ptr::null_mut()),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: FirmwareVersionListPtr(std::ptr::null()),
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: ConstPtr(Box::into_raw(Box::new(err)) as _),
                }
            }
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerFirmwareVersionListPointer(
    mut cnt: ControllerPtr,
) -> FfiFuture<ResultFirmwareVersionList> {
    async move {
        let r: ResultFirmwareVersionList = cnt.firmware_version().await.into();
        r
    }
    .into_ffi()
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
) -> FfiFuture<ResultI32> {
    async move {
        let r: ResultI32 = cnt.send(DynamicDatagramPack { d: d.into() }).await.into();
        r
    }
    .into_ffi()
}

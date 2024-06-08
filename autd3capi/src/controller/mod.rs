pub mod builder;
pub mod group;

use autd3::{error::AUTDError, Controller};
use autd3capi_driver::{
    driver::{
        datagram::Datagram,
        firmware::{fpga::FPGAState, version::FirmwareVersion},
    },
    tokio,
};

use std::ffi::c_char;

use autd3capi_driver::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ControllerPtr(pub ConstPtr);

impl std::ops::Deref for ControllerPtr {
    type Target = SyncController;
    fn deref(&self) -> &'static Self::Target {
        unsafe { (self.0 as *const SyncController).as_ref().unwrap() }
    }
}

impl std::ops::DerefMut for ControllerPtr {
    fn deref_mut(&mut self) -> &'static mut Self::Target {
        unsafe { (self.0 as *mut SyncController).as_mut().unwrap() }
    }
}

pub struct SyncController {
    runtime: tokio::runtime::Runtime,
    parallel_threshold: usize,
    last_parallel_threshold: usize,
    pub inner: Controller<SyncLink>,
}

impl SyncController {
    pub fn send<S: Datagram>(&mut self, s: S) -> Result<(), AUTDError> {
        self.last_parallel_threshold = s.parallel_threshold().unwrap_or(self.parallel_threshold);
        self.runtime.block_on(self.inner.send(s))
    }

    pub fn close(&mut self) -> Result<(), AUTDError> {
        self.runtime.block_on(self.inner.close())
    }

    pub fn firmware_version(&mut self) -> Result<Vec<FirmwareVersion>, AUTDError> {
        self.runtime.block_on(self.inner.firmware_version())
    }

    pub fn fpga_state(&mut self) -> Result<Vec<Option<FPGAState>>, AUTDError> {
        self.runtime.block_on(self.inner.fpga_state())
    }
}

#[repr(C)]

pub struct ResultController {
    pub result: ControllerPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl From<Result<SyncController, AUTDError>> for ResultController {
    fn from(r: Result<SyncController, AUTDError>) -> Self {
        match r {
            Ok(v) => Self {
                result: ControllerPtr(Box::into_raw(Box::new(v)) as _),
                err_len: 0,
                err: std::ptr::null_mut(),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: ControllerPtr(std::ptr::null()),
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: Box::into_raw(Box::new(err)) as _,
                }
            }
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerClose(mut cnt: ControllerPtr) -> ResultI32 {
    cnt.close().into()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDControllerDelete(mut cnt: ControllerPtr) -> ResultI32 {
    cnt.close()
        .map(|r| {
            let _ = take!(cnt, SyncController);
            r
        })
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerLastParallelThreshold(cnt: ControllerPtr) -> u16 {
    cnt.last_parallel_threshold as u16
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerFPGAState(
    mut cnt: ControllerPtr,
    out: *mut i32,
) -> ResultI32 {
    cnt.fpga_state()
        .map(|states| {
            states.iter().enumerate().for_each(|(i, state)| {
                out.add(i).write(state.map_or(-1, |s| s.state() as i32));
            });
            true
        })
        .into()
}

#[repr(C)]
pub struct FirmwareVersionListPtr(pub ConstPtr);

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
                err: std::ptr::null_mut(),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: FirmwareVersionListPtr(std::ptr::null()),
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: Box::into_raw(Box::new(err)) as _,
                }
            }
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerFirmwareVersionListPointer(
    mut cnt: ControllerPtr,
) -> ResultFirmwareVersionList {
    cnt.firmware_version().into()
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
pub unsafe extern "C" fn AUTDControllerSend(mut cnt: ControllerPtr, d: DatagramPtr) -> ResultI32 {
    cnt.send(DynamicDatagramPack { d: d.into() }).into()
}

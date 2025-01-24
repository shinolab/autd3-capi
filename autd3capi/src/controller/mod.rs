pub mod group;
pub mod sender;

use autd3::{core::link::Link, Controller};
use driver::{
    autd3_device::AUTD3,
    firmware::{fpga::FPGAState, version::FirmwareVersion},
    geometry::{Quaternion, UnitQuaternion},
};

use std::ffi::c_char;

use autd3capi_driver::*;

use sender::SenderOption;

#[repr(C)]
pub struct ResultController {
    pub result: ControllerPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl_result!(ResultController, ControllerPtr);

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerOpen(
    pos: *const Point3,
    rot: *const Quaternion,
    len: u16,
    link_builder: LinkBuilderPtr,
    option: SenderOption,
) -> ResultController {
    let pos = vec_from_raw!(pos, Point3, len);
    let rot = vec_from_raw!(rot, Quaternion, len);
    let link_builder = take!(link_builder, DynamicLinkBuilder);
    Controller::open_with_option(
        pos.into_iter().zip(rot).map(|(pos, rot)| AUTD3 {
            pos,
            rot: UnitQuaternion::from_quaternion(rot),
        }),
        *link_builder,
        option.into(),
    )
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerClose(cnt: ControllerPtr) -> ResultStatus {
    take!(cnt, Controller<Box<dyn Link>>).close().into()
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

impl_result!(ResultFPGAStateList, FPGAStateListPtr);

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerFPGAState(mut cnt: ControllerPtr) -> ResultFPGAStateList {
    cnt.fpga_state().into()
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

impl_result!(ResultFirmwareVersionList, FirmwareVersionListPtr);

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
pub unsafe extern "C" fn AUTDControllerSend(
    mut cnt: ControllerPtr,
    d: DatagramPtr,
) -> ResultStatus {
    cnt.send(*Box::<DynDatagram>::from(d)).into()
}

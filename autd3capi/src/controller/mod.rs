pub mod sender;

use autd3::{
    Controller,
    core::{link::Link, sleep::Sleep},
    driver::{
        autd3_device::AUTD3,
        firmware::{driver::TimerStrategy, v10::fpga::FPGAState, version::FirmwareVersion},
        geometry::{Quaternion, UnitQuaternion},
    },
};

use std::ffi::c_char;

use autd3capi_driver::{autd3::firmware::Auto, *};

use sender::SenderOption;

#[repr(C)]
pub struct ResultController {
    pub result: ControllerPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl_result!(ResultController, ControllerPtr);

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDControllerOpen(
    pos: *const Point3,
    rot: *const Quaternion,
    len: u16,
    link: LinkPtr,
    option: SenderOption,
    timer_strategy: TimerStrategyWrap,
) -> ResultController {
    let pos = vec_from_raw!(pos, Point3, len);
    let rot = vec_from_raw!(rot, Quaternion, len);
    let link = unsafe { take!(link, Box<dyn Link>) };
    Controller::open_with_option(
        pos.into_iter().zip(rot).map(|(pos, rot)| AUTD3 {
            pos,
            rot: UnitQuaternion::from_quaternion(rot),
        }),
        *link,
        option.into(),
        Box::<dyn TimerStrategy<Box<dyn Sleep>>>::from(timer_strategy),
    )
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDControllerClose(cnt: ControllerPtr) -> ResultStatus {
    unsafe { take!(cnt, Controller<Box<dyn Link>, Auto>) }
        .close()
        .into()
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

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDControllerFPGAState(mut cnt: ControllerPtr) -> ResultFPGAStateList {
    cnt.fpga_state().into()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDControllerFPGAStateGet(p: FPGAStateListPtr, idx: u32) -> i16 {
    p[idx as usize].map_or(-1, |v| v.state() as i16)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDControllerFPGAStateDelete(p: FPGAStateListPtr) {
    let _ = unsafe { take!(p, Vec<Option<FPGAState>>) };
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

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDControllerFirmwareVersionListPointer(
    mut cnt: ControllerPtr,
) -> ResultFirmwareVersionList {
    cnt.firmware_version().into()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDControllerFirmwareVersionGet(
    p_info_list: FirmwareVersionListPtr,
    idx: u32,
    info: *mut c_char,
) {
    unsafe {
        let info_str = std::ffi::CString::new(p_info_list[idx as usize].to_string()).unwrap();
        libc::strcpy(info, info_str.as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDControllerFirmwareVersionListPointerDelete(
    p_info_list: FirmwareVersionListPtr,
) {
    let _ = unsafe { take!(p_info_list, Vec<FirmwareVersion>) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDFirmwareLatest(latest: *mut c_char) {
    unsafe {
        let info_str = std::ffi::CString::new(FirmwareVersion::latest()).unwrap();
        libc::strcpy(latest, info_str.as_ptr());
    }
}

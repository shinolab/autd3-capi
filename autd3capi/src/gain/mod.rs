use std::collections::HashMap;

use autd3capi_driver::{
    autd3::prelude::IntoDatagramWithSegment, driver::error::AUTDInternalError, *,
};

pub mod bessel;
pub mod custom;
pub mod focus;
pub mod group;
pub mod null;
pub mod plane;
pub mod raw;
pub mod transform;
pub mod uniform;

#[repr(C)]
pub struct GainCalcDrivesMapPtr(pub ConstPtr);

#[repr(C)]

pub struct ResultGainCalcDrivesMap {
    pub result: GainCalcDrivesMapPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl From<Result<HashMap<usize, Vec<autd3::prelude::Drive>>, AUTDInternalError>>
    for ResultGainCalcDrivesMap
{
    fn from(r: Result<HashMap<usize, Vec<autd3::prelude::Drive>>, AUTDInternalError>) -> Self {
        match r {
            Ok(v) => Self {
                result: GainCalcDrivesMapPtr(Box::into_raw(Box::new(v)) as _),
                err_len: 0,
                err: std::ptr::null_mut(),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: GainCalcDrivesMapPtr(std::ptr::null()),
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: Box::into_raw(Box::new(err)) as _,
                }
            }
        }
    }
}

impl std::ops::Deref for GainCalcDrivesMapPtr {
    type Target = HashMap<usize, Vec<autd3::prelude::Drive>>;

    fn deref(&self) -> &Self::Target {
        unsafe { (self.0 as *mut Self::Target).as_ref().unwrap() }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainIntoDatagramWithSegment(
    gain: GainPtr,
    segment: Segment,
    update_segment: bool,
) -> DatagramPtr {
    (*take!(gain, Box<G>))
        .with_segment(segment.into(), update_segment)
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainIntoDatagram(gain: GainPtr) -> DatagramPtr {
    (*take!(gain, Box<G>)).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainCalc(
    gain: GainPtr,
    geometry: GeometryPtr,
) -> ResultGainCalcDrivesMap {
    take!(gain, Box<G>)
        .calc(&geometry)
        .map(|res| {
            geometry
                .devices()
                .map(|dev| (dev.idx(), dev.iter().map(|tr| res(dev)(tr)).collect()))
                .collect::<HashMap<_, _>>()
        })
        .into()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainCalcGetResult(
    src: GainCalcDrivesMapPtr,
    dst: *mut Drive,
    device: DevicePtr,
) {
    let src = &src[&device.idx()];
    std::ptr::copy_nonoverlapping(src.as_ptr() as *const _, dst, src.len());
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainCalcFreeResult(src: GainCalcDrivesMapPtr) {
    let _ = take!(src, HashMap<usize, Vec<Drive>>);
}

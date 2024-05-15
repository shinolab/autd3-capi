use std::collections::HashMap;

use autd3capi_driver::{
    driver::{
        datagram::{GainFilter, SwapSegment},
        error::AUTDInternalError,
    },
    *,
};

pub mod bessel;
pub mod custom;
pub mod focus;
pub mod group;
pub mod null;
pub mod plane;
pub mod raw;
pub mod uniform;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GainCalcDrivesMapPtr(pub ConstPtr);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResultGainCalcDrivesMap {
    pub result: GainCalcDrivesMapPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl
    From<
        Result<
            HashMap<usize, Vec<autd3capi_driver::driver::firmware::fpga::Drive>>,
            AUTDInternalError,
        >,
    > for ResultGainCalcDrivesMap
{
    fn from(
        r: Result<
            HashMap<usize, Vec<autd3capi_driver::driver::firmware::fpga::Drive>>,
            AUTDInternalError,
        >,
    ) -> Self {
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
    type Target = HashMap<usize, Vec<Drive>>;

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
        .with_segment(segment, update_segment)
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
    take!(gain, Box<G>).calc(&geometry, GainFilter::All).into()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainCalcGetResult(
    src: GainCalcDrivesMapPtr,
    dst: *mut Drive,
    idx: u32,
) {
    let idx = idx as usize;
    std::ptr::copy_nonoverlapping(src[&idx].as_ptr(), dst, src[&idx].len());
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainCalcFreeResult(src: GainCalcDrivesMapPtr) {
    let _ = take!(src, HashMap<usize, Vec<Drive>>);
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramChangeGainSegment(segment: Segment) -> DatagramPtr {
    SwapSegment::gain(segment.into()).into()
}

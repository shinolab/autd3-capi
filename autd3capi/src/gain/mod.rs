use autd3::derive::GainCalcFn;
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
pub struct GainCalcPtr(pub *const libc::c_void);

#[repr(C)]

pub struct ResultGainCalcDrivesMap {
    pub result: GainCalcPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl<'a> From<Result<GainCalcFn<'a>, AUTDInternalError>> for ResultGainCalcDrivesMap {
    fn from(r: Result<GainCalcFn<'a>, AUTDInternalError>) -> Self {
        match r {
            Ok(v) => Self {
                result: GainCalcPtr(Box::into_raw(Box::new(v)) as _),
                err_len: 0,
                err: ConstPtr(std::ptr::null_mut()),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: GainCalcPtr(std::ptr::null()),
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: ConstPtr(Box::into_raw(Box::new(err)) as _),
                }
            }
        }
    }
}

impl std::ops::Deref for GainCalcPtr {
    type Target = GainCalcFn<'static>;

    fn deref(&self) -> &Self::Target {
        unsafe { (self.0 as *const Self::Target).as_ref().unwrap() }
    }
}
impl std::ops::DerefMut for GainCalcPtr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { (self.0 as *mut Self::Target).as_mut().unwrap() }
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
    (gain.0 as *mut Box<G>)
        .as_ref()
        .unwrap()
        .calc(&geometry)
        .into()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainFree(gain: GainPtr) {
    let _ = take!(gain, Box<G>);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainCalcGetResult(
    src: GainCalcPtr,
    dst: *mut Drive,
    device: DevicePtr,
) {
    let mut src = src;
    let src = src(&device);
    let dst = dst as *mut autd3::prelude::Drive;
    device
        .iter()
        .for_each(|tr| dst.add(tr.idx()).write(src(tr)));
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainCalcFreeResult(src: GainCalcPtr) {
    let _ = take!(src, GainCalcFn<'static>);
}

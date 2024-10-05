use std::sync::Arc;

use autd3::derive::SamplingConfig;
use autd3capi_driver::*;
use driver::{datagram::IntoDatagramWithSegmentTransition, error::AUTDInternalError};

pub mod custom;
pub mod fir;
pub mod fourier;
pub mod radiation_pressure;
pub mod sine;
pub mod square;
pub mod r#static;

#[repr(C)]
pub struct ModulationCalcPtr(pub *const libc::c_void);

impl_ptr!(ModulationCalcPtr, Vec<u8>);

#[repr(C)]
pub struct ResultModulationCalc {
    pub result: ModulationCalcPtr,
    pub config: SamplingConfig,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl From<(Result<Arc<Vec<u8>>, AUTDInternalError>, SamplingConfig)> for ResultModulationCalc {
    fn from(r: (Result<Arc<Vec<u8>>, AUTDInternalError>, SamplingConfig)) -> Self {
        match r {
            (Ok(v), config) => Self {
                result: ModulationCalcPtr(Arc::into_raw(v) as _),
                config,
                err_len: 0,
                err: ConstPtr(std::ptr::null_mut()),
            },
            (Err(e), config) => {
                let err = e.to_string();
                Self {
                    result: ModulationCalcPtr(std::ptr::null()),
                    config,
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: ConstPtr(Box::into_raw(Box::new(err)) as _),
                }
            }
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSamplingConfig(m: ModulationPtr) -> SamplingConfig {
    (m.0 as *const Box<M>).as_ref().unwrap().sampling_config()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationIntoDatagramWithSegment(
    m: ModulationPtr,
    segment: Segment,
) -> DatagramPtr {
    (*take!(m, Box<M>))
        .with_segment(segment.into(), None)
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationIntoDatagramWithSegmentTransition(
    m: ModulationPtr,
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    (*take!(m, Box<M>))
        .with_segment(segment.into(), Some(transition_mode.into()))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationIntoDatagram(m: ModulationPtr) -> DatagramPtr {
    (*take!(m, Box<M>)).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationCalc(m: ModulationPtr) -> ResultModulationCalc {
    let m = take!(m, Box<M>);
    let config = m.sampling_config();
    (m.calc(), config).into()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationCalcGetResult(src: ModulationCalcPtr, dst: *mut u8) {
    std::ptr::copy_nonoverlapping(src.as_ptr() as _, dst, src.len());
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationCalcGetSize(src: ModulationCalcPtr) -> u16 {
    src.len() as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationCalcFreeResult(src: ModulationCalcPtr) {
    let _ = Arc::from_raw(src.0 as *mut Vec<u8>);
}

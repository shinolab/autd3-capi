use autd3capi_driver::{driver::error::AUTDInternalError, *};

pub mod fourier;
pub mod radiation_pressure;
pub mod raw;
pub mod sine;
pub mod square;
pub mod r#static;
pub mod transform;

#[repr(C)]
pub struct ModulationCalcPtr(pub ConstPtr);

impl std::ops::Deref for ModulationCalcPtr {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        unsafe { (self.0 as *mut Self::Target).as_ref().unwrap() }
    }
}

#[repr(C)]
pub struct ResultModulationCalc {
    pub result: ModulationCalcPtr,
    pub config: SamplingConfigWrap,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl From<(Result<Vec<u8>, AUTDInternalError>, SamplingConfigWrap)> for ResultModulationCalc {
    fn from(r: (Result<Vec<u8>, AUTDInternalError>, SamplingConfigWrap)) -> Self {
        match r {
            (Ok(v), config) => Self {
                result: ModulationCalcPtr(Box::into_raw(Box::new(v)) as _),
                config,
                err_len: 0,
                err: std::ptr::null_mut(),
            },
            (Err(e), config) => {
                let err = e.to_string();
                Self {
                    result: ModulationCalcPtr(std::ptr::null()),
                    config,
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: Box::into_raw(Box::new(err)) as _,
                }
            }
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationIntoDatagramWithSegment(
    m: ModulationPtr,
    segment: Segment,
) -> DatagramPtr {
    (*take!(m, Box<M>)).with_segment(segment, None).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationIntoDatagramWithSegmentTransition(
    m: ModulationPtr,
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    (*take!(m, Box<M>))
        .with_segment(segment, Some(transition_mode))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationIntoDatagram(m: ModulationPtr) -> DatagramPtr {
    (*take!(m, Box<M>)).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationCalc(
    m: ModulationPtr,
    geometry: GeometryPtr,
) -> ResultModulationCalc {
    let m = take!(m, Box<M>);
    let config = m.sampling_config();
    (m.calc(&geometry), config.into()).into()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationCalcGetResult(src: ModulationCalcPtr, dst: *mut u8) {
    std::ptr::copy_nonoverlapping(src.as_ptr() as _, dst, src.len());
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationCalcGetSize(src: ModulationCalcPtr) -> u32 {
    src.len() as u32
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationCalcFreeResult(src: ModulationCalcPtr) {
    let _ = take!(src, Vec<u8>);
}

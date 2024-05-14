use autd3capi_driver::{
    driver::{datagram::SwapSegment, error::AUTDInternalError},
    *,
};

pub mod fourier;
pub mod radiation_pressure;
pub mod raw;
pub mod sine;
pub mod square;
pub mod r#static;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ModulationCalcPtr(pub ConstPtr);

impl std::ops::Deref for ModulationCalcPtr {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        unsafe { (self.0 as *mut Self::Target).as_ref().unwrap() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResultModulationCalc {
    pub result: ModulationCalcPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl From<Result<Vec<u8>, AUTDInternalError>> for ResultModulationCalc {
    fn from(r: Result<Vec<u8>, AUTDInternalError>) -> Self {
        match r {
            Ok(v) => Self {
                result: ModulationCalcPtr(Box::into_raw(Box::new(v)) as _),
                err_len: 0,
                err: std::ptr::null_mut(),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: ModulationCalcPtr(std::ptr::null()),
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
    transition_mode: TransitionMode,
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
    take!(m, Box<M>).calc(&geometry).into()
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

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSwapSegmentModulation(
    segment: Segment,
    transition_mode: TransitionMode,
) -> DatagramPtr {
    SwapSegment::modulation(segment.into(), transition_mode.into()).into()
}

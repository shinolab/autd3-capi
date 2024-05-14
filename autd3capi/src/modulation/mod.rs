use autd3capi_def::{
    driver::{datagram::ChangeModulationSegment, derive::EmitIntensity},
    *,
};

pub mod custom;
pub mod fourier;
pub mod radiation_pressure;
pub mod sine;
pub mod square;
pub mod r#static;
pub mod transform;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ModulationCalcPtr(pub ConstPtr);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResultModulationCalc {
    pub result: ModulationCalcPtr,
    pub result_len: u32,
    pub freq_div: u32,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl
    From<(
        autd3capi_def::driver::common::SamplingConfig,
        Result<Vec<EmitIntensity>, AUTDInternalError>,
    )> for ResultModulationCalc
{
    fn from(
        r: (
            autd3capi_def::driver::common::SamplingConfig,
            Result<Vec<EmitIntensity>, AUTDInternalError>,
        ),
    ) -> Self {
        match r.1 {
            Ok(v) => Self {
                result_len: v.len() as u32,
                freq_div: r.0.frequency_division(),
                result: ModulationCalcPtr(Box::into_raw(Box::new(v)) as _),
                err_len: 0,
                err: std::ptr::null_mut(),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: ModulationCalcPtr(std::ptr::null()),
                    result_len: 0,
                    freq_div: 0,
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: Box::into_raw(Box::new(err)) as _,
                }
            }
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSamplingConfig(m: ModulationPtr) -> SamplingConfig {
    take!(m, Box<M>).sampling_config().into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationIntoDatagramWithSegment(
    m: ModulationPtr,
    segment: Segment,
    update_segment: bool,
) -> DatagramPtr {
    (*take!(m, Box<M>))
        .with_segment(segment, update_segment)
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationIntoDatagram(m: ModulationPtr) -> DatagramPtr {
    (*take!(m, Box<M>)).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSize(m: ModulationPtr) -> ResultI32 {
    take!(m, Box<M>).len().into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationCalc(m: ModulationPtr) -> ResultModulationCalc {
    let m = take!(m, Box<M>);
    (m.sampling_config(), m.calc()).into()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationCalcGetResult(src: ModulationCalcPtr, dst: *mut u8) {
    let src = take!(src, Vec<EmitIntensity>);
    std::ptr::copy_nonoverlapping(src.as_ptr() as _, dst, src.len());
}

#[repr(u8)]
pub enum SamplingMode {
    ExactFrequency = 0,
    SizeOptimized = 1,
}

impl From<SamplingMode> for autd3::modulation::SamplingMode {
    fn from(mode: SamplingMode) -> Self {
        match mode {
            SamplingMode::ExactFrequency => autd3::modulation::SamplingMode::ExactFrequency,
            SamplingMode::SizeOptimized => autd3::modulation::SamplingMode::SizeOptimized,
        }
    }
}

impl From<autd3::modulation::SamplingMode> for SamplingMode {
    fn from(mode: autd3::modulation::SamplingMode) -> Self {
        match mode {
            autd3::modulation::SamplingMode::ExactFrequency => SamplingMode::ExactFrequency,
            autd3::modulation::SamplingMode::SizeOptimized => SamplingMode::SizeOptimized,
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramChangeModulationSegment(segment: Segment) -> DatagramPtr {
    ChangeModulationSegment::new(segment.into()).into()
}

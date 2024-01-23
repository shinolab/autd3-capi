use autd3capi_def::{driver::derive::EmitIntensity, *};

pub mod custom;
pub mod fourier;
pub mod radiation_pressure;
pub mod sine;
pub mod square;
pub mod r#static;
pub mod transform;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSamplingConfig(m: ModulationPtr) -> SamplingConfiguration {
    Box::from_raw(m.0 as *mut Box<M>).sampling_config().into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationIntoDatagram(m: ModulationPtr) -> DatagramPtr {
    DatagramPtr::new(*Box::from_raw(m.0 as *mut Box<M>))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSize(m: ModulationPtr) -> ResultI32 {
    Box::from_raw(m.0 as *mut Box<M>).len().into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationCalc(m: ModulationPtr) -> ResultModulationCalc {
    let m = Box::from_raw(m.0 as *mut Box<M>);
    (m.sampling_config(), m.calc()).into()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationCalcGetResult(src: ModulationCalcPtr, dst: *mut u8) {
    let src = cast!(src.0, Vec<EmitIntensity>);
    std::ptr::copy_nonoverlapping(src.as_ptr() as _, dst, src.len());
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationCalcFreeResult(src: ModulationCalcPtr) {
    let _ = Box::from_raw(src.0 as *mut Vec<EmitIntensity>);
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

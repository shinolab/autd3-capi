use autd3capi_def::{autd3::gain::Uniform, *};
use driver::common::Phase;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainUniform(intensity: u8, phase: u8) -> GainPtr {
    Uniform::new(intensity).with_phase(Phase::new(phase)).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainUniformDefaultPhase() -> u8 {
    Uniform::new(0).phase().value()
}

use autd3::derive::EmitIntensity;
use autd3capi_driver::{
    autd3::{derive::Phase, gain::Uniform},
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainUniform(intensity: u8, phase: u8) -> GainPtr {
    Uniform::new((EmitIntensity::new(intensity), Phase::new(phase))).into()
}

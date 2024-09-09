use autd3::prelude::{EmitIntensity, Phase};
use autd3capi_driver::{autd3::gain::Uniform, *};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainUniform(intensity: u8, phase: u8) -> GainPtr {
    Uniform::new((EmitIntensity::new(intensity), Phase::new(phase))).into()
}

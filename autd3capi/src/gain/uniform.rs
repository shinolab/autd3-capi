use autd3::core::gain::{Intensity, Phase};
use autd3capi_driver::{autd3::gain::Uniform, *};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGainUniform(intensity: Intensity, phase: Phase) -> GainPtr {
    Uniform { intensity, phase }.into()
}

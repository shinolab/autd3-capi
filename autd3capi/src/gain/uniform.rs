use autd3::core::gain::{EmitIntensity, Phase};
use autd3capi_driver::{autd3::gain::Uniform, *};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainUniform(intensity: EmitIntensity, phase: Phase) -> GainPtr {
    Uniform { intensity, phase }.into()
}

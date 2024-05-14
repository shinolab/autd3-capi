use autd3capi_driver::{
    autd3::{derive::Phase, gain::Uniform},
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainUniform(intensity: u8, phase: u8) -> GainPtr {
    Uniform::new(intensity).with_phase(Phase::new(phase)).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainUniformIsDefault(uniform: GainPtr) -> bool {
    let g = take_gain!(uniform, Uniform);
    g.phase() == Uniform::new(0).phase()
}

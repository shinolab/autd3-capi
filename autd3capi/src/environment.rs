use autd3capi_driver::{ControllerPtr, EnvironmentPtr};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDEnvironment(cnt: ControllerPtr) -> EnvironmentPtr {
    EnvironmentPtr(&cnt.environment as *const _ as *mut _)
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDEnvironmentGetSoundSpeed(env: EnvironmentPtr) -> f32 {
    env.sound_speed
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDEnvironmentSetSoundSpeed(mut env: EnvironmentPtr, value: f32) {
    env.sound_speed = value;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDEnvironmentSetSoundSpeedFromTemp(
    mut env: EnvironmentPtr,
    temp: f32,
    k: f32,
    r: f32,
    m: f32,
) {
    env.set_sound_speed_from_temp_with(temp, k, r, m);
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDEnvironmentWavelength(env: EnvironmentPtr) -> f32 {
    env.wavelength()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDEnvironmentWavenumber(env: EnvironmentPtr) -> f32 {
    env.wavenumber()
}

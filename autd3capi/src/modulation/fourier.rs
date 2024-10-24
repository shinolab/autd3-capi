#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::{
    autd3::{
        derive::{LoopBehavior, SamplingConfig},
        modulation::{Fourier, Sine},
        prelude::{rad, Hz},
    },
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationFourierExact(
    sine_freq: *const u32,
    sine_config: *const SamplingConfig,
    sine_intensity: *const u8,
    sine_offset: *const u8,
    sine_phase: *const f32,
    sine_clamp: *const bool,
    size: u32,
    clamp: bool,
    scale_factor: f32,
    offset: u8,
    loop_behavior: LoopBehavior,
) -> ResultModulation {
    Fourier::new((0..size as usize).map(|i| {
        Sine::new(sine_freq.add(i).read() * Hz)
            .with_intensity(sine_intensity.add(i).read())
            .with_offset(sine_offset.add(i).read())
            .with_phase(sine_phase.add(i).read() * rad)
            .with_clamp(sine_clamp.add(i).read())
            .with_sampling_config(sine_config.add(i).read())
            .unwrap()
    }))
    .map(|f| {
        f.with_clamp(clamp)
            .with_scale_factor(if scale_factor.is_nan() {
                None
            } else {
                Some(scale_factor)
            })
            .with_offset(offset)
            .with_loop_behavior(loop_behavior)
    })
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationFourierExactFloat(
    sine_freq: *const f32,
    sine_config: *const SamplingConfig,
    sine_intensity: *const u8,
    sine_offset: *const u8,
    sine_phase: *const f32,
    sine_clamp: *const bool,
    size: u32,
    clamp: bool,
    scale_factor: f32,
    offset: u8,
    loop_behavior: LoopBehavior,
) -> ResultModulation {
    Fourier::new((0..size as usize).map(|i| {
        Sine::new(sine_freq.add(i).read() * Hz)
            .with_intensity(sine_intensity.add(i).read())
            .with_offset(sine_offset.add(i).read())
            .with_phase(sine_phase.add(i).read() * rad)
            .with_clamp(sine_clamp.add(i).read())
            .with_sampling_config(sine_config.add(i).read())
            .unwrap()
    }))
    .map(|f| {
        f.with_clamp(clamp)
            .with_scale_factor(if scale_factor.is_nan() {
                None
            } else {
                Some(scale_factor)
            })
            .with_offset(offset)
            .with_loop_behavior(loop_behavior)
    })
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationFourierNearest(
    sine_freq: *const f32,
    sine_config: *const SamplingConfig,
    sine_intensity: *const u8,
    sine_offset: *const u8,
    sine_phase: *const f32,
    sine_clamp: *const bool,
    size: u32,
    clamp: bool,
    scale_factor: f32,
    offset: u8,
    loop_behavior: LoopBehavior,
) -> ResultModulation {
    Fourier::new((0..size as usize).map(|i| {
        Sine::new_nearest(sine_freq.add(i).read() * Hz)
            .with_intensity(sine_intensity.add(i).read())
            .with_offset(sine_offset.add(i).read())
            .with_phase(sine_phase.add(i).read() * rad)
            .with_clamp(sine_clamp.add(i).read())
            .with_sampling_config(sine_config.add(i).read())
            .unwrap()
    }))
    .map(|f| {
        f.with_clamp(clamp)
            .with_scale_factor(if scale_factor.is_nan() {
                None
            } else {
                Some(scale_factor)
            })
            .with_offset(offset)
            .with_loop_behavior(loop_behavior)
    })
    .into()
}

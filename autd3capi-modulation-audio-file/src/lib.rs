#![allow(clippy::missing_safety_doc)]

use std::ffi::c_char;

use autd3::{
    core::modulation::{LoopBehavior, SamplingConfig},
    prelude::Hz,
};
use autd3capi_driver::*;

use autd3_modulation_audio_file::{Csv, RawPCM, Wav};

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationAudioFileTracingInit() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationAudioFileTracingInitWithFile(
    path: *const c_char,
) -> ResultStatus {
    let path = validate_cstr!(path, AUTDStatus, ResultStatus);
    std::fs::File::options()
        .append(true)
        .create(true)
        .open(path)
        .map(|f| {
            tracing_subscriber::fmt()
                .with_writer(f)
                .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
                .with_ansi(false)
                .init();
            AUTDStatus::AUTDTrue
        })
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationAudioFileWav(
    path: *const c_char,
    loop_behavior: LoopBehavior,
) -> ResultModulation {
    let path = validate_cstr!(path, ModulationPtr, ResultModulation);
    Wav::new(path)
        .map(|m| m.with_loop_behavior(loop_behavior))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationAudioFileWavWithResample(
    path: *const c_char,
    loop_behavior: LoopBehavior,
    target: SamplingConfig,
    resample: DynSincInterpolator,
) -> ResultModulation {
    let path = validate_cstr!(path, ModulationPtr, ResultModulation);
    Wav::new_with_resample(path, target, resample)
        .map(|m| m.with_loop_behavior(loop_behavior))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationAudioFileRawPCM(
    path: *const c_char,
    config: SamplingConfig,
    loop_behavior: LoopBehavior,
) -> ResultModulation {
    let path = validate_cstr!(path, ModulationPtr, ResultModulation);
    RawPCM::new(path, config)
        .map(|m| m.with_loop_behavior(loop_behavior))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationAudioFileRawPCMWithResample(
    path: *const c_char,
    loop_behavior: LoopBehavior,
    src: f32,
    target: SamplingConfig,
    resample: DynSincInterpolator,
) -> ResultModulation {
    let path = validate_cstr!(path, ModulationPtr, ResultModulation);
    RawPCM::new_with_resample(path, src * Hz, target, resample)
        .map(|m| m.with_loop_behavior(loop_behavior))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationAudioFileCsv(
    path: *const c_char,
    config: SamplingConfig,
    deliminator: u8,
    loop_behavior: LoopBehavior,
) -> ResultModulation {
    let path = validate_cstr!(path, ModulationPtr, ResultModulation);
    Csv::new(path, config)
        .map(|m| {
            m.with_deliminator(deliminator)
                .with_loop_behavior(loop_behavior)
        })
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationAudioFileCsvWithResample(
    path: *const c_char,
    deliminator: u8,
    loop_behavior: LoopBehavior,
    src: f32,
    target: SamplingConfig,
    resample: DynSincInterpolator,
) -> ResultModulation {
    let path = validate_cstr!(path, ModulationPtr, ResultModulation);
    Csv::new_with_resample(path, src * Hz, target, resample)
        .map(|m| {
            m.with_deliminator(deliminator)
                .with_loop_behavior(loop_behavior)
        })
        .into()
}

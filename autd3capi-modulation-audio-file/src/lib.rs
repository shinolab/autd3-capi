#![allow(clippy::missing_safety_doc)]

use std::ffi::{c_char, CStr};

use autd3capi_driver::{
    driver::{defined::Hz, derive::ModulationProperty},
    *,
};

use autd3_modulation_audio_file::{Csv, RawPCM, Wav};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationWav(
    path: *const c_char,
    config: SamplingConfigWrap,
    loop_behavior: LoopBehavior,
) -> ResultModulation {
    match CStr::from_ptr(path).to_str().map(|path| {
        Wav::new(path)
            .with_sampling_config(config.into())
            .with_loop_behavior(loop_behavior.into())
    }) {
        Ok(v) => ResultModulation {
            result: v.into(),
            err_len: 0,
            err: std::ptr::null_mut(),
        },
        Err(e) => {
            let err = e.to_string();
            return ResultModulation {
                result: ModulationPtr(std::ptr::null()),
                err_len: err.as_bytes().len() as u32 + 1,
                err: Box::into_raw(Box::new(err)) as _,
            };
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationWavIsDefault(wav: ModulationPtr) -> bool {
    let m = take_mod!(wav, Wav);
    let default = Wav::new("");
    m.sampling_config() == default.sampling_config()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationRawPCM(
    path: *const c_char,
    sample_rate: u32,
    config: SamplingConfigWrap,
    loop_behavior: LoopBehavior,
) -> ResultModulation {
    match CStr::from_ptr(path).to_str().map(|path| {
        RawPCM::new(path, sample_rate * Hz)
            .with_sampling_config(config.into())
            .with_loop_behavior(loop_behavior.into())
    }) {
        Ok(v) => ResultModulation {
            result: v.into(),
            err_len: 0,
            err: std::ptr::null_mut(),
        },
        Err(e) => {
            let err = e.to_string();
            return ResultModulation {
                result: ModulationPtr(std::ptr::null()),
                err_len: err.as_bytes().len() as u32 + 1,
                err: Box::into_raw(Box::new(err)) as _,
            };
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationRawPCMIsDefault(rawpcm: ModulationPtr) -> bool {
    let m = take_mod!(rawpcm, RawPCM);
    let default = RawPCM::new("", 0 * Hz);
    m.sampling_config() == default.sampling_config()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationCsv(
    path: *const c_char,
    sample_rate: u32,
    config: SamplingConfigWrap,
    loop_behavior: LoopBehavior,
) -> ResultModulation {
    match CStr::from_ptr(path).to_str().map(|path| {
        Csv::new(path, sample_rate * Hz)
            .with_sampling_config(config.into())
            .with_loop_behavior(loop_behavior.into())
    }) {
        Ok(v) => ResultModulation {
            result: v.into(),
            err_len: 0,
            err: std::ptr::null_mut(),
        },
        Err(e) => {
            let err = e.to_string();
            return ResultModulation {
                result: ModulationPtr(std::ptr::null()),
                err_len: err.as_bytes().len() as u32 + 1,
                err: Box::into_raw(Box::new(err)) as _,
            };
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationCsvIsDefault(rawpcm: ModulationPtr) -> bool {
    let m = take_mod!(rawpcm, Csv);
    let default = Csv::new("", 0 * Hz);
    m.sampling_config() == default.sampling_config()
}

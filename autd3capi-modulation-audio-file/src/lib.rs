#![allow(clippy::missing_safety_doc)]

use std::ffi::{c_char, CStr};

use autd3::derive::SamplingConfig;
use autd3capi_driver::*;

use autd3_modulation_audio_file::{Csv, RawPCM, Wav};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationAudioFileWav(
    path: *const c_char,
    loop_behavior: LoopBehavior,
) -> ResultModulation {
    let path = match CStr::from_ptr(path).to_str() {
        Ok(v) => v,
        Err(e) => {
            let err = e.to_string();
            return ResultModulation {
                result: ModulationPtr(std::ptr::null()),
                err_len: err.as_bytes().len() as u32 + 1,
                err: ConstPtr(Box::into_raw(Box::new(err)) as _),
            };
        }
    };
    match Wav::new(path) {
        Ok(v) => ResultModulation {
            result: v.with_loop_behavior(loop_behavior.into()).into(),
            err_len: 0,
            err: ConstPtr(std::ptr::null_mut()),
        },
        Err(e) => {
            let err = e.to_string();
            return ResultModulation {
                result: ModulationPtr(std::ptr::null()),
                err_len: err.as_bytes().len() as u32 + 1,
                err: ConstPtr(Box::into_raw(Box::new(err)) as _),
            };
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationAudioFileRawPCM(
    path: *const c_char,
    config: SamplingConfig,
    loop_behavior: LoopBehavior,
) -> ResultModulation {
    let path = match CStr::from_ptr(path).to_str() {
        Ok(v) => v,
        Err(e) => {
            let err = e.to_string();
            return ResultModulation {
                result: ModulationPtr(std::ptr::null()),
                err_len: err.as_bytes().len() as u32 + 1,
                err: ConstPtr(Box::into_raw(Box::new(err)) as _),
            };
        }
    };
    RawPCM::new(path, config)
        .map(|m| m.with_loop_behavior(loop_behavior.into()))
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
    let path = match CStr::from_ptr(path).to_str() {
        Ok(v) => v,
        Err(e) => {
            let err = e.to_string();
            return ResultModulation {
                result: ModulationPtr(std::ptr::null()),
                err_len: err.as_bytes().len() as u32 + 1,
                err: ConstPtr(Box::into_raw(Box::new(err)) as _),
            };
        }
    };
    Csv::new(path, config)
        .map(|m| {
            m.with_deliminator(deliminator)
                .with_loop_behavior(loop_behavior.into())
        })
        .into()
}

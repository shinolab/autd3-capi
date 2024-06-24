#![allow(clippy::missing_safety_doc)]

use std::ffi::{c_char, CStr};

use autd3capi_driver::{
    driver::{defined::Hz, derive::ModulationProperty},
    *,
};

use autd3_modulation_audio_file::{Csv, RawPCM, Wav};

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationAudioFileSetUltrasoundFreq(f: u32) {
    autd3capi_driver::driver::set_ultrasound_freq(f * autd3capi_driver::driver::defined::Hz);
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationAudioFileWav(
    path: *const c_char,
    loop_behavior: LoopBehavior,
) -> ResultModulation {
    match CStr::from_ptr(path)
        .to_str()
        .map(|path| Wav::new(path).with_loop_behavior(loop_behavior.into()))
    {
        Ok(v) => ResultModulation {
            result: v.into(),
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
pub unsafe extern "C" fn AUTDModulationAudioFileWavIsDefault(wav: ModulationPtr) -> bool {
    let m = take_mod!(wav, Wav);
    let default = Wav::new("");
    m.sampling_config() == default.sampling_config()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationAudioFileRawPCM(
    path: *const c_char,
    sample_rate: u32,
    loop_behavior: LoopBehavior,
) -> ResultModulation {
    match CStr::from_ptr(path)
        .to_str()
        .map(|path| RawPCM::new(path, sample_rate * Hz).with_loop_behavior(loop_behavior.into()))
    {
        Ok(v) => ResultModulation {
            result: v.into(),
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
pub unsafe extern "C" fn AUTDModulationAudioFileCsv(
    path: *const c_char,
    sample_rate: u32,
    deliminator: u8,
    loop_behavior: LoopBehavior,
) -> ResultModulation {
    match CStr::from_ptr(path).to_str().map(|path| {
        Csv::new(path, sample_rate * Hz)
            .with_deliminator(deliminator)
            .with_loop_behavior(loop_behavior.into())
    }) {
        Ok(v) => ResultModulation {
            result: v.into(),
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

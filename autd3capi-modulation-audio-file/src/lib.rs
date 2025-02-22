#![allow(clippy::missing_safety_doc)]

use std::{convert::Infallible, ffi::c_char};

use autd3capi_driver::*;

use autd3_modulation_audio_file::{Csv, CsvOption, Wav};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDModulationAudioFileTracingInit() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}

#[unsafe(no_mangle)]
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

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDModulationAudioFileWav(path: *const c_char) -> ResultModulation {
    let path = validate_cstr!(path, ModulationPtr, ResultModulation);
    Wav::new(path).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDModulationAudioFileCsv(
    path: *const c_char,
    sampling_config: SamplingConfigWrap,
    delimiter: u8,
) -> ResultModulation {
    let path = validate_cstr!(path, ModulationPtr, ResultModulation);
    Result::<_, Infallible>::Ok(Csv {
        path: path.to_owned(),
        sampling_config,
        option: CsvOption { delimiter },
    })
    .into()
}

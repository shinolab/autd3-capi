#![allow(clippy::missing_safety_doc)]

use std::{convert::Infallible, ffi::c_char};

use autd3capi_driver::*;

use autd3_modulation_audio_file::{Csv, CsvOption, Wav};

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

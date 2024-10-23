use autd3capi_driver::{impl_ffi_result, ConstPtr};

use crate::{RecordPtr, SoundFieldPtr};

#[repr(C)]
pub struct ResultRecord {
    pub result: RecordPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl_ffi_result!(ResultRecord, RecordPtr);

#[repr(C)]
pub struct ResultSoundField {
    pub result: SoundFieldPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl_ffi_result!(ResultSoundField, SoundFieldPtr);

use autd3capi_driver::{GainPtr, autd3::gain::Null};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGainNull() -> GainPtr {
    Null.into()
}

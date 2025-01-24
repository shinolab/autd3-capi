use autd3capi_driver::{autd3::gain::Null, GainPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainNull() -> GainPtr {
    Null.into()
}

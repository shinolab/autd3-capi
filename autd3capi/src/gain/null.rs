use autd3capi_def::{autd3::gain::Null, GainPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainNull() -> GainPtr {
    GainPtr::new(Null::new())
}

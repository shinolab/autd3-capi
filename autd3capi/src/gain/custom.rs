use autd3capi_def::*;

#[no_mangle]
#[must_use]
#[allow(clippy::uninit_vec)]
pub unsafe extern "C" fn AUTDGainCustom() -> GainPtr {
    CustomGain::default().into()
}

#[no_mangle]
#[must_use]
#[allow(clippy::uninit_vec)]
pub unsafe extern "C" fn AUTDGainCustomSet(
    custom: GainPtr,
    dev_idx: u32,
    ptr: *const Drive,
    len: u32,
) -> GainPtr {
    take_gain!(custom, CustomGain)
        .set(
            dev_idx as _,
            vec_from_raw!(ptr, autd3capi_def::driver::common::Drive, len),
        )
        .into()
}

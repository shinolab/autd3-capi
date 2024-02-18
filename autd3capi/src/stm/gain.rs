#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{
    driver::datagram::{GainSTM, STMProps},
    *,
};

use super::STMPropsPtr;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGain(
    props: STMPropsPtr,
    segment: Segment,
    update_segment: bool,
    gains: *const GainPtr,
    size: u32,
    mode: GainSTMMode,
) -> ResultDatagram {
    GainSTM::<Box<dyn Gain>>::from_props_mode(*take!(props, STMProps), mode.into())
        .add_gains_from_iter((0..size as usize).map(|i| *take!(gains.add(i).read(), Box<G>)))
        .map(|stm| stm.with_segment(segment, update_segment))
        .into()
}

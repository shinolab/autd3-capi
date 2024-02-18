#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{
    driver::datagram::{ChangeGainSTMSegment, GainSTM, STMProps},
    *,
};

use super::STMPropsPtr;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGain(
    props: STMPropsPtr,
    gains: *const GainPtr,
    size: u32,
    mode: GainSTMMode,
) -> ResultGainSTM {
    GainSTM::<Box<dyn Gain>>::from_props_mode(*take!(props, STMProps), mode.into())
        .add_gains_from_iter((0..size as usize).map(|i| *take!(gains.add(i).read(), Box<G>)))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGainAddGain(stm: GainSTMPtr, gain: GainPtr) -> ResultDatagram {
    take!(stm, GainSTM<_>).add_gain(*take!(gain, Box<G>)).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGainIntoDatagramWithSegment(
    stm: GainSTMPtr,
    segment: Segment,
    update_segment: bool,
) -> DatagramPtr {
    take!(stm, GainSTM<Box<G>>)
        .with_segment(segment, update_segment)
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGainIntoDatagram(stm: GainSTMPtr) -> DatagramPtr {
    (*take!(stm, GainSTM<Box<G>>)).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramChangeGainSTMSegment(segment: Segment) -> DatagramPtr {
    ChangeGainSTMSegment::new(segment.into()).into()
}

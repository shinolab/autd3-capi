use autd3capi_driver::{
    driver::datagram::{GainSTM, STMProps, SwapSegment},
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
) -> GainSTMPtr {
    GainSTM::<Box<G>>::from_props_mode(*take!(props, STMProps), mode.into())
        .add_gains_from_iter((0..size as usize).map(|i| *take!(gains.add(i).read(), Box<G>)))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGainIntoDatagramWithSegment(
    stm: GainSTMPtr,
    segment: Segment,
) -> DatagramPtr {
    take!(stm, GainSTM<Box<G>>)
        .with_segment(segment, None)
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGainIntoDatagramWithSegmentTransition(
    stm: GainSTMPtr,
    segment: Segment,
    transition_mode: TransitionMode,
) -> DatagramPtr {
    take!(stm, GainSTM<Box<G>>)
        .with_segment(segment, Some(transition_mode))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGainIntoDatagram(stm: GainSTMPtr) -> DatagramPtr {
    (*take!(stm, GainSTM<Box<G>>)).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSwapSegmentGainSTM(
    segment: Segment,
    transition_mode: TransitionMode,
) -> DatagramPtr {
    SwapSegment::gain_stm(segment.into(), transition_mode.into()).into()
}

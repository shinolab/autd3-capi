use autd3capi_driver::{driver::datagram::GainSTM, *};
use driver::datagram::IntoDatagramWithSegmentTransition;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGain(
    config: STMConfigWrap,
    gains: *const GainPtr,
    size: u16,
) -> ResultGainSTM {
    GainSTM::<Box<G>>::new(
        config,
        (0..size as usize).map(|i| *take!(gains.add(i).read(), Box<G>)),
    )
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGainNearest(
    config: STMConfigWrap,
    gains: *const GainPtr,
    size: u16,
) -> ResultGainSTM {
    GainSTM::<Box<G>>::new_nearest(
        config,
        (0..size as usize).map(|i| *take!(gains.add(i).read(), Box<G>)),
    )
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGainWithMode(stm: GainSTMPtr, mode: GainSTMMode) -> GainSTMPtr {
    take!(stm, GainSTM<Box<G>>).with_mode(mode.into()).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGainWithLoopBehavior(
    stm: GainSTMPtr,
    loop_behavior: LoopBehavior,
) -> GainSTMPtr {
    take!(stm, GainSTM<Box<G>>)
        .with_loop_behavior(loop_behavior.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGainIntoDatagramWithSegment(
    stm: GainSTMPtr,
    segment: Segment,
) -> DatagramPtr {
    take!(stm, GainSTM<Box<G>>)
        .with_segment(segment.into(), None)
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGainIntoDatagramWithSegmentTransition(
    stm: GainSTMPtr,
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    take!(stm, GainSTM<Box<G>>)
        .with_segment(segment.into(), Some(transition_mode.into()))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGainIntoDatagram(stm: GainSTMPtr) -> DatagramPtr {
    (*take!(stm, GainSTM<Box<G>>)).into()
}

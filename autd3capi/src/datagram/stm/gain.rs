use autd3::{
    derive::{LoopBehavior, SamplingConfig, Segment},
    prelude::GainSTMMode,
};
use autd3capi_driver::{driver::datagram::GainSTM, *};
use driver::datagram::{BoxedGain, IntoDatagramWithSegment};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGain(
    config: SamplingConfig,
    gains: *const GainPtr,
    size: u16,
    mode: GainSTMMode,
    loop_behavior: LoopBehavior,
) -> ResultGainSTM {
    GainSTM::<Vec<BoxedGain>>::new(
        config,
        (0..size as usize).map(|i| *take!(gains.add(i).read(), BoxedGain)),
    )
    .map(|stm| stm.with_loop_behavior(loop_behavior))
    .map(|stm| stm.with_mode(mode))
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGainIntoDatagramWithSegment(
    stm: GainSTMPtr,
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    take!(stm, GainSTM<Vec<BoxedGain>>)
        .with_segment(segment, transition_mode.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGainIntoDatagram(stm: GainSTMPtr) -> DatagramPtr {
    (*take!(stm, GainSTM<Vec<BoxedGain>>)).into()
}

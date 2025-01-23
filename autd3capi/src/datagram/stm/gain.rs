use autd3capi_driver::{
    autd3::core::{datagram::Segment, modulation::SamplingConfig},
    autd3::prelude::GainSTMMode,
    driver::datagram::GainSTM,
    *,
};
use driver::datagram::BoxedGain;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGain(
    config: SamplingConfig,
    gains: *const GainPtr,
    size: u16,
    mode: GainSTMMode,
) -> ResultGainSTM {
    GainSTM::<Vec<BoxedGain>>::new(
        config,
        (0..size as usize).map(|i| *take!(gains.add(i).read(), BoxedGain)),
    )
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

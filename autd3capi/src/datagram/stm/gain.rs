use autd3capi_driver::{
    autd3::core::{datagram::Segment, modulation::SamplingConfig},
    driver::datagram::{BoxedGain, GainSTM, GainSTMOption, WithLoopBehavior, WithSegment},
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGain(
    config: SamplingConfig,
    gains: *const GainPtr,
    size: u16,
    option: GainSTMOption,
) -> GainSTMPtr {
    GainSTM::<Vec<BoxedGain>, SamplingConfig> {
        gains: (0..size as usize)
            .map(|i| *take!(gains.add(i).read(), BoxedGain))
            .collect(),
        config,
        option,
    }
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGainIntoDatagramWithSegment(
    stm: GainSTMPtr,
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    WithSegment {
        inner: *take!(stm, GainSTM<Vec<BoxedGain>, SamplingConfig>),
        segment,
        transition_mode: transition_mode.into(),
    }
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGainIntoDatagramWithLoopBehavior(
    stm: GainSTMPtr,
    segment: Segment,
    transition_mode: TransitionModeWrap,
    loop_behavior: LoopBehavior,
) -> DatagramPtr {
    WithLoopBehavior {
        inner: *take!(stm, GainSTM<Vec<BoxedGain>, SamplingConfig>),
        segment,
        transition_mode: transition_mode.into(),
        loop_behavior: loop_behavior.into(),
    }
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGainIntoDatagram(stm: GainSTMPtr) -> DatagramPtr {
    (*take!(stm, GainSTM<Vec<BoxedGain>, SamplingConfig>)).into()
}

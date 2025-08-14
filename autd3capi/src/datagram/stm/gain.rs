use std::num::NonZeroU16;

use autd3capi_driver::{
    autd3::core::firmware::{SamplingConfig, Segment},
    driver::datagram::{BoxedGain, GainSTM, GainSTMOption, WithFiniteLoop, WithSegment},
    *,
};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGain(
    config: SamplingConfigWrap,
    gains: *const GainPtr,
    size: u16,
    option: GainSTMOption,
) -> GainSTMPtr {
    GainSTM::<Vec<BoxedGain>, SamplingConfig> {
        gains: (0..size as usize)
            .map(|i| unsafe { *take!(gains.add(i).read(), BoxedGain) })
            .collect(),
        config: config.into(),
        option,
    }
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGainIntoDatagramWithSegment(
    stm: GainSTMPtr,
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    WithSegment {
        inner: unsafe { *take!(stm, GainSTM<Vec<BoxedGain>, SamplingConfig>) },
        segment,
        transition_mode,
    }
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGainIntoDatagramWithFiniteLoop(
    stm: GainSTMPtr,
    segment: Segment,
    transition_mode: TransitionModeWrap,
    loop_count: u16,
) -> DatagramPtr {
    WithFiniteLoop {
        inner: unsafe { *take!(stm, GainSTM<Vec<BoxedGain>, SamplingConfig>) },
        segment,
        transition_mode,
        loop_count: NonZeroU16::new(loop_count).unwrap(),
    }
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGainIntoDatagram(stm: GainSTMPtr) -> DatagramPtr {
    unsafe { *take!(stm, GainSTM<Vec<BoxedGain>, SamplingConfig>) }.into()
}

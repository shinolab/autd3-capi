use autd3capi_driver::{
    driver::{datagram::FociSTM, defined::Hz},
    *,
};
use driver::{datagram::IntoDatagramWithSegmentTransition, defined::ControlPoints};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFociFromFreq(
    freq: f32,
    points: ConstPtr,
    size: u16,
    n: u8,
) -> ResultFociSTM {
    seq_macro::seq!(N in 1..=8 {
        match n {
                #(N => FociSTM::from_freq(
                    freq * Hz,
                    vec_from_raw!(points, ControlPoints<N>, size)
                )
                .into(),)*
            _ => unreachable!(),
        }
    })
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFociFromFreqNearest(
    freq: f32,
    points: ConstPtr,
    size: u16,
    n: u8,
) -> ResultFociSTM {
    seq_macro::seq!(N in 1..=8 {
        match n {
                #(N => FociSTM::from_freq_nearest(
                    freq * Hz,
                    vec_from_raw!(points, ControlPoints<N>, size)
                )
                .into(),)*
            _ => unreachable!(),
        }
    })
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFociFromSamplingConfig(
    config: SamplingConfigWrap,
    points: ConstPtr,
    size: u16,
    n: u8,
) -> FociSTMPtr {
    seq_macro::seq!(N in 1..=8 {
        match n {
                #(N => FociSTM::from_sampling_config(
                    config.into(),
                    vec_from_raw!(points, ControlPoints<N>, size)
                )
                .into(),)*
            _ => unreachable!(),
        }
    })
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFociWithLoopBehavior(
    stm: FociSTMPtr,
    n: u8,
    loop_behavior: LoopBehavior,
) -> FociSTMPtr {
    seq_macro::seq!(N in 1..=8 {
        match n {
                #(N => take!(stm, FociSTM<N>)
                .with_loop_behavior(loop_behavior.into())
                .into(),)*
            _ => unreachable!(),
        }
    })
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFociIntoDatagramWithSegment(
    stm: FociSTMPtr,
    n: u8,
    segment: Segment,
) -> DatagramPtr {
    seq_macro::seq!(N in 1..=8 {
        match n {
                #(N => take!(stm, FociSTM<N>)
                .with_segment(segment.into(), None)
                .into(),)*
            _ => unreachable!(),
        }
    })
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFociIntoDatagramWithSegmentTransition(
    stm: FociSTMPtr,
    n: u8,
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    seq_macro::seq!(N in 1..=8 {
        match n {
                #(N => take!(stm, FociSTM<N>)
                .with_segment(segment.into(), Some(transition_mode.into()))
                .into(),)*
            _ => unreachable!(),
        }
    })
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFociIntoDatagram(stm: FociSTMPtr, n: u8) -> DatagramPtr {
    seq_macro::seq!(N in 1..=8 {
        match n {
                #(N => (*take!(stm, FociSTM<1>)).into(),)*
            _ => unreachable!(),
        }
    })
}

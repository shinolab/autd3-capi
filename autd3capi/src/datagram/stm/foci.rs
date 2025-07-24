use std::num::NonZeroU16;

use autd3capi_driver::{
    core::{datagram::Segment, sampling_config::SamplingConfig},
    driver::datagram::{ControlPoints, FociSTM, WithFiniteLoop, WithSegment},
    *,
};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFoci(
    config: SamplingConfigWrap,
    points: ConstPtr,
    size: u16,
    n: u8,
) -> FociSTMPtr {
    unsafe {
        seq_macro::seq!(N in 1..=8 {
            match n {
                    #(N => {
                        let points = points.0 as *const ControlPoints<N>;
                        FociSTM::<N, Vec<_>, SamplingConfig> {
                            foci: (0..size as usize).map(|i| (points.add(i).read())).collect(),
                            config: config.into(),
                        }
                        .into()
                    },)*
                _ => unreachable!(),
            }
        })
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFociIntoDatagramWithSegment(
    stm: FociSTMPtr,
    n: u8,
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    seq_macro::seq!(N in 1..=8 {
        match n {
                #(N => WithSegment {
                    inner: unsafe { *take!(stm, FociSTM<N, Vec<ControlPoints<N>>, SamplingConfig>) },
                    segment,
                    transition_mode
                }
                .into(),)*
            _ => unreachable!(),
        }
    })
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFociIntoDatagramWithFiniteLoop(
    stm: FociSTMPtr,
    n: u8,
    segment: Segment,
    transition_mode: TransitionModeWrap,
    loop_count: u16,
) -> DatagramPtr {
    seq_macro::seq!(N in 1..=8 {
        match n {
                #(N => WithFiniteLoop {
                    inner: unsafe { *take!(stm, FociSTM<N, Vec<ControlPoints<N>>, SamplingConfig>) },
                    segment,
                    transition_mode,
                    loop_count: NonZeroU16::new(loop_count).unwrap(),
                }
                .into(),)*
            _ => unreachable!(),
        }
    })
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFociIntoDatagram(stm: FociSTMPtr, n: u8) -> DatagramPtr {
    seq_macro::seq!(N in 1..=8 {
        match n {
                #(N => unsafe { *take!(stm, FociSTM<N, Vec<ControlPoints<N>>, SamplingConfig>) }.into(),)*
            _ => unreachable!(),
        }
    })
}

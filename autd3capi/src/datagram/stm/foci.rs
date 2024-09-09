use autd3::derive::SamplingConfig;
use autd3capi_driver::{driver::datagram::FociSTM, *};
use driver::{datagram::IntoDatagramWithSegmentTransition, defined::ControlPoints};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFoci(
    config: SamplingConfig,
    points: ConstPtr,
    size: u16,
    n: u8,
) -> ResultFociSTM {
    seq_macro::seq!(N in 1..=8 {
        match n {
                #(N => {
                    let points = points.0 as *const ControlPoints<N>;
                    FociSTM::new(
                        config,
                        (0..size as usize).map(|i| (points.add(i).read())),
                    )
                    .into()
                },)*
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
                #(N => (*take!(stm, FociSTM<N>)).into(),)*
            _ => unreachable!(),
        }
    })
}

use autd3capi_driver::{
    autd3::core::{datagram::Segment, modulation::SamplingConfig},
    driver::datagram::FociSTM,
    *,
};
use driver::firmware::operation::ControlPoints;

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
pub unsafe extern "C" fn AUTDSTMFociIntoDatagramWithSegment(
    stm: FociSTMPtr,
    n: u8,
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    seq_macro::seq!(N in 1..=8 {
        match n {
                #(N => take!(stm, FociSTM<N, Vec<ControlPoints<N>>>)
                .with_segment(segment, transition_mode.into())
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
                #(N => (*take!(stm, FociSTM<N, Vec<ControlPoints<N>>>)).into(),)*
            _ => unreachable!(),
        }
    })
}

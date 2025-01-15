use autd3capi_driver::{
    autd3::{core::datagram::Segment, prelude::IntoDatagramWithSegment},
    *,
};
use driver::datagram::BoxedGain;

pub mod bessel;
pub mod cache;
pub mod custom;
pub mod focus;
pub mod group;
pub mod null;
pub mod plane;
pub mod uniform;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainIntoDatagramWithSegment(
    gain: GainPtr,
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    (*take!(gain, BoxedGain))
        .with_segment(segment, transition_mode.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainIntoDatagram(gain: GainPtr) -> DatagramPtr {
    (*take!(gain, BoxedGain)).into()
}

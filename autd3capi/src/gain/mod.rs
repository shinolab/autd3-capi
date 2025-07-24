use autd3capi_driver::{autd3::core::datagram::Segment, *};
use driver::datagram::{BoxedGain, WithSegment};

pub mod bessel;
pub mod custom;
pub mod focus;
pub mod group;
pub mod null;
pub mod plane;
pub mod uniform;

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGainIntoDatagramWithSegment(
    gain: GainPtr,
    segment: Segment,
    transition_mode: TransitionModeWrap,
) -> DatagramPtr {
    WithSegment {
        inner: unsafe { *take!(gain, BoxedGain) },
        segment,
        transition_mode,
    }
    .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGainIntoDatagram(gain: GainPtr) -> DatagramPtr {
    unsafe { *take!(gain, BoxedGain) }.into()
}

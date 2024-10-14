use autd3capi_driver::{autd3::prelude::IntoDatagramWithSegment, *};

pub mod bessel;
pub mod cache;
pub mod custom;
pub mod focus;
pub mod group;
pub mod null;
pub mod plane;
pub mod transform;
pub mod uniform;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainIntoDatagramWithSegment(
    gain: GainPtr,
    segment: Segment,
    update_segment: bool,
) -> DatagramPtr {
    (*take!(gain, Box<G>))
        .with_segment(segment.into(), update_segment)
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainIntoDatagram(gain: GainPtr) -> DatagramPtr {
    (*take!(gain, Box<G>)).into()
}

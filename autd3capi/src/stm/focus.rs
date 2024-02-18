#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{
    driver::{
        datagram::{FocusSTM, STMProps},
        geometry::Vector3,
    },
    *,
};

use super::STMPropsPtr;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFocus(
    props: STMPropsPtr,
    segment: Segment,
    update_segment: bool,
    points: *const float,
    intensities: *const u8,
    size: u64,
) -> ResultDatagram {
    FocusSTM::from_props(*take!(props, STMProps))
        .add_foci_from_iter((0..size as usize).map(|i| {
            let p = Vector3::new(
                points.add(i * 3).read(),
                points.add(i * 3 + 1).read(),
                points.add(i * 3 + 2).read(),
            );
            let intensity = *intensities.add(i);
            (p, intensity)
        }))
        .map(|stm| stm.with_segment(segment, update_segment))
        .into()
}

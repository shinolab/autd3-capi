#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{
    driver::{
        datagram::{ChangeFocusSTMSegment, FocusSTM, STMProps},
        geometry::Vector3,
    },
    *,
};

use super::STMPropsPtr;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFocus(
    props: STMPropsPtr,
    points: *const f64,
    intensities: *const u8,
    size: u64,
) -> ResultFocusSTM {
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
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFocusIntoDatagramWithSegment(
    stm: FocusSTMPtr,
    segment: Segment,
    update_segment: bool,
) -> DatagramPtr {
    take!(stm, FocusSTM)
        .with_segment(segment, update_segment)
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMFocusIntoDatagram(stm: FocusSTMPtr) -> DatagramPtr {
    (*take!(stm, FocusSTM)).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramChangeFocusSTMSegment(segment: Segment) -> DatagramPtr {
    ChangeFocusSTMSegment::new(segment.into()).into()
}

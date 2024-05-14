use autd3capi_driver::{ConstPtr, DatagramPtr, DynamicForceFan, GeometryPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramForceFan(
    f: ConstPtr,
    context: ConstPtr,
    geometry: GeometryPtr,
) -> DatagramPtr {
    DynamicForceFan::new(f, context, geometry).into()
}

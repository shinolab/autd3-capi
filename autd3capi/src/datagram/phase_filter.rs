use autd3capi_driver::{ConstPtr, DatagramPtr, DynamicPhaseFilter, GeometryPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramPhaseFilter(
    f: ConstPtr,
    context: ConstPtr,
    geometry: GeometryPtr,
) -> DatagramPtr {
    DynamicPhaseFilter::additive(f, context, geometry).into()
}

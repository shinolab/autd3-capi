use autd3capi_driver::{ConstPtr, DatagramPtr, DynamicPhaseFilter, GeometryPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramPhaseFilterAdditive(
    f: ConstPtr,
    context: ConstPtr,
    geometry: GeometryPtr,
) -> DatagramPtr {
    DynamicPhaseFilter {
        f,
        context,
        geometry,
    }
    .into()
}

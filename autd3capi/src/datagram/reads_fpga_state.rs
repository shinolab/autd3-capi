use autd3capi_driver::{ConstPtr, DatagramPtr, DynamicReadsFPGAState, GeometryPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramReadsFPGAState(
    f: ConstPtr,
    context: ConstPtr,
    geometry: GeometryPtr,
) -> DatagramPtr {
    DynamicReadsFPGAState {
        f,
        context,
        geometry,
    }
    .into()
}

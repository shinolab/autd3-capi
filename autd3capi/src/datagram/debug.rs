use autd3capi_driver::{ConstPtr, DatagramPtr, DynamicDebugSettings, GeometryPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramDebugSettings(
    f: ConstPtr,
    context: ConstPtr,
    geometry: GeometryPtr,
) -> DatagramPtr {
    DynamicDebugSettings {
        f,
        context,
        geometry,
    }
    .into()
}

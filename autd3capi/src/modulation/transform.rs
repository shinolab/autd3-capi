#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{driver::datagram::ModulationTransform, *};
use driver::common::EmitIntensity;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationWithTransform(
    m: ModulationPtr,
    f: ConstPtr,
    context: ConstPtr,
) -> ModulationPtr {
    ModulationTransform::new(*take!(m, Box<M>), move |i, d| {
        EmitIntensity::new((std::mem::transmute::<
            _,
            unsafe extern "C" fn(ConstPtr, u32, u8) -> u8,
        >(f))(context, i as u32, d.value()))
    })
    .into()
}

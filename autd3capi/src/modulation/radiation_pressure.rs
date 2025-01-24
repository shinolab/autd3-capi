use autd3::modulation::RadiationPressure;
use autd3capi_driver::*;
use driver::datagram::BoxedModulation;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationWithRadiationPressure(m: ModulationPtr) -> ModulationPtr {
    RadiationPressure::new(*take!(m, BoxedModulation)).into()
}

#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{
    autd3::modulation::{Fourier, Sine},
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationFourier(m: ModulationPtr) -> ModulationPtr {
    Fourier::from(**take_mod!(m, Sine)).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationFourierAddComponent(
    fourier: ModulationPtr,
    m: ModulationPtr,
) -> ModulationPtr {
    take_mod!(fourier, Fourier)
        .add_component(**take_mod!(m, Sine))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationFourierAddComponents(
    fourier: ModulationPtr,
    components: *const ModulationPtr,
    size: u32,
) -> ModulationPtr {
    take_mod!(fourier, Fourier)
        .add_components_from_iter(
            (0..size as usize).map(|i| **take_mod!(components.add(i).read(), Sine)),
        )
        .into()
}

use autd3::modulation::Fir;
use autd3capi_driver::*;
use driver::datagram::BoxedModulation;

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDModulationWithFir(
    m: ModulationPtr,
    coef: *const f32,
    n_tap: u32,
) -> ModulationPtr {
    unsafe {
        Fir {
            target: *take!(m, BoxedModulation),
            coef: (0..n_tap as usize).map(|i| coef.add(i).read()).collect(),
        }
        .into()
    }
}

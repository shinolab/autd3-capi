#![allow(clippy::missing_safety_doc)]

use autd3capi_driver::{
    autd3::modulation::{Fourier, Sine, SineOption},
    driver::defined::Hz,
    *,
};

#[repr(C)]
pub struct FourierOption {
    pub has_scale_factor: bool,
    pub scale_factor: f32,
    pub clamp: bool,
    pub offset: u8,
}

impl From<FourierOption> for autd3::modulation::FourierOption {
    fn from(option: FourierOption) -> Self {
        autd3::modulation::FourierOption {
            scale_factor: option.has_scale_factor.then_some(option.scale_factor),
            clamp: option.clamp,
            offset: option.offset,
        }
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDModulationFourierExact(
    sine_freq: *const u32,
    sine_clamp: *const SineOption,
    size: u32,
    option: FourierOption,
) -> ModulationPtr {
    unsafe {
        Fourier {
            components: (0..size as usize)
                .map(|i| Sine {
                    freq: sine_freq.add(i).read() * Hz,
                    option: sine_clamp.add(i).read(),
                })
                .collect(),
            option: option.into(),
        }
        .into()
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDModulationFourierExactFloat(
    sine_freq: *const f32,
    sine_clamp: *const SineOption,
    size: u32,
    option: FourierOption,
) -> ModulationPtr {
    unsafe {
        Fourier {
            components: (0..size as usize)
                .map(|i| Sine {
                    freq: sine_freq.add(i).read() * Hz,
                    option: sine_clamp.add(i).read(),
                })
                .collect(),
            option: option.into(),
        }
        .into()
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDModulationFourierNearest(
    sine_freq: *const f32,
    sine_clamp: *const SineOption,
    size: u32,
    option: FourierOption,
) -> ModulationPtr {
    unsafe {
        Fourier {
            components: (0..size as usize)
                .map(|i| {
                    Sine {
                        freq: sine_freq.add(i).read() * Hz,
                        option: sine_clamp.add(i).read(),
                    }
                    .into_nearest()
                })
                .collect(),
            option: option.into(),
        }
        .into()
    }
}

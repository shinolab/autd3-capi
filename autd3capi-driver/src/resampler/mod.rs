use std::num::NonZeroUsize;

use autd3::modulation::resample::{Blackman, Rectangular, Resampler, SincInterpolation};

#[repr(u32)]
#[derive(Debug)]
pub enum DynWindow {
    Rectangular = 0,
    Blackman = 1,
}

#[repr(C)]
#[derive(Debug)]
pub struct DynSincInterpolator {
    pub window: DynWindow,
    pub window_size: u32,
}

impl Resampler for DynSincInterpolator {
    fn upsample(&self, buffer: &[u8], ratio: f64) -> Vec<u8> {
        match self.window {
            DynWindow::Rectangular => SincInterpolation {
                window: Rectangular {
                    size: NonZeroUsize::new(self.window_size as _).unwrap(),
                },
            }
            .upsample(buffer, ratio),
            DynWindow::Blackman => SincInterpolation {
                window: Blackman {
                    size: NonZeroUsize::new(self.window_size as _).unwrap(),
                },
            }
            .upsample(buffer, ratio),
        }
    }

    fn downsample(&self, buffer: &[u8], ratio: f64) -> Vec<u8> {
        match self.window {
            DynWindow::Rectangular => SincInterpolation {
                window: Rectangular {
                    size: NonZeroUsize::new(self.window_size as _).unwrap(),
                },
            }
            .downsample(buffer, ratio),
            DynWindow::Blackman => SincInterpolation {
                window: Blackman {
                    size: NonZeroUsize::new(self.window_size as _).unwrap(),
                },
            }
            .downsample(buffer, ratio),
        }
    }
}

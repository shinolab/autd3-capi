use std::ffi::{c_char, CStr};

use autd3_link_visualizer::{ListedColorMap, PlotConfig, PlottersBackend, Visualizer};
use autd3capi_def::{
    driver::acoustics::directivity::{Sphere, T4010A1},
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerSpherePlotters(
    use_gpu: bool,
    gpu_idx: i32,
) -> LinkBuilderPtr {
    let mut builder = Visualizer::builder()
        .with_directivity::<Sphere>()
        .with_backend::<PlottersBackend>();
    if use_gpu {
        builder = builder.with_gpu(gpu_idx);
    }
    SyncLinkBuilder::new(builder)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerT4010A1Plotters(
    use_gpu: bool,
    gpu_idx: i32,
) -> LinkBuilderPtr {
    let mut builder = Visualizer::builder()
        .with_directivity::<T4010A1>()
        .with_backend::<PlottersBackend>();
    if use_gpu {
        builder = builder.with_gpu(gpu_idx);
    }
    SyncLinkBuilder::new(builder)
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PlotConfigPtr(pub ConstPtr);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ResultPlotConfig {
    pub result: PlotConfigPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

#[repr(u8)]
pub enum CMap {
    Jet = 0,
    Viridis = 1,
    Magma = 2,
    Inferno = 3,
    Plasma = 4,
    Cividis = 5,
    Turbo = 6,
    Circle = 7,
    Bluered = 8,
    Breeze = 9,
    Mist = 10,
    Earth = 11,
    Hell = 12,
}

impl From<CMap> for ListedColorMap {
    fn from(value: CMap) -> Self {
        match value {
            CMap::Jet => autd3_link_visualizer::colormap::jet(),
            CMap::Viridis => ListedColorMap::viridis(),
            CMap::Magma => ListedColorMap::magma(),
            CMap::Inferno => ListedColorMap::inferno(),
            CMap::Plasma => ListedColorMap::plasma(),
            CMap::Cividis => ListedColorMap::cividis(),
            CMap::Turbo => ListedColorMap::turbo(),
            CMap::Circle => ListedColorMap::circle(),
            CMap::Bluered => ListedColorMap::bluered(),
            CMap::Breeze => ListedColorMap::breeze(),
            CMap::Mist => ListedColorMap::mist(),
            CMap::Earth => ListedColorMap::earth(),
            CMap::Hell => ListedColorMap::hell(),
        }
    }
}

#[no_mangle]
#[must_use]
#[allow(clippy::box_default)]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotConfig(
    width: u32,
    height: u32,
    cbar_size: float,
    font_size: u32,
    label_area_size: u32,
    margin: u32,
    ticks_step: float,
    cmap: CMap,
    fname: *const c_char,
) -> ResultPlotConfig {
    let fname = match CStr::from_ptr(fname).to_str() {
        Ok(v) => v.to_owned(),
        Err(e) => {
            let err = e.to_string();
            return ResultPlotConfig {
                result: PlotConfigPtr(std::ptr::null()),
                err_len: err.as_bytes().len() as u32 + 1,
                err: Box::into_raw(Box::new(err)) as _,
            };
        }
    };
    ResultPlotConfig {
        result: PlotConfigPtr(Box::into_raw(Box::new(PlotConfig {
            figsize: (width, height),
            cbar_size,
            font_size,
            label_area_size,
            margin,
            ticks_step,
            cmap: cmap.into(),
            fname: fname.into(),
        })) as _),
        err_len: 0,
        err: std::ptr::null_mut(),
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotConfigDefaultFigSizeWidth() -> u32 {
    PlotConfig::default().figsize.0
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotConfigDefaultFigSizeHeight() -> u32 {
    PlotConfig::default().figsize.1
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotConfigDefaultCBarSize() -> float {
    PlotConfig::default().cbar_size
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotConfigDefaultFontSize() -> u32 {
    PlotConfig::default().font_size
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotConfigDefaultLabelAreaSize() -> u32 {
    PlotConfig::default().label_area_size
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotConfigDefaultMargin() -> u32 {
    PlotConfig::default().margin
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotConfigDefaultTicksStep() -> float {
    PlotConfig::default().ticks_step
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotConfigDefaultCMap() -> CMap {
    CMap::Jet // TODO
}

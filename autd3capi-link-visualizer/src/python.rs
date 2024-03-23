use std::ffi::{c_char, CStr};

use autd3_link_visualizer::{PyPlotConfig, PythonBackend, Visualizer};
use autd3capi_def::{
    driver::acoustics::directivity::{Sphere, T4010A1},
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerSpherePython(
    use_gpu: bool,
    gpu_idx: i32,
) -> LinkBuilderPtr {
    let mut builder = Visualizer::builder()
        .with_directivity::<Sphere>()
        .with_backend::<PythonBackend>();
    if use_gpu {
        builder = builder.with_gpu(gpu_idx);
    }
    SyncLinkBuilder::new(builder)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerT4010A1Python(
    use_gpu: bool,
    gpu_idx: i32,
) -> LinkBuilderPtr {
    let mut builder = Visualizer::builder()
        .with_directivity::<T4010A1>()
        .with_backend::<PythonBackend>();
    if use_gpu {
        builder = builder.with_gpu(gpu_idx);
    }
    SyncLinkBuilder::new(builder)
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PyPlotConfigPtr(pub ConstPtr);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ResultPyPlotConfig {
    pub result: PyPlotConfigPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

#[no_mangle]
#[must_use]
#[allow(clippy::box_default)]
pub unsafe extern "C" fn AUTDLinkVisualizerPyPlotConfig(
    width: i32,
    height: i32,
    dpi: i32,
    cbar_position: *const c_char,
    cbar_size: *const c_char,
    cbar_pad: *const c_char,
    fontsize: i32,
    ticks_step: f64,
    cmap: *const c_char,
    show: bool,
    fname: *const c_char,
) -> ResultPyPlotConfig {
    macro_rules! to_stirng {
        ($char:expr) => {
            match CStr::from_ptr($char).to_str() {
                Ok(v) => v.to_owned(),
                Err(e) => {
                    let err = e.to_string();
                    return ResultPyPlotConfig {
                        result: PyPlotConfigPtr(std::ptr::null()),
                        err_len: err.as_bytes().len() as u32 + 1,
                        err: Box::into_raw(Box::new(err)) as _,
                    };
                }
            }
        };
    }
    let cbar_position = to_stirng!(cbar_position);
    let cbar_size = to_stirng!(cbar_size);
    let cbar_pad = to_stirng!(cbar_pad);
    let cmap = to_stirng!(cmap);
    let fname = to_stirng!(fname);
    ResultPyPlotConfig {
        result: PyPlotConfigPtr(Box::into_raw(Box::new(PyPlotConfig {
            figsize: (width, height),
            dpi,
            cbar_position,
            cbar_size,
            cbar_pad,
            fontsize,
            ticks_step,
            cmap,
            show,
            fname: fname.into(),
        })) as _),
        err_len: 0,
        err: std::ptr::null_mut(),
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPyPlotConfigIsDefault(config: PyPlotConfigPtr) -> bool {
    *take!(config, PyPlotConfig) == PyPlotConfig::default()
}

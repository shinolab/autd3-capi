use std::ffi::{c_char, CStr, CString};

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
    ticks_step: float,
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
pub unsafe extern "C" fn AUTDLinkVisualizerPyPlotConfigDefaultFigSizeWidth() -> i32 {
    PyPlotConfig::default().figsize.0
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPyPlotConfigDefaultFigSizeHeight() -> i32 {
    PyPlotConfig::default().figsize.1
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPyPlotConfigDefaultDPI() -> i32 {
    PyPlotConfig::default().dpi
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPyPlotConfigDefaultCBarPosition(
    cbar: *mut c_char,
) -> u32 {
    let config = PyPlotConfig::default().cbar_position;
    if !cbar.is_null() {
        let c_string: CString = CString::new(config.as_str()).unwrap();
        let c_str: &CStr = c_string.as_c_str();
        libc::strcpy(cbar, c_str.as_ptr());
    }
    return config.as_bytes().len() as u32 + 1;
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPyPlotConfigDefaultCBarSize(cbar: *mut c_char) -> u32 {
    let config = PyPlotConfig::default().cbar_size;
    if !cbar.is_null() {
        let c_string: CString = CString::new(config.as_str()).unwrap();
        let c_str: &CStr = c_string.as_c_str();
        libc::strcpy(cbar, c_str.as_ptr());
    }
    return config.as_bytes().len() as u32 + 1;
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPyPlotConfigDefaultCBarPad(cbar: *mut c_char) -> u32 {
    let config = PyPlotConfig::default().cbar_size;
    if !cbar.is_null() {
        let c_string: CString = CString::new(config.as_str()).unwrap();
        let c_str: &CStr = c_string.as_c_str();
        libc::strcpy(cbar, c_str.as_ptr());
    }
    return config.as_bytes().len() as u32 + 1;
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPyPlotConfigDefaultFontSize() -> i32 {
    PyPlotConfig::default().fontsize
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPyPlotConfigDefaultTicksStep() -> float {
    PyPlotConfig::default().ticks_step
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPyPlotConfigDefaultCMap(cmap: *mut c_char) -> u32 {
    let config = PyPlotConfig::default().cmap;
    if !cmap.is_null() {
        let c_string: CString = CString::new(config.as_str()).unwrap();
        let c_str: &CStr = c_string.as_c_str();
        libc::strcpy(cmap, c_str.as_ptr());
    }
    return config.as_bytes().len() as u32 + 1;
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPyPlotConfigDefaultShow() -> bool {
    PyPlotConfig::default().show
}

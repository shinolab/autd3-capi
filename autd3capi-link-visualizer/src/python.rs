/*
 * File: python.rs
 * Project: src
 * Created Date: 13/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

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
    LinkBuilderPtr::new(builder)
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
    LinkBuilderPtr::new(builder)
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
pub unsafe extern "C" fn AUTDLinkVisualizerPyPlotConfigDefault() -> PyPlotConfigPtr {
    PyPlotConfigPtr(Box::into_raw(Box::new(PyPlotConfig::default())) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPyPlotConfigWithFigSize(
    config: PyPlotConfigPtr,
    width: i32,
    height: i32,
) -> PyPlotConfigPtr {
    if config.0.is_null() {
        return PyPlotConfigPtr(std::ptr::null());
    }
    let config = *Box::from_raw(config.0 as *mut PyPlotConfig);
    PyPlotConfigPtr(Box::into_raw(Box::new(PyPlotConfig {
        figsize: (width, height),
        ..config
    })) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPyPlotConfigWithDPI(
    config: PyPlotConfigPtr,
    dpi: i32,
) -> PyPlotConfigPtr {
    if config.0.is_null() {
        return PyPlotConfigPtr(std::ptr::null());
    }
    let config = *Box::from_raw(config.0 as *mut PyPlotConfig);
    PyPlotConfigPtr(Box::into_raw(Box::new(PyPlotConfig { dpi, ..config })) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPyPlotConfigWithCBarPosition(
    config: PyPlotConfigPtr,
    cbar_position: *const c_char,
) -> ResultPyPlotConfig {
    let cbar_position = match CStr::from_ptr(cbar_position).to_str() {
        Ok(v) => v.to_owned(),
        Err(e) => {
            let err = e.to_string();
            return ResultPyPlotConfig {
                result: PyPlotConfigPtr(std::ptr::null()),
                err_len: err.as_bytes().len() as u32 + 1,
                err: Box::into_raw(Box::new(err)) as _,
            };
        }
    };
    let config = *Box::from_raw(config.0 as *mut PyPlotConfig);
    ResultPyPlotConfig {
        result: PyPlotConfigPtr(Box::into_raw(Box::new(PyPlotConfig {
            cbar_position,
            ..config
        })) as _),
        err_len: 0,
        err: std::ptr::null_mut(),
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPyPlotConfigWithCBarSize(
    config: PyPlotConfigPtr,
    cbar_size: *const c_char,
) -> ResultPyPlotConfig {
    let cbar_size = match CStr::from_ptr(cbar_size).to_str() {
        Ok(v) => v.to_owned(),
        Err(e) => {
            let err = e.to_string();
            return ResultPyPlotConfig {
                result: PyPlotConfigPtr(std::ptr::null()),
                err_len: err.as_bytes().len() as u32 + 1,
                err: Box::into_raw(Box::new(err)) as _,
            };
        }
    };
    let config = *Box::from_raw(config.0 as *mut PyPlotConfig);
    ResultPyPlotConfig {
        result: PyPlotConfigPtr(Box::into_raw(Box::new(PyPlotConfig {
            cbar_size,
            ..config
        })) as _),
        err_len: 0,
        err: std::ptr::null_mut(),
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPyPlotConfigWithCBarPad(
    config: PyPlotConfigPtr,
    cbar_pad: *const c_char,
) -> ResultPyPlotConfig {
    let cbar_pad = match CStr::from_ptr(cbar_pad).to_str() {
        Ok(v) => v.to_owned(),
        Err(e) => {
            let err = e.to_string();
            return ResultPyPlotConfig {
                result: PyPlotConfigPtr(std::ptr::null()),
                err_len: err.as_bytes().len() as u32 + 1,
                err: Box::into_raw(Box::new(err)) as _,
            };
        }
    };
    let config = *Box::from_raw(config.0 as *mut PyPlotConfig);
    ResultPyPlotConfig {
        result: PyPlotConfigPtr(Box::into_raw(Box::new(PyPlotConfig { cbar_pad, ..config })) as _),
        err_len: 0,
        err: std::ptr::null_mut(),
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPyPlotConfigWithFontSize(
    config: PyPlotConfigPtr,
    fontsize: i32,
) -> PyPlotConfigPtr {
    if config.0.is_null() {
        return PyPlotConfigPtr(std::ptr::null());
    }
    let config = *Box::from_raw(config.0 as *mut PyPlotConfig);
    PyPlotConfigPtr(Box::into_raw(Box::new(PyPlotConfig { fontsize, ..config })) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPyPlotConfigWithTicksStep(
    config: PyPlotConfigPtr,
    ticks_step: float,
) -> PyPlotConfigPtr {
    if config.0.is_null() {
        return PyPlotConfigPtr(std::ptr::null());
    }
    let config = *Box::from_raw(config.0 as *mut PyPlotConfig);
    PyPlotConfigPtr(Box::into_raw(Box::new(PyPlotConfig {
        ticks_step,
        ..config
    })) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPyPlotConfigWithCMap(
    config: PyPlotConfigPtr,
    cmap: *const c_char,
) -> ResultPyPlotConfig {
    let cmap = match CStr::from_ptr(cmap).to_str() {
        Ok(v) => v.to_owned(),
        Err(e) => {
            let err = e.to_string();
            return ResultPyPlotConfig {
                result: PyPlotConfigPtr(std::ptr::null()),
                err_len: err.as_bytes().len() as u32 + 1,
                err: Box::into_raw(Box::new(err)) as _,
            };
        }
    };
    let config = *Box::from_raw(config.0 as *mut PyPlotConfig);
    ResultPyPlotConfig {
        result: PyPlotConfigPtr(Box::into_raw(Box::new(PyPlotConfig { cmap, ..config })) as _),
        err_len: 0,
        err: std::ptr::null_mut(),
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPyPlotConfigWithShow(
    config: PyPlotConfigPtr,
    show: bool,
) -> PyPlotConfigPtr {
    if config.0.is_null() {
        return PyPlotConfigPtr(std::ptr::null());
    }
    let config = *Box::from_raw(config.0 as *mut PyPlotConfig);
    PyPlotConfigPtr(Box::into_raw(Box::new(PyPlotConfig { show, ..config })) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPyPlotConfigWithFName(
    config: PyPlotConfigPtr,
    fname: *const c_char,
) -> ResultPyPlotConfig {
    let fname = match CStr::from_ptr(fname).to_str() {
        Ok(v) => v.to_owned(),
        Err(e) => {
            let err = e.to_string();
            return ResultPyPlotConfig {
                result: PyPlotConfigPtr(std::ptr::null()),
                err_len: err.as_bytes().len() as u32 + 1,
                err: Box::into_raw(Box::new(err)) as _,
            };
        }
    };
    let config = *Box::from_raw(config.0 as *mut PyPlotConfig);
    ResultPyPlotConfig {
        result: PyPlotConfigPtr(Box::into_raw(Box::new(PyPlotConfig {
            fname: fname.into(),
            ..config
        })) as _),
        err_len: 0,
        err: std::ptr::null_mut(),
    }
}

#![allow(clippy::missing_safety_doc)]

pub mod null;
pub mod plotters;
pub mod python;

use autd3_link_visualizer::{
    NullBackend, NullPlotConfig, PlotConfig, PlotRange, PlottersBackend, PyPlotConfig,
    PythonBackend, Visualizer,
};
use autd3capi_def::{
    driver::acoustics::directivity::{Sphere, T4010A1},
    *,
};

#[repr(u8)]
pub enum Backend {
    Plotters = 0,
    Python = 1,
    Null = 2,
}

#[repr(u8)]
pub enum Directivity {
    Sphere = 0,
    T4010A1 = 1,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ConfigPtr(pub ConstPtr);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PlotRangePtr(pub ConstPtr);

impl_ptr!(PlotRangePtr, PlotRange);

macro_rules! match_visualizer {
    ($b:expr, $d:expr, $v: expr, $call:tt,  $( $args:expr ),*) => {
        match $b {
            Backend::Plotters => match $d {
                Directivity::Sphere => $v.cast::<Visualizer<Sphere, PlottersBackend>>().$call( $($args),*),
                Directivity::T4010A1 => $v.cast::<Visualizer<T4010A1, PlottersBackend>>().$call( $($args),*),
            },
            Backend::Python =>  match $d {
                Directivity::Sphere =>  $v.cast::<Visualizer<Sphere, PythonBackend>>().$call($($args),*),
                Directivity::T4010A1 =>  $v.cast::<Visualizer<T4010A1, PythonBackend>>().$call( $($args),*),
            },
            Backend::Null =>  match $d {
                Directivity::Sphere =>  $v.cast::<Visualizer<Sphere, NullBackend>>().$call($($args),*),
                Directivity::T4010A1 =>  $v.cast::<Visualizer<T4010A1, NullBackend>>().$call( $($args),*),
            },
        }
    };
}

macro_rules! match_visualizer_plot {
    ($b:expr, $d:expr, $v: expr, $call:tt, $config:expr, $( $args:expr ),*) => {
        match $b {
            Backend::Plotters => match $d {
                Directivity::Sphere =>  $v.cast::<Visualizer<Sphere, PlottersBackend>>().$call(*take!($config, PlotConfig), $($args),*),
                Directivity::T4010A1 =>  $v.cast::<Visualizer<T4010A1, PlottersBackend>>().$call(*take!($config, PlotConfig), $($args),*),
            },
            Backend::Python =>  match $d {
                Directivity::Sphere =>  $v.cast::<Visualizer<Sphere, PythonBackend>>().$call(*take!($config, PyPlotConfig), $($args),*),
                Directivity::T4010A1 =>  $v.cast::<Visualizer<T4010A1, PythonBackend>>().$call(*take!($config, PyPlotConfig), $($args),*),
            },
            Backend::Null =>  match $d {
                Directivity::Sphere =>  $v.cast::<Visualizer<Sphere, NullBackend>>().$call(*take!($config, NullPlotConfig), $($args),*),
                Directivity::T4010A1 =>  $v.cast::<Visualizer<T4010A1, NullBackend>>().$call(*take!($config, NullPlotConfig), $($args),*),
            },
        }
    };
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotRange(
    x_min: float,
    x_max: float,
    y_min: float,
    y_max: float,
    z_min: float,
    z_max: float,
    resolution: float,
) -> PlotRangePtr {
    PlotRangePtr(Box::into_raw(Box::new(PlotRange {
        x_range: x_min..x_max,
        y_range: y_min..y_max,
        z_range: z_min..z_max,
        resolution,
    })) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotRangeObservePointsLen(range: PlotRangePtr) -> u64 {
    let n = |range: &std::ops::Range<float>, resolution: float| -> usize {
        ((range.end - range.start) / resolution).floor() as usize + 1
    };
    let nx = n(&range.x_range, range.resolution);
    let ny = n(&range.y_range, range.resolution);
    let nz = n(&range.z_range, range.resolution);
    (nx * ny * nz) as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotRangeObservePoints(
    range: PlotRangePtr,
    points: *mut float,
) {
    let range = take!(range, PlotRange);
    let observe_points = range.observe_points();
    std::ptr::copy_nonoverlapping(
        observe_points.as_ptr() as *const float,
        points,
        observe_points.len() * 3,
    );
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkVisualizerPhasesOf(
    visualizer: LinkPtr,
    backend: Backend,
    directivity: Directivity,
    idx: u32,
    buf: *mut u8,
) -> u32 {
    let idx = idx as usize;
    let m = match_visualizer!(backend, directivity, visualizer, phases_of, idx);
    if !buf.is_null() {
        std::ptr::copy_nonoverlapping(m.as_ptr(), buf, m.len());
    }
    m.len() as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkVisualizerIntensitiesOf(
    visualizer: LinkPtr,
    backend: Backend,
    directivity: Directivity,
    idx: u32,
    buf: *mut u8,
) -> u32 {
    let idx = idx as usize;
    let m = match_visualizer!(backend, directivity, visualizer, intensities_of, idx);
    if !buf.is_null() {
        std::ptr::copy_nonoverlapping(m.as_ptr(), buf, m.len());
    }
    m.len() as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkVisualizerModulation(
    visualizer: LinkPtr,
    backend: Backend,
    directivity: Directivity,
    buf: *mut u8,
) -> u32 {
    let m = match_visualizer!(backend, directivity, visualizer, modulation,);
    if !buf.is_null() {
        std::ptr::copy_nonoverlapping(m.as_ptr(), buf, m.len());
    }
    m.len() as _
}

macro_rules! into_result {
    ($r:expr) => {
        match $r {
            Ok(_) => ResultI32 {
                result: AUTD3_TRUE,
                err_len: 0,
                err: std::ptr::null_mut(),
            },
            Err(e) => {
                let err = e.to_string();
                ResultI32 {
                    result: 0,
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: Box::into_raw(Box::new(err)) as _,
                }
            }
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkVisualizerCalcFieldOf(
    visualizer: LinkPtr,
    backend: Backend,
    directivity: Directivity,
    points: *const float,
    points_len: u32,
    geometry: GeometryPtr,
    idx: u32,
    buf: *mut float,
) -> ResultI32 {
    let idx = idx as usize;
    let len = points_len as usize;
    let points = std::slice::from_raw_parts(points as *const Vector3, len);
    into_result!(match_visualizer!(
        backend,
        directivity,
        visualizer,
        calc_field_of,
        points.iter(),
        &geometry,
        idx
    )
    .and_then(|m| {
        std::ptr::copy_nonoverlapping(m.as_ptr(), buf as *mut _, m.len());
        Ok(())
    }))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotFieldOf(
    visualizer: LinkPtr,
    backend: Backend,
    directivity: Directivity,
    config: ConfigPtr,
    range: PlotRangePtr,
    geometry: GeometryPtr,
    idx: u32,
) -> ResultI32 {
    let idx = idx as usize;
    into_result!(match_visualizer_plot!(
        backend,
        directivity,
        visualizer,
        plot_field_of,
        config,
        *take!(range, PlotRange),
        &geometry,
        idx
    ))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotPhaseOf(
    visualizer: LinkPtr,
    backend: Backend,
    directivity: Directivity,
    config: ConfigPtr,
    geometry: GeometryPtr,
    idx: u32,
) -> ResultI32 {
    let idx = idx as usize;
    into_result!(match_visualizer_plot!(
        backend,
        directivity,
        visualizer,
        plot_phase_of,
        config,
        &geometry,
        idx
    ))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotModulation(
    visualizer: LinkPtr,
    backend: Backend,
    directivity: Directivity,
    config: ConfigPtr,
) -> ResultI32 {
    into_result!(match_visualizer_plot!(
        backend,
        directivity,
        visualizer,
        plot_modulation,
        config,
    ))
}

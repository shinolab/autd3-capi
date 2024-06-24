#![allow(clippy::missing_safety_doc)]

pub mod null;
pub mod plotters;
pub mod python;

use autd3_link_visualizer::{
    NullBackend, NullPlotConfig, PlotConfig, PlotRange, PlottersBackend, PyPlotConfig,
    PythonBackend, Visualizer,
};
use autd3capi_driver::{
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
pub struct ConfigPtr(pub *const libc::c_void);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PlotRangePtr(pub *const libc::c_void);

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
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
    z_min: f32,
    z_max: f32,
    resolution: f32,
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
    let n = |range: &std::ops::Range<f32>, resolution: f32| -> usize {
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
    points: *mut f32,
) {
    let range = take!(range, PlotRange);
    let observe_points = range.observe_points();
    std::ptr::copy_nonoverlapping(
        observe_points.as_ptr() as *const f32,
        points,
        observe_points.len() * 3,
    );
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkVisualizerPhasesOf(
    visualizer: LinkPtr,
    backend: Backend,
    directivity: Directivity,
    segment: Segment,
    idx: u16,
    buf: *mut u8,
) -> u32 {
    let idx = idx as usize;
    let segment = segment.into();
    let m = match_visualizer!(backend, directivity, visualizer, phases, segment, idx);
    if !buf.is_null() {
        std::ptr::copy_nonoverlapping(m.as_ptr() as _, buf, m.len());
    }
    m.len() as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkVisualizerIntensities(
    visualizer: LinkPtr,
    backend: Backend,
    directivity: Directivity,
    segment: Segment,
    idx: u16,
    buf: *mut u8,
) -> u32 {
    let idx = idx as usize;
    let segment = segment.into();
    let m = match_visualizer!(backend, directivity, visualizer, intensities, segment, idx);
    if !buf.is_null() {
        std::ptr::copy_nonoverlapping(m.as_ptr() as _, buf, m.len());
    }
    m.len() as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkVisualizerModulation(
    visualizer: LinkPtr,
    backend: Backend,
    directivity: Directivity,
    segment: Segment,
    buf: *mut u8,
) -> u32 {
    let segment = segment.into();
    let m = match_visualizer!(backend, directivity, visualizer, modulation, segment);
    if !buf.is_null() {
        std::ptr::copy_nonoverlapping(m.as_ptr() as _, buf, m.len());
    }
    m.len() as _
}

macro_rules! into_result {
    ($r:expr) => {
        match $r {
            Ok(_) => ResultI32 {
                result: AUTD3_TRUE,
                err_len: 0,
                err: ConstPtr(std::ptr::null_mut()),
            },
            Err(e) => {
                let err = e.to_string();
                ResultI32 {
                    result: 0,
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: ConstPtr(Box::into_raw(Box::new(err)) as _),
                }
            }
        }
    };
}

#[no_mangle]
#[allow(clippy::blocks_in_conditions)]
pub unsafe extern "C" fn AUTDLinkVisualizerCalcField(
    visualizer: LinkPtr,
    backend: Backend,
    directivity: Directivity,
    points: *const Vector3,
    points_len: u32,
    geometry: GeometryPtr,
    segment: Segment,
    idx: u16,
    buf: *mut f32,
) -> ResultI32 {
    let idx = idx as usize;
    let segment = segment.into();
    let len = points_len as usize;
    let points = std::slice::from_raw_parts(points as *const Vector3, len);
    into_result!(match_visualizer!(
        backend,
        directivity,
        visualizer,
        calc_field,
        points.iter(),
        &geometry,
        segment,
        idx
    )
    .and_then(|m| {
        std::ptr::copy_nonoverlapping(m.as_ptr(), buf as *mut _, m.len());
        Ok(())
    }))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotField(
    visualizer: LinkPtr,
    backend: Backend,
    directivity: Directivity,
    config: ConfigPtr,
    range: PlotRangePtr,
    geometry: GeometryPtr,
    segment: Segment,
    idx: u16,
) -> ResultI32 {
    let idx = idx as usize;
    let segment = segment.into();
    into_result!(match_visualizer_plot!(
        backend,
        directivity,
        visualizer,
        plot_field,
        config,
        *take!(range, PlotRange),
        &geometry,
        segment,
        idx
    ))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotPhase(
    visualizer: LinkPtr,
    backend: Backend,
    directivity: Directivity,
    config: ConfigPtr,
    geometry: GeometryPtr,
    segment: Segment,
    idx: u16,
) -> ResultI32 {
    let idx = idx as usize;
    let segment = segment.into();
    into_result!(match_visualizer_plot!(
        backend,
        directivity,
        visualizer,
        plot_phase,
        config,
        &geometry,
        segment,
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
    segment: Segment,
) -> ResultI32 {
    let segment = segment.into();
    into_result!(match_visualizer_plot!(
        backend,
        directivity,
        visualizer,
        plot_modulation,
        config,
        segment
    ))
}

use autd3_link_visualizer::{NullBackend, NullPlotConfig, Visualizer};
use autd3capi_driver::{
    driver::acoustics::directivity::{Sphere, T4010A1},
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerSphereNull(
    use_gpu: bool,
    gpu_idx: i32,
) -> LinkBuilderPtr {
    let mut builder = Visualizer::builder()
        .with_directivity::<Sphere>()
        .with_backend::<NullBackend>();
    if use_gpu {
        builder = builder.with_gpu(gpu_idx);
    }
    DynamicLinkBuilder::new(builder)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerT4010A1Null(
    use_gpu: bool,
    gpu_idx: i32,
) -> LinkBuilderPtr {
    let mut builder = Visualizer::builder()
        .with_directivity::<T4010A1>()
        .with_backend::<NullBackend>();
    if use_gpu {
        builder = builder.with_gpu(gpu_idx);
    }
    DynamicLinkBuilder::new(builder)
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct NullPlotConfigPtr(pub ConstPtr);

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerNullPlotConfig() -> NullPlotConfigPtr {
    NullPlotConfigPtr(Box::into_raw(Box::new(NullPlotConfig {})) as _)
}

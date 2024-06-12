use std::time::Duration;

use autd3capi_driver::{
    async_ffi::{FfiFuture, FutureExt},
    autd3::{controller::ControllerBuilder, Controller},
    driver::{
        autd3_device::AUTD3,
        defined::Hz,
        geometry::{Quaternion, UnitQuaternion, Vector3},
    },
    take, vec_from_raw, ConstPtr, DynamicLinkBuilder, LinkBuilderPtr,
};

use super::{ControllerWrap, ResultController};

#[repr(C)]
pub struct ControllerBuilderPtr(pub ConstPtr);

impl ControllerBuilderPtr {
    pub fn new(builder: ControllerBuilder) -> Self {
        Self(Box::into_raw(Box::new(builder)) as _)
    }
}

#[no_mangle]
#[must_use]
#[allow(clippy::box_default)]
pub unsafe extern "C" fn AUTDControllerBuilder(
    pos: *const Vector3,
    rot: *const Quaternion,
    len: u16,
) -> ControllerBuilderPtr {
    let pos = vec_from_raw!(pos, Vector3, len);
    let rot = vec_from_raw!(rot, Quaternion, len);
    ControllerBuilderPtr::new(Controller::builder(
        pos.into_iter()
            .zip(rot)
            .map(|(p, r)| AUTD3::new(p).with_rotation(UnitQuaternion::from_quaternion(r))),
    ))
}

#[no_mangle]
#[must_use]
#[allow(clippy::box_default)]
pub unsafe extern "C" fn AUTDControllerBuilderWithUltrasoundFreq(
    builder: ControllerBuilderPtr,
    ultrasound_freq: u32,
) -> ControllerBuilderPtr {
    ControllerBuilderPtr::new(
        take!(builder, ControllerBuilder).with_ultrasound_freq(ultrasound_freq * Hz),
    )
}

#[no_mangle]
#[must_use]
#[allow(clippy::box_default)]
pub unsafe extern "C" fn AUTDControllerBuilderWithParallelThreshold(
    builder: ControllerBuilderPtr,
    parallel_threshold: u16,
) -> ControllerBuilderPtr {
    ControllerBuilderPtr::new(
        take!(builder, ControllerBuilder).with_parallel_threshold(parallel_threshold as _),
    )
}

#[no_mangle]
#[must_use]
#[allow(clippy::box_default)]
pub unsafe extern "C" fn AUTDControllerBuilderWithSendInterval(
    builder: ControllerBuilderPtr,
    interval_ns: u64,
) -> ControllerBuilderPtr {
    ControllerBuilderPtr::new(
        take!(builder, ControllerBuilder)
            .with_send_interval(std::time::Duration::from_nanos(interval_ns)),
    )
}

#[no_mangle]
#[must_use]
#[allow(clippy::box_default)]
pub unsafe extern "C" fn AUTDControllerBuilderWithTimerResolution(
    builder: ControllerBuilderPtr,
    resolution: u32,
) -> ControllerBuilderPtr {
    #[cfg(target_os = "windows")]
    {
        ControllerBuilderPtr::new(
            take!(builder, ControllerBuilder).with_timer_resolution(resolution),
        )
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = resolution;
        builder
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerOpen(
    builder: ControllerBuilderPtr,
    link_builder: LinkBuilderPtr,
    timeout_ns: i64,
) -> FfiFuture<ResultController> {
    let builder = take!(builder, ControllerBuilder);
    let parallel_threshold = builder.parallel_threshold();
    let link_builder = take!(link_builder, DynamicLinkBuilder);
    async move {
        match timeout_ns {
            v if v < 0 => builder.open(*link_builder).await,
            _ => {
                builder
                    .open_with_timeout(*link_builder, Duration::from_nanos(timeout_ns as _))
                    .await
            }
        }
        .map(|c| ControllerWrap {
            inner: c,
            parallel_threshold,
            last_parallel_threshold: parallel_threshold,
        })
        .into()
    }
    .into_ffi()
}

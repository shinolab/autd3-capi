use std::time::Duration;

use autd3capi_driver::{
    autd3::{controller::ControllerBuilder, error::AUTDError, Controller},
    driver::{
        autd3_device::AUTD3,
        defined::{Freq, Hz},
        geometry::{IntoDevice, Quaternion, UnitQuaternion, Vector3},
    },
    take, vec_from_raw, ConstPtr, LinkBuilderPtr, SyncLinkBuilder,
};

use super::{ResultController, SyncController};

pub struct SyncControllerBuilder {
    inner: ControllerBuilder,
    parallel_threshold: usize,
}

impl SyncControllerBuilder {
    pub fn new<D: IntoDevice, I: IntoIterator<Item = D>>(iter: I) -> Self {
        Self {
            inner: Controller::builder(iter),
            parallel_threshold: 4,
        }
    }

    pub fn with_ultrasound_freq(self, ultrasound_freq: Freq<u32>) -> Self {
        Self {
            inner: self.inner.with_ultrasound_freq(ultrasound_freq),
            ..self
        }
    }

    pub fn with_parallel_threshold(self, parallel_threshold: usize) -> Self {
        Self {
            inner: self.inner.with_parallel_threshold(parallel_threshold),
            parallel_threshold,
        }
    }

    pub fn with_send_interval(self, interval: std::time::Duration) -> Self {
        Self {
            inner: self.inner.with_send_interval(interval),
            ..self
        }
    }

    pub fn with_timer_resolution(self, resolution: u32) -> Self {
        #[cfg(target_os = "windows")]
        {
            Self {
                inner: self.inner.with_timer_resolution(resolution),
                ..self
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = resolution;
            self
        }
    }

    pub fn open(self, link_builder: SyncLinkBuilder) -> Result<SyncController, AUTDError> {
        Self::open_with_timeout(self, link_builder, std::time::Duration::from_millis(200))
    }

    pub fn open_with_timeout(
        self,
        mut link_builder: SyncLinkBuilder,
        timeout: std::time::Duration,
    ) -> Result<SyncController, AUTDError> {
        let runtime = link_builder.runtime.take().unwrap();
        Ok(SyncController {
            inner: runtime.block_on(self.inner.open_with_timeout(link_builder, timeout))?,
            parallel_threshold: self.parallel_threshold,
            last_parallel_threshold: self.parallel_threshold,
            runtime,
        })
    }
}

#[repr(C)]
pub struct ControllerBuilderPtr(pub ConstPtr);

impl ControllerBuilderPtr {
    pub fn new(builder: SyncControllerBuilder) -> Self {
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
    ControllerBuilderPtr::new(SyncControllerBuilder::new(
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
        take!(builder, SyncControllerBuilder).with_ultrasound_freq(ultrasound_freq * Hz),
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
        take!(builder, SyncControllerBuilder).with_parallel_threshold(parallel_threshold as _),
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
        take!(builder, SyncControllerBuilder)
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
    ControllerBuilderPtr::new(
        take!(builder, SyncControllerBuilder).with_timer_resolution(resolution),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerOpen(
    builder: ControllerBuilderPtr,
    link_builder: LinkBuilderPtr,
    timeout_ns: i64,
) -> ResultController {
    let builder = take!(builder, SyncControllerBuilder);
    let link_builder = take!(link_builder, SyncLinkBuilder);
    match timeout_ns {
        v if v < 0 => builder.open(*link_builder),
        _ => builder.open_with_timeout(*link_builder, Duration::from_nanos(timeout_ns as _)),
    }
    .into()
}

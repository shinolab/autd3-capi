use std::time::Duration;

use autd3capi_driver::{
    autd3::{controller::ControllerBuilder, error::AUTDError, Controller},
    driver::{
        autd3_device::AUTD3,
        defined::{Freq, Hz},
        geometry::{IntoDevice, Quaternion, UnitQuaternion, Vector3},
    },
    take, ConstPtr, LinkBuilderPtr, SyncLinkBuilder,
};

use super::{ResultController, SyncController};

pub struct SyncControllerBuilder {
    inner: ControllerBuilder,
}

impl SyncControllerBuilder {
    pub const fn new(ultrasound_freq: Freq<u32>) -> Self {
        Self {
            inner: Controller::builder_with_ultrasound_freq(ultrasound_freq),
        }
    }

    pub fn add_device<D: IntoDevice>(self, dev: D) -> Self {
        Self {
            inner: self.inner.add_device(dev),
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
            runtime,
        })
    }
}

#[derive(Debug, Clone, Copy)]
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
pub unsafe extern "C" fn AUTDControllerBuilder(ultrasound_freq: u32) -> ControllerBuilderPtr {
    ControllerBuilderPtr::new(SyncControllerBuilder::new(ultrasound_freq * Hz))
}

#[no_mangle]
pub unsafe extern "C" fn AUTDControllerBuilderAddDevice(
    builder: ControllerBuilderPtr,
    x: f64,
    y: f64,
    z: f64,
    qw: f64,
    qx: f64,
    qy: f64,
    qz: f64,
) -> ControllerBuilderPtr {
    ControllerBuilderPtr::new(take!(builder, SyncControllerBuilder).add_device(
        AUTD3::new(Vector3::new(x, y, z)).with_rotation(UnitQuaternion::from_quaternion(
            Quaternion::new(qw, qx, qy, qz),
        )),
    ))
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

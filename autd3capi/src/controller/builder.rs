use std::time::Duration;

use autd3capi_driver::{
    async_ffi::{FfiFuture, FutureExt},
    autd3::{
        controller::{timer::TimerStrategy, ControllerBuilder},
        derive::Device,
        link::Nop,
        Controller,
    },
    driver::{
        autd3_device::AUTD3,
        geometry::{Quaternion, UnitQuaternion, Vector3},
    },
    take, vec_from_raw, DynamicLinkBuilder, LinkBuilderPtr, TimerStrategyWrap,
};

use super::ResultController;

#[repr(C)]
pub struct ControllerBuilderPtr(pub *const libc::c_void);

impl ControllerBuilderPtr {
    pub fn new(builder: ControllerBuilder) -> Self {
        Self(Box::into_raw(Box::new(builder)) as _)
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerBuilder(
    pos: *const Vector3,
    rot: *const Quaternion,
    len: u16,
    fallback_parallel_threshold: u16,
    fallback_timeout: u64,
    send_interval_ns: u64,
    receive_interval_ns: u64,
    timer_strategy: TimerStrategyWrap,
) -> ControllerBuilderPtr {
    let pos = vec_from_raw!(pos, Vector3, len);
    let rot = vec_from_raw!(rot, Quaternion, len);
    ControllerBuilderPtr::new(
        Controller::builder(
            pos.into_iter()
                .zip(rot)
                .map(|(p, r)| AUTD3::new(p).with_rotation(UnitQuaternion::from_quaternion(r))),
        )
        .with_fallback_parallel_threshold(fallback_parallel_threshold as _)
        .with_fallback_timeout(Duration::from_nanos(fallback_timeout))
        .with_send_interval(Duration::from_nanos(send_interval_ns))
        .with_receive_interval(Duration::from_nanos(receive_interval_ns))
        .with_timer_strategy(timer_strategy.into()),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerBuilderIsDefault(
    fallback_parallel_threshold: u16,
    fallback_timeout: u64,
    send_interval_ns: u64,
    receive_interval_ns: u64,
    timer_strategy: TimerStrategyWrap,
) -> bool {
    let default = Controller::<Nop>::builder::<Device, _>([]);
    fallback_parallel_threshold as usize == default.fallback_parallel_threshold()
        && fallback_timeout as u128 == default.fallback_timeout().as_nanos()
        && send_interval_ns as u128 == default.send_interval().as_nanos()
        && receive_interval_ns as u128 == default.receive_interval().as_nanos()
        && &TimerStrategy::from(timer_strategy) == default.timer_strategy()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerOpen(
    builder: ControllerBuilderPtr,
    link_builder: LinkBuilderPtr,
    timeout_ns: i64,
) -> FfiFuture<ResultController> {
    let builder = take!(builder, ControllerBuilder);
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
        .into()
    }
    .into_ffi()
}

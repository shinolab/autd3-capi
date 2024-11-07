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
    take, vec_from_raw, ControllerBuilderPtr, Duration, DynamicLinkBuilder, LinkBuilderPtr,
    OptionDuration, TimerStrategyWrap,
};

use super::ResultController;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerBuilder(
    pos: *const Vector3,
    rot: *const Quaternion,
    len: u16,
    fallback_parallel_threshold: u16,
    fallback_timeout: Duration,
    send_interval: Duration,
    receive_interval: Duration,
    timer_strategy: TimerStrategyWrap,
) -> ControllerBuilderPtr {
    let pos = vec_from_raw!(pos, Vector3, len);
    let rot = vec_from_raw!(rot, Quaternion, len);
    Controller::builder(
        pos.into_iter()
            .zip(rot)
            .map(|(p, r)| AUTD3::new(p).with_rotation(UnitQuaternion::from_quaternion(r))),
    )
    .with_fallback_parallel_threshold(fallback_parallel_threshold as _)
    .with_fallback_timeout(fallback_timeout.into())
    .with_send_interval(send_interval.into())
    .with_receive_interval(receive_interval.into())
    .with_timer_strategy(timer_strategy.into())
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerBuilderIsDefault(
    fallback_parallel_threshold: u16,
    fallback_timeout: Duration,
    send_interval_ns: Duration,
    receive_interval: Duration,
    timer_strategy: TimerStrategyWrap,
) -> bool {
    let default = Controller::<Nop>::builder::<Device, _>([]);
    fallback_parallel_threshold as usize == default.fallback_parallel_threshold()
        && std::time::Duration::from(fallback_timeout) == default.fallback_timeout()
        && std::time::Duration::from(send_interval_ns) == default.send_interval()
        && std::time::Duration::from(receive_interval) == default.receive_interval()
        && &TimerStrategy::from(timer_strategy) == default.timer_strategy()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerOpen(
    builder: ControllerBuilderPtr,
    link_builder: LinkBuilderPtr,
    timeout: OptionDuration,
) -> FfiFuture<ResultController> {
    let builder = take!(builder, ControllerBuilder);
    let link_builder = take!(link_builder, DynamicLinkBuilder);
    async move {
        match timeout.into() {
            None => builder.open(*link_builder).await,
            Some(timeout) => builder.open_with_timeout(*link_builder, timeout).await,
        }
        .into()
    }
    .into_ffi()
}

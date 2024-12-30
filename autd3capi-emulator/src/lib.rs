#![allow(clippy::missing_safety_doc)]

mod drive;
mod instant;
mod output;
mod ptr;
mod range;
mod result;
mod rms;

use std::ffi::c_char;

use ptr::*;
use result::*;

use autd3::controller::ControllerBuilder;
use autd3_emulator::{ControllerBuilderIntoEmulatorExt, Emulator, Record, Recorder};
use autd3capi_driver::{async_ffi::*, autd3::prelude::*, *};
use driver::link::Link;

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorTracingInit() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorTracingInitWithFile(path: *const c_char) -> ResultStatus {
    let path = validate_cstr!(path, AUTDStatus, ResultStatus);
    std::fs::File::options()
        .append(true)
        .create(true)
        .open(path)
        .map(|f| {
            tracing_subscriber::fmt()
                .with_writer(f)
                .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
                .with_ansi(false)
                .init();
            AUTDStatus::AUTDTrue
        })
        .into()
}

#[no_mangle]
#[must_use]
#[allow(clippy::box_default)]
pub unsafe extern "C" fn AUTDEmulator(builder: ControllerBuilderPtr) -> EmulatorPtr {
    take!(builder, ControllerBuilder).into_emulator().into()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorFree(emulator: EmulatorPtr) {
    let _ = take!(emulator, Emulator);
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorGeometry(emulator: EmulatorPtr) -> GeometryPtr {
    GeometryPtr(emulator.geometry() as *const _ as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorRecordFrom(
    emulator: EmulatorPtr,
    start_time: DcSysTime,
    f: ConstPtr,
) -> FfiFuture<ResultRecord> {
    async move {
        emulator
            .record_from(start_time, move |cnt| async move {
                let f = std::mem::transmute::<ConstPtr, unsafe extern "C" fn(ControllerPtr)>(f);
                let cnt = cnt.into_boxed_link();
                let cnt_ptr = ControllerPtr(Box::into_raw(Box::new(cnt)) as _);
                tokio::task::block_in_place(|| f(cnt_ptr));
                let cnt = Controller::from_boxed_link(*Box::from_raw(
                    cnt_ptr.0 as *mut Controller<Box<dyn Link>>,
                ));
                Ok(cnt)
            })
            .await
            .into()
    }
    .into_ffi()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorRecordFree(record: RecordPtr) {
    let _ = take!(record, Record);
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorWaitResultRecord(
    handle: HandlePtr,
    future: FfiFuture<ResultRecord>,
) -> ResultRecord {
    handle.block_on(future)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorTickNs(mut record: LinkPtr, tick: Duration) -> ResultStatus {
    record.cast_mut::<Recorder>().tick(tick.into()).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorTransducerTableRows(emulator: EmulatorPtr) -> u64 {
    emulator.transducer_table_rows() as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorTransducerTable(
    emulator: EmulatorPtr,
    dev_indices: *mut u16,
    tr_indices: *mut u8,
    x: *mut f32,
    y: *mut f32,
    z: *mut f32,
    nx: *mut f32,
    ny: *mut f32,
    nz: *mut f32,
) {
    let n = emulator.transducer_table_rows();
    emulator.dev_indices_inplace(std::slice::from_raw_parts_mut(dev_indices, n));
    emulator.tr_indices_inplace(std::slice::from_raw_parts_mut(tr_indices, n));
    emulator.tr_positions_inplace(
        std::slice::from_raw_parts_mut(x, n),
        std::slice::from_raw_parts_mut(y, n),
        std::slice::from_raw_parts_mut(z, n),
    );
    emulator.tr_dir_inplace(
        std::slice::from_raw_parts_mut(nx, n),
        std::slice::from_raw_parts_mut(ny, n),
        std::slice::from_raw_parts_mut(nz, n),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use autd3capi::*;

    pub unsafe fn create_emulator() -> EmulatorPtr {
        let pos = [Point3::new(0.0, 0.0, 0.0); 1];
        let rot = [Quaternion::new(1.0, 0.0, 0.0, 0.0); 1];
        let builder = controller::builder::AUTDControllerBuilder(
            pos.as_ptr(),
            rot.as_ptr(),
            1,
            4,
            std::time::Duration::from_millis(20).into(),
            std::time::Duration::from_millis(1).into(),
            std::time::Duration::from_millis(1).into(),
            controller::timer::AUTDTimerStrategySpin(
                controller::timer::AUTDTimerStrategySpinDefaultAccuracy(),
                autd3capi_driver::SpinStrategyTag::SpinLoopHint,
            ),
        );
        AUTDEmulator(builder)
    }

    #[test]
    fn transducer_table() {
        unsafe {
            let emulator = create_emulator();

            let rows = AUTDEmulatorTransducerTableRows(emulator);
            assert_eq!(249, rows);
            let mut dev_indices = vec![0; rows as _];
            let mut tr_indices = vec![0; rows as _];
            let mut x = vec![0.0; rows as _];
            let mut y = vec![0.0; rows as _];
            let mut z = vec![0.0; rows as _];
            let mut nx = vec![0.0; rows as _];
            let mut ny = vec![0.0; rows as _];
            let mut nz = vec![0.0; rows as _];
            AUTDEmulatorTransducerTable(
                emulator,
                dev_indices.as_mut_ptr(),
                tr_indices.as_mut_ptr(),
                x.as_mut_ptr(),
                y.as_mut_ptr(),
                z.as_mut_ptr(),
                nx.as_mut_ptr(),
                ny.as_mut_ptr(),
                nz.as_mut_ptr(),
            );
            assert_eq!(vec![0; 249], dev_indices);
            assert_eq!((0..249).collect::<Vec<_>>(), tr_indices);
            assert_eq!(vec![0.; 249], z);
            assert_eq!(vec![0.; 249], nx);
            assert_eq!(vec![0.; 249], ny);
            assert_eq!(vec![1.; 249], nz);
        }
    }
}

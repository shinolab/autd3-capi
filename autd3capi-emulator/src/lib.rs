#![allow(clippy::missing_safety_doc)]

mod instant;
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
use driver::{ethercat::ECAT_DC_SYS_TIME_BASE, link::Link};

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
    start_time_ns: u64,
    f: ConstPtr,
) -> FfiFuture<ResultRecord> {
    async move {
        emulator
            .record_from(
                DcSysTime::from_utc(
                    ECAT_DC_SYS_TIME_BASE + std::time::Duration::from_nanos(start_time_ns),
                )
                .unwrap(),
                move |cnt| async move {
                    let f = std::mem::transmute::<ConstPtr, unsafe extern "C" fn(ControllerPtr)>(f);
                    let cnt = cnt.into_boxed_link();
                    let cnt_ptr = ControllerPtr(Box::into_raw(Box::new(cnt)) as _);
                    tokio::task::block_in_place(|| f(cnt_ptr));
                    let cnt = Controller::from_boxed_link(*Box::from_raw(
                        cnt_ptr.0 as *mut Controller<Box<dyn Link>>,
                    ));
                    Ok(cnt)
                },
            )
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
pub unsafe extern "C" fn AUTDEmulatorRecordNumDevices(record: RecordPtr) -> u16 {
    record.num_devices() as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorRecordNumTransducers(record: RecordPtr, dev_idx: u16) -> u8 {
    record.num_transducers(dev_idx as usize) as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorRecordDriveLen(record: RecordPtr) -> u64 {
    record.drive_time_len() as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorRecordDriveTime(record: RecordPtr, time: *mut u64) {
    record.drive_time_inplace(std::slice::from_raw_parts_mut(
        time,
        record.drive_time_len(),
    ));
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorRecordDrivePulseWidth(
    record: RecordPtr,
    dev_idx: u16,
    tr_idx: u8,
    pulsewidth: *mut u8,
) {
    record.drive_pulsewidth_inplace(
        dev_idx as usize,
        tr_idx as usize,
        std::slice::from_raw_parts_mut(pulsewidth, record.drive_time_len()),
    );
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorRecordDrivePhase(
    record: RecordPtr,
    dev_idx: u16,
    tr_idx: u8,
    pulsewidth: *mut u8,
) {
    record.drive_phase_inplace(
        dev_idx as usize,
        tr_idx as usize,
        std::slice::from_raw_parts_mut(pulsewidth, AUTDEmulatorRecordDriveLen(record) as _),
    );
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorRecordOutputLen(record: RecordPtr) -> u64 {
    record.output_len() as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorRecordOutputTime(record: RecordPtr, time: *mut u64) {
    record.output_voltage_time_inplace(std::slice::from_raw_parts_mut(time, record.output_len()));
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorRecordOutputVoltage(
    record: RecordPtr,
    dev_idx: u16,
    tr_idx: u8,
    v: *mut f32,
) {
    record.output_voltage_inplace(
        dev_idx as usize,
        tr_idx as usize,
        std::slice::from_raw_parts_mut(v, record.output_len()),
    );
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorRecordOutputUltrasound(
    record: RecordPtr,
    dev_idx: u16,
    tr_idx: u8,
    v: *mut f32,
) {
    record.output_ultrasound_inplace(
        dev_idx as usize,
        tr_idx as usize,
        std::slice::from_raw_parts_mut(v, record.output_len()),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use autd3capi::*;
    use link::AUTDLinkGet;
    use tokio::runtime::Handle;

    #[test]
    fn record_drive() {
        unsafe extern "C" fn f(cnt: ControllerPtr) {
            let handle = Handle::current();

            let handle = HandlePtr(&raw const handle as _);

            let link_ptr = AUTDLinkGet(cnt);

            let g = gain::uniform::AUTDGainUniform(0xFF, 0x40);
            let d = gain::AUTDGainIntoDatagram(g);

            let future = controller::AUTDControllerSend(cnt, d);
            let result = AUTDWaitResultStatus(handle, future);
            assert_eq!(AUTDStatus::AUTDTrue, result.result);

            let result = AUTDEmulatorTickNs(link_ptr, (10 * ULTRASOUND_PERIOD).into());
            assert_eq!(AUTDStatus::AUTDTrue, result.result);
        }

        unsafe {
            let runtime = AUTDCreateRuntime();
            let handle = AUTDGetRuntimeHandle(runtime);

            let pos = [Vector3::new(0.0, 0.0, 0.0); 1];
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
            let emulator = AUTDEmulator(builder);

            let record = AUTDEmulatorRecordFrom(
                emulator,
                0,
                std::mem::transmute::<unsafe extern "C" fn(ControllerPtr), ConstPtr>(f),
            );
            let record = AUTDEmulatorWaitResultRecord(handle, record);
            assert!(!record.result.0.is_null());
            let record = record.result;

            let len = AUTDEmulatorRecordDriveLen(record);
            let mut time = vec![0; len as _];
            let mut pulsewidth = vec![0; len as _];
            let mut phase = vec![0; len as _];
            AUTDEmulatorRecordDriveTime(record, time.as_mut_ptr());
            AUTDEmulatorRecordDrivePulseWidth(record, 0, 0, pulsewidth.as_mut_ptr());
            AUTDEmulatorRecordDrivePhase(record, 0, 0, phase.as_mut_ptr());

            assert_eq!(
                vec![0, 25000, 50000, 75000, 100000, 125000, 150000, 175000, 200000, 225000],
                time
            );
            assert_eq!(vec![8, 16, 25, 34, 42, 52, 63, 76, 91, 128,], pulsewidth);
            assert_eq!(vec![1, 3, 4, 6, 8, 9, 11, 12, 14, 16,], phase);

            AUTDEmulatorRecordFree(record);

            AUTDEmulatorFree(emulator);

            AUTDDeleteRuntime(runtime);
        }
    }

    #[test]
    fn record_output_voltage() {
        unsafe extern "C" fn f(cnt: ControllerPtr) {
            let handle = Handle::current();

            let handle = HandlePtr(&raw const handle as _);

            let link_ptr = AUTDLinkGet(cnt);

            let g = gain::uniform::AUTDGainUniform(0xFF, 0x40);
            let d = gain::AUTDGainIntoDatagram(g);

            let future = controller::AUTDControllerSend(cnt, d);
            let result = AUTDWaitResultStatus(handle, future);
            assert_eq!(AUTDStatus::AUTDTrue, result.result);

            let result = AUTDEmulatorTickNs(link_ptr, ULTRASOUND_PERIOD.into());
            assert_eq!(AUTDStatus::AUTDTrue, result.result);
        }

        unsafe {
            let runtime = AUTDCreateRuntime();
            let handle = AUTDGetRuntimeHandle(runtime);

            let pos = [Vector3::new(0.0, 0.0, 0.0); 1];
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
            let emulator = AUTDEmulator(builder);

            let record = AUTDEmulatorRecordFrom(
                emulator,
                0,
                std::mem::transmute::<unsafe extern "C" fn(ControllerPtr), ConstPtr>(f),
            );
            let record = AUTDEmulatorWaitResultRecord(handle, record);
            assert!(!record.result.0.is_null());
            let record = record.result;

            let len = AUTDEmulatorRecordOutputLen(record);
            let mut time = vec![0; len as _];
            let mut v = vec![0.; len as _];
            AUTDEmulatorRecordOutputTime(record, time.as_mut_ptr());
            AUTDEmulatorRecordOutputVoltage(record, 0, 0, v.as_mut_ptr());

            assert_eq!((0..len).collect::<Vec<_>>(), time);
            assert_eq!(
                vec![
                    12.0, 12.0, 12.0, 12.0, 12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                    -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                    -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                    -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                    -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                    -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                    -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                    -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                    -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                    -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                    -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                    -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                    -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                    -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                    -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                    -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                    -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                    -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                    -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                    -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                    -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                    -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                    -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, 12.0,
                    12.0, 12.0,
                ],
                v
            );

            AUTDEmulatorRecordFree(record);

            AUTDEmulatorFree(emulator);

            AUTDDeleteRuntime(runtime);
        }
    }
}

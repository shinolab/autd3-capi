#![allow(clippy::missing_safety_doc)]

mod option;
mod ptr;
mod range;
mod result;

use option::*;
use ptr::*;
use range::*;
use result::*;

use autd3::controller::ControllerBuilder;
use autd3_emulator::{ControllerBuilderIntoEmulatorExt, Emulator, Record, Recorder, SoundField};
use autd3capi_driver::{async_ffi::*, autd3::prelude::*, *};
use driver::{ethercat::ECAT_DC_SYS_TIME_BASE, link::Link};

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
    start_time: u64,
    f: ConstPtr,
) -> FfiFuture<ResultRecord> {
    async move {
        emulator
            .record_from(
                DcSysTime::from_utc(
                    ECAT_DC_SYS_TIME_BASE + std::time::Duration::from_nanos(start_time),
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
pub unsafe extern "C" fn AUTDEmulatorTickNs(mut record: LinkPtr, tick_ns: u64) -> ResultStatus {
    record
        .cast_mut::<Recorder>()
        .tick(std::time::Duration::from_nanos(tick_ns))
        .into()
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

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorSoundField(
    record: RecordPtr,
    range: Range,
    option: RecordOption,
) -> LocalFfiFuture<ResultSoundField> {
    async move {
        let r = record
            .static_deref()
            .sound_field(range.into(), option.into())
            .await;
        r.into()
    }
    .into_local_ffi()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorWaitSoundField(
    handle: HandlePtr,
    future: LocalFfiFuture<ResultSoundField>,
) -> ResultSoundField {
    handle.block_on(future)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldTimeLen(
    sound_field: SoundFieldPtr,
    duration_ns: u64,
) -> u64 {
    sound_field.next_time_len(std::time::Duration::from_nanos(duration_ns)) as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldPointsLen(sound_field: SoundFieldPtr) -> u64 {
    sound_field.next_points_len() as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldGetX(sound_field: SoundFieldPtr, x: *mut f32) {
    sound_field.x_inplace(std::slice::from_raw_parts_mut(
        x,
        sound_field.next_points_len(),
    ));
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldGetY(sound_field: SoundFieldPtr, y: *mut f32) {
    sound_field.y_inplace(std::slice::from_raw_parts_mut(
        y,
        sound_field.next_points_len(),
    ));
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldGetZ(sound_field: SoundFieldPtr, z: *mut f32) {
    sound_field.z_inplace(std::slice::from_raw_parts_mut(
        z,
        sound_field.next_points_len(),
    ));
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldSkip(
    mut sound_field: SoundFieldPtr,
    duration_ns: u64,
) -> LocalFfiFuture<ResultStatus> {
    async move {
        sound_field
            .next_inplace(
                std::time::Duration::from_nanos(duration_ns),
                true,
                &mut [],
                std::iter::empty(),
            )
            .await
            .into()
    }
    .into_local_ffi()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldNext(
    mut sound_field: SoundFieldPtr,
    duration_ns: u64,
    time: *mut u64,
    v: *const *mut f32,
) -> LocalFfiFuture<ResultStatus> {
    let n = sound_field.next_time_len(std::time::Duration::from_nanos(duration_ns));
    let time = std::slice::from_raw_parts_mut(time, n as _);
    let iter = (0..n).map(move |i| v.add(i as _).read());
    async move {
        sound_field
            .next_inplace(
                std::time::Duration::from_nanos(duration_ns),
                false,
                time,
                iter,
            )
            .await
            .into()
    }
    .into_local_ffi()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldFree(sound_field: SoundFieldPtr) {
    let _ = take!(sound_field, SoundField<'static>);
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

            let result = AUTDEmulatorTickNs(link_ptr, 10 * ULTRASOUND_PERIOD.as_nanos() as u64);
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
                20_000_000,
                1_000_000,
                1_000_000,
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

            let result = AUTDEmulatorTickNs(link_ptr, ULTRASOUND_PERIOD.as_nanos() as u64);
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
                20_000_000,
                1_000_000,
                1_000_000,
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

    #[test]
    fn record_sound_field() {
        unsafe extern "C" fn f(cnt: ControllerPtr) {
            let handle = Handle::current();

            let handle = HandlePtr(&raw const handle as _);

            let link_ptr = AUTDLinkGet(cnt);

            let g = gain::uniform::AUTDGainUniform(0xFF, 0x40);
            let d = gain::AUTDGainIntoDatagram(g);

            let future = controller::AUTDControllerSend(cnt, d);
            let result = AUTDWaitResultStatus(handle, future);
            assert_eq!(AUTDStatus::AUTDTrue, result.result);

            let result = AUTDEmulatorTickNs(link_ptr, 10 * ULTRASOUND_PERIOD.as_nanos() as u64);
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
                20_000_000,
                1_000_000,
                1_000_000,
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

            let range = Range {
                x_start: -1.,
                x_end: 1.,
                y_start: 0.,
                y_end: 0.,
                z_start: 10.,
                z_end: 10.,
                resolution: 1.,
            };
            let option = RecordOption {
                sound_speed: 340e3,
                time_step_ns: 1000,
                print_progress: true,
                memory_limits_hint_mb: 128,
                gpu: false,
            };
            let sound_filed = AUTDEmulatorSoundField(record, range, option);
            let sound_field = AUTDEmulatorWaitSoundField(handle, sound_filed);
            assert!(!sound_field.result.0.is_null());
            let sound_field = sound_field.result;

            {
                let duration_ns = 9 * ULTRASOUND_PERIOD.as_nanos() as u64;
                let res = AUTDEmulatorSoundFieldSkip(sound_field, duration_ns);
                let res = AUTDWaitLocalResultStatus(handle, res);
                assert_eq!(AUTDStatus::AUTDTrue, res.result);
            }

            {
                let duration_ns = ULTRASOUND_PERIOD.as_nanos() as u64;
                let len = AUTDEmulatorSoundFieldTimeLen(sound_field, duration_ns);
                let points_len = AUTDEmulatorSoundFieldPointsLen(sound_field);
                let mut time = vec![0; len as _];
                let mut v = vec![vec![0.0f32; points_len as _]; len as _];

                let vp = v.iter_mut().map(|v| v.as_mut_ptr()).collect::<Vec<_>>();
                let res = AUTDEmulatorSoundFieldNext(
                    sound_field,
                    duration_ns,
                    time.as_mut_ptr(),
                    vp.as_ptr(),
                );
                let res = AUTDWaitLocalResultStatus(handle, res);
                assert_eq!(AUTDStatus::AUTDTrue, res.result);

                assert_eq!(
                    vec![
                        225000, 226000, 227000, 228000, 229000, 230000, 231000, 232000, 233000,
                        234000, 235000, 236000, 237000, 238000, 239000, 240000, 241000, 242000,
                        243000, 244000, 245000, 246000, 247000, 248000, 249000
                    ],
                    time
                );
                assert_eq!(
                    vec![
                        vec![190.96082, 81.954926, -43.268303,],
                        vec![136.85818, 16.451601, -93.33133,],
                        vec![73.53783, -45.61621, -141.77232,],
                        vec![14.913208, -93.125145, -181.12187,],
                        vec![-39.191376, -136.62769, -209.01646,],
                        vec![-89.41525, -176.0103, -224.6327,],
                        vec![-136.45963, -210.92961, -229.61143,],
                        vec![-177.71375, -232.3485, -221.54436,],
                        vec![-210.82788, -238.78197, -201.05437,],
                        vec![-234.92473, -235.05318, -166.11269,],
                        vec![-244.49503, -217.45775, -118.06169,],
                        vec![-242.93349, -189.04797, -63.85608,],
                        vec![-234.70769, -149.43028, -7.931027,],
                        vec![-210.70303, -101.2128, 52.683804,],
                        vec![-170.8035, -40.90981, 110.96725,],
                        vec![-114.901855, 28.199083, 158.98596,],
                        vec![-41.33793, 101.697235, 206.40062,],
                        vec![37.44646, 166.75967, 243.50082,],
                        vec![114.190094, 221.40279, 261.52933,],
                        vec![183.8636, 265.89404, 261.34058,],
                        vec![241.15149, 290.09818, 244.31012,],
                        vec![284.7516, 294.92493, 215.68811,],
                        vec![307.23727, 279.7575, 166.62605,],
                        vec![307.96487, 247.80507, 105.14593,],
                        vec![287.04877, 192.38678, 34.32131,],
                    ],
                    v
                );
            }

            AUTDEmulatorSoundFieldFree(sound_field);
            AUTDEmulatorRecordFree(record);
            AUTDEmulatorFree(emulator);
            AUTDDeleteRuntime(runtime);
        }
    }
}

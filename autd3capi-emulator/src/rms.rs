use crate::ptr::*;
use crate::range::*;
use crate::result::*;

use autd3_emulator::Rms;
use autd3capi_driver::{async_ffi::*, Duration, *};

#[repr(C)]
pub struct RmsRecordOption {
    pub sound_speed: f32,
    pub print_progress: bool,
    pub gpu: bool,
}

impl From<RmsRecordOption> for autd3_emulator::RmsRecordOption {
    fn from(value: RmsRecordOption) -> Self {
        autd3_emulator::RmsRecordOption {
            sound_speed: value.sound_speed,
            print_progress: value.print_progress,
            gpu: value.gpu,
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldRms(
    record: RecordPtr,
    range: RangeXYZ,
    option: RmsRecordOption,
) -> LocalFfiFuture<ResultRms> {
    async move {
        let r = record
            .sound_field(
                autd3_emulator::RangeXYZ::from(range),
                autd3_emulator::RmsRecordOption::from(option),
            )
            .await;
        r.into()
    }
    .into_local_ffi()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldRmsWait(
    handle: HandlePtr,
    future: LocalFfiFuture<ResultRms>,
) -> ResultRms {
    handle.block_on(future)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldRmsTimeLen(
    sound_field: RmsPtr,
    duration: Duration,
) -> u64 {
    sound_field.next_time_len(duration.into()) as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldRmsPointsLen(sound_field: RmsPtr) -> u64 {
    sound_field.next_points_len() as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldRmsGetX(sound_field: RmsPtr, x: *mut f32) {
    sound_field.x_inplace(std::slice::from_raw_parts_mut(
        x,
        sound_field.next_points_len(),
    ));
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldRmsGetY(sound_field: RmsPtr, y: *mut f32) {
    sound_field.y_inplace(std::slice::from_raw_parts_mut(
        y,
        sound_field.next_points_len(),
    ));
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldRmsGetZ(sound_field: RmsPtr, z: *mut f32) {
    sound_field.z_inplace(std::slice::from_raw_parts_mut(
        z,
        sound_field.next_points_len(),
    ));
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldRmsSkip(
    mut sound_field: RmsPtr,
    duration: Duration,
) -> LocalFfiFuture<ResultStatus> {
    async move {
        sound_field
            .next_inplace(duration.into(), true, &mut [], std::iter::empty())
            .await
            .into()
    }
    .into_local_ffi()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldRmsNext(
    mut sound_field: RmsPtr,
    duration: Duration,
    time: *mut u64,
    v: *const *mut f32,
) -> LocalFfiFuture<ResultStatus> {
    let n = sound_field.next_time_len(duration.into());
    let time = std::slice::from_raw_parts_mut(time, n as _);
    let iter = (0..n).map(move |i| v.add(i as _).read());
    async move {
        sound_field
            .next_inplace(duration.into(), false, time, iter)
            .await
            .into()
    }
    .into_local_ffi()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldRmsFree(sound_field: RmsPtr) {
    let _ = take!(sound_field, Rms);
}

#[cfg(test)]
mod tests {
    use autd3::prelude::{DcSysTime, Quaternion, ULTRASOUND_PERIOD};
    use autd3capi::{
        controller, gain, link::AUTDLinkGet, AUTDCreateRuntime, AUTDDeleteRuntime,
        AUTDGetRuntimeHandle, AUTDWaitLocalResultStatus, AUTDWaitResultStatus,
    };
    use tokio::runtime::Handle;

    use crate::{
        AUTDEmulator, AUTDEmulatorFree, AUTDEmulatorRecordFree, AUTDEmulatorRecordFrom,
        AUTDEmulatorTickNs, AUTDEmulatorWaitResultRecord,
    };

    use super::*;

    #[test]
    fn record_sound_field_rms() {
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

            let pos = [Point3::origin(); 1];
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
                DcSysTime::ZERO,
                std::mem::transmute::<unsafe extern "C" fn(ControllerPtr), ConstPtr>(f),
            );
            let record = AUTDEmulatorWaitResultRecord(handle, record);
            assert!(!record.result.0.is_null());
            let record = record.result;

            let range = RangeXYZ {
                x_start: -1.,
                x_end: 1.,
                y_start: 0.,
                y_end: 0.,
                z_start: 10.,
                z_end: 10.,
                resolution: 1.,
            };
            let option = RmsRecordOption {
                sound_speed: 340e3,
                print_progress: true,
                gpu: false,
            };
            let sound_filed = AUTDEmulatorSoundFieldRms(record, range, option);
            let sound_field = AUTDEmulatorSoundFieldRmsWait(handle, sound_filed);
            assert!(!sound_field.result.0.is_null());
            let sound_field = sound_field.result;

            {
                let res =
                    AUTDEmulatorSoundFieldRmsSkip(sound_field, (9 * ULTRASOUND_PERIOD).into());
                let res = AUTDWaitLocalResultStatus(handle, res);
                assert_eq!(AUTDStatus::AUTDTrue, res.result);
            }

            {
                let len = AUTDEmulatorSoundFieldRmsTimeLen(sound_field, ULTRASOUND_PERIOD.into());
                let points_len = AUTDEmulatorSoundFieldRmsPointsLen(sound_field);
                let mut time = vec![0; len as _];
                let mut v = vec![vec![0.0f32; points_len as _]; len as _];

                let vp = v.iter_mut().map(|v| v.as_mut_ptr()).collect::<Vec<_>>();
                let res = AUTDEmulatorSoundFieldRmsNext(
                    sound_field,
                    ULTRASOUND_PERIOD.into(),
                    time.as_mut_ptr(),
                    vp.as_ptr(),
                );
                let res = AUTDWaitLocalResultStatus(handle, res);
                assert_eq!(AUTDStatus::AUTDTrue, res.result);

                assert_eq!(vec![225000], time);
                assert_eq!(vec![vec![445.02795, 440.45087, 408.70248]], v);
            }

            AUTDEmulatorSoundFieldRmsFree(sound_field);
            AUTDEmulatorRecordFree(record);
            AUTDEmulatorFree(emulator);
            AUTDDeleteRuntime(runtime);
        }
    }
}

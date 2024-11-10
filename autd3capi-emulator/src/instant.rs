use crate::ptr::*;
use crate::range::*;
use crate::result::*;

use autd3_emulator::Instant;
use autd3capi_driver::{async_ffi::*, Duration, *};

#[repr(C)]
pub struct InstantRecordOption {
    pub sound_speed: f32,
    pub time_step: Duration,
    pub print_progress: bool,
    pub memory_limits_hint_mb: u64,
    pub gpu: bool,
}

impl From<InstantRecordOption> for autd3_emulator::InstantRecordOption {
    fn from(value: InstantRecordOption) -> Self {
        autd3_emulator::InstantRecordOption {
            sound_speed: value.sound_speed,
            time_step: value.time_step.into(),
            print_progress: value.print_progress,
            memory_limits_hint_mb: value.memory_limits_hint_mb as _,
            gpu: value.gpu,
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldInstant(
    record: RecordPtr,
    range: Range,
    option: InstantRecordOption,
) -> LocalFfiFuture<ResultInstant> {
    async move {
        let r = record
            .static_deref()
            .sound_field(
                range.into(),
                autd3_emulator::InstantRecordOption::from(option),
            )
            .await;
        r.into()
    }
    .into_local_ffi()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldInstantWait(
    handle: HandlePtr,
    future: LocalFfiFuture<ResultInstant>,
) -> ResultInstant {
    handle.block_on(future)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldInstantTimeLen(
    sound_field: InstantPtr,
    duration: Duration,
) -> u64 {
    sound_field.next_time_len(duration.into()) as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldInstantPointsLen(sound_field: InstantPtr) -> u64 {
    sound_field.next_points_len() as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldInstantGetX(sound_field: InstantPtr, x: *mut f32) {
    sound_field.x_inplace(std::slice::from_raw_parts_mut(
        x,
        sound_field.next_points_len(),
    ));
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldInstantGetY(sound_field: InstantPtr, y: *mut f32) {
    sound_field.y_inplace(std::slice::from_raw_parts_mut(
        y,
        sound_field.next_points_len(),
    ));
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldInstantGetZ(sound_field: InstantPtr, z: *mut f32) {
    sound_field.z_inplace(std::slice::from_raw_parts_mut(
        z,
        sound_field.next_points_len(),
    ));
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorSoundFieldInstantSkip(
    mut sound_field: InstantPtr,
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
pub unsafe extern "C" fn AUTDEmulatorSoundFieldInstantNext(
    mut sound_field: InstantPtr,
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
pub unsafe extern "C" fn AUTDEmulatorSoundFieldInstantFree(sound_field: InstantPtr) {
    let _ = take!(sound_field, Instant<'static>);
}

#[cfg(test)]
mod tests {
    use autd3::prelude::{Quaternion, ULTRASOUND_PERIOD};
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
    fn record_sound_field_instant() {
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

            let range = Range {
                x_start: -1.,
                x_end: 1.,
                y_start: 0.,
                y_end: 0.,
                z_start: 10.,
                z_end: 10.,
                resolution: 1.,
            };
            let option = InstantRecordOption {
                sound_speed: 340e3,
                time_step: std::time::Duration::from_micros(1).into(),
                print_progress: true,
                memory_limits_hint_mb: 128,
                gpu: false,
            };
            let sound_filed = AUTDEmulatorSoundFieldInstant(record, range, option);
            let sound_field = AUTDEmulatorSoundFieldInstantWait(handle, sound_filed);
            assert!(!sound_field.result.0.is_null());
            let sound_field = sound_field.result;

            {
                let res =
                    AUTDEmulatorSoundFieldInstantSkip(sound_field, (9 * ULTRASOUND_PERIOD).into());
                let res = AUTDWaitLocalResultStatus(handle, res);
                assert_eq!(AUTDStatus::AUTDTrue, res.result);
            }

            {
                let len =
                    AUTDEmulatorSoundFieldInstantTimeLen(sound_field, ULTRASOUND_PERIOD.into());
                let points_len = AUTDEmulatorSoundFieldInstantPointsLen(sound_field);
                let mut time = vec![0; len as _];
                let mut v = vec![vec![0.0f32; points_len as _]; len as _];

                let vp = v.iter_mut().map(|v| v.as_mut_ptr()).collect::<Vec<_>>();
                let res = AUTDEmulatorSoundFieldInstantNext(
                    sound_field,
                    ULTRASOUND_PERIOD.into(),
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
                        vec![190.96082, 81.954926, -43.268303],
                        vec![136.85695, 16.451164, -93.331795],
                        vec![73.53783, -45.61621, -141.77232],
                        vec![14.913369, -93.12651, -181.12183],
                        vec![-39.191376, -136.62769, -209.01646],
                        vec![-89.41609, -176.0108, -224.63297],
                        vec![-136.45963, -210.92961, -229.61143],
                        vec![-177.71426, -232.34837, -221.5439],
                        vec![-210.82788, -238.78197, -201.05437],
                        vec![-234.92447, -235.0538, -166.11209],
                        vec![-244.49503, -217.45775, -118.06169],
                        vec![-242.93336, -189.04541, -63.854618],
                        vec![-234.70769, -149.43028, -7.931027],
                        vec![-210.70303, -101.2128, 52.683804],
                        vec![-170.8035, -40.90981, 110.96725],
                        vec![-114.901855, 28.199083, 158.98596],
                        vec![-41.33793, 101.697235, 206.40062],
                        vec![37.44646, 166.75967, 243.50082],
                        vec![114.190094, 221.40279, 261.52933],
                        vec![183.8636, 265.89404, 261.34058],
                        vec![241.15149, 290.09818, 244.31012],
                        vec![284.7516, 294.92493, 215.68811],
                        vec![307.23727, 279.7575, 166.62605],
                        vec![307.96487, 247.80507, 105.14593],
                        vec![287.04877, 192.38678, 34.32131]
                    ],
                    v
                );
            }

            AUTDEmulatorSoundFieldInstantFree(sound_field);
            AUTDEmulatorRecordFree(record);
            AUTDEmulatorFree(emulator);
            AUTDDeleteRuntime(runtime);
        }
    }
}

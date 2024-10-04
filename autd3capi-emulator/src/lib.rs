#![allow(clippy::missing_safety_doc)]

mod option;
mod ptr;
mod range;
mod result;

use option::*;
use ptr::*;
use range::*;
use result::*;

use autd3_emulator::{Emulator, Record, Recorder, SoundField};
use autd3capi_driver::{async_ffi::*, autd3::prelude::*, *};
use driver::{ethercat::ECAT_DC_SYS_TIME_BASE, link::Link};

#[no_mangle]
#[must_use]
#[allow(clippy::box_default)]
pub unsafe extern "C" fn AUTDEmulator(
    pos: *const Vector3,
    rot: *const Quaternion,
    len: u16,
) -> EmulatorPtr {
    let pos = vec_from_raw!(pos, Vector3, len);
    let rot = vec_from_raw!(rot, Quaternion, len);
    EmulatorPtr::new(Emulator::new(pos.into_iter().zip(rot).map(|(p, r)| {
        AUTD3::new(p).with_rotation(UnitQuaternion::from_quaternion(r))
    })))
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorFree(emulator: EmulatorPtr) {
    let _ = take!(emulator, Emulator);
}

#[no_mangle]
#[must_use]
#[allow(clippy::box_default)]
pub unsafe extern "C" fn AUTDEmulatorWithParallelThreshold(
    emulator: EmulatorPtr,
    parallel_threshold: u16,
) -> EmulatorPtr {
    EmulatorPtr::new(take!(emulator, Emulator).with_parallel_threshold(parallel_threshold as _))
}

#[no_mangle]
#[must_use]
#[allow(clippy::box_default)]
pub unsafe extern "C" fn AUTDEmulatorWithSendInterval(
    emulator: EmulatorPtr,
    interval_ns: u64,
) -> EmulatorPtr {
    EmulatorPtr::new(
        take!(emulator, Emulator).with_send_interval(std::time::Duration::from_nanos(interval_ns)),
    )
}

#[no_mangle]
#[must_use]
#[allow(clippy::box_default)]
pub unsafe extern "C" fn AUTDEmulatorWithReceiveInterval(
    emulator: EmulatorPtr,
    interval_ns: u64,
) -> EmulatorPtr {
    EmulatorPtr::new(
        take!(emulator, Emulator)
            .with_receive_interval(std::time::Duration::from_nanos(interval_ns)),
    )
}

#[no_mangle]
#[must_use]
#[allow(clippy::box_default)]
pub unsafe extern "C" fn AUTDEmulatorWithTimerResolution(
    emulator: EmulatorPtr,
    resolution: u32,
) -> EmulatorPtr {
    #[cfg(target_os = "windows")]
    {
        EmulatorPtr::new(
            take!(emulator, Emulator).with_timer_resolution(std::num::NonZeroU32::new(resolution)),
        )
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = resolution;
        emulator
    }
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
pub unsafe extern "C" fn AUTDEmulatorTickNs(
    mut record: LinkPtr,
    tick_ns: u64,
) -> ResultEmualtorErr {
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
pub unsafe extern "C" fn AUTDEmulatorRecordDriveTime(record: RecordPtr, time: *mut f32) {
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
pub unsafe extern "C" fn AUTDEmulatorRecordOutputTime(record: RecordPtr, time: *mut f32) {
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
) -> LocalFfiFuture<ResultEmualtorErr> {
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
    time: *mut f32,
    v: *const *mut f32,
) -> LocalFfiFuture<ResultEmualtorErr> {
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
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorWaitResultEmualtorErr(
    handle: HandlePtr,
    future: LocalFfiFuture<ResultEmualtorErr>,
) -> ResultEmualtorErr {
    handle.block_on(future)
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

            let handle = HandlePtr(&handle as *const _ as _);

            let link_ptr = AUTDLinkGet(cnt);

            let g = gain::uniform::AUTDGainUniform(0xFF, 0x40);
            let d = gain::AUTDGainIntoDatagram(g);

            let future = controller::AUTDControllerSend(cnt, d);
            let result = AUTDWaitResultI32(handle, future);
            assert_eq!(AUTD3_TRUE, result.result);

            let result = AUTDEmulatorTickNs(link_ptr, 10 * ULTRASOUND_PERIOD.as_nanos() as u64);
            assert_eq!(AUTD3_TRUE, result.result);
        }

        unsafe {
            let runtime = AUTDCreateRuntime();
            let handle = AUTDGetRuntimeHandle(runtime);

            let emulator = {
                let pos = [Vector3::new(0.0, 0.0, 0.0); 1];
                let rot = [Quaternion::new(1.0, 0.0, 0.0, 0.0); 1];
                AUTDEmulator(pos.as_ptr(), rot.as_ptr(), 1)
            };

            let record = AUTDEmulatorRecordFrom(
                emulator,
                0,
                std::mem::transmute::<unsafe extern "C" fn(ControllerPtr), ConstPtr>(f),
            );
            let record = AUTDEmulatorWaitResultRecord(handle, record);
            assert!(!record.result.0.is_null());
            let record = record.result;

            let len = AUTDEmulatorRecordDriveLen(record);
            let mut time = vec![0.0; len as _];
            let mut pulsewidth = vec![0; len as _];
            let mut phase = vec![0; len as _];
            AUTDEmulatorRecordDriveTime(record, time.as_mut_ptr());
            AUTDEmulatorRecordDrivePulseWidth(record, 0, 0, pulsewidth.as_mut_ptr());
            AUTDEmulatorRecordDrivePhase(record, 0, 0, phase.as_mut_ptr());

            assert_eq!(
                vec![
                    0.0, 2.5e-5, 5e-5, 7.5e-5, 0.0001, 0.000125, 0.00015, 0.000175, 0.0002,
                    0.000225
                ],
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

            let handle = HandlePtr(&handle as *const _ as _);

            let link_ptr = AUTDLinkGet(cnt);

            let g = gain::uniform::AUTDGainUniform(0xFF, 0x40);
            let d = gain::AUTDGainIntoDatagram(g);

            let future = controller::AUTDControllerSend(cnt, d);
            let result = AUTDWaitResultI32(handle, future);
            assert_eq!(AUTD3_TRUE, result.result);

            let result = AUTDEmulatorTickNs(link_ptr, ULTRASOUND_PERIOD.as_nanos() as u64);
            assert_eq!(AUTD3_TRUE, result.result);
        }

        unsafe {
            let runtime = AUTDCreateRuntime();
            let handle = AUTDGetRuntimeHandle(runtime);

            let emulator = {
                let pos = [Vector3::new(0.0, 0.0, 0.0); 1];
                let rot = [Quaternion::new(1.0, 0.0, 0.0, 0.0); 1];
                AUTDEmulator(pos.as_ptr(), rot.as_ptr(), 1)
            };

            let record = AUTDEmulatorRecordFrom(
                emulator,
                0,
                std::mem::transmute::<unsafe extern "C" fn(ControllerPtr), ConstPtr>(f),
            );
            let record = AUTDEmulatorWaitResultRecord(handle, record);
            assert!(!record.result.0.is_null());
            let record = record.result;

            let len = AUTDEmulatorRecordOutputLen(record);
            let mut time = vec![0.0; len as _];
            let mut v = vec![0.; len as _];
            AUTDEmulatorRecordOutputTime(record, time.as_mut_ptr());
            AUTDEmulatorRecordOutputVoltage(record, 0, 0, v.as_mut_ptr());

            assert_eq!(
                vec![
                    0.0,
                    9.765625e-8,
                    1.953125e-7,
                    2.9296874e-7,
                    3.90625e-7,
                    4.882812e-7,
                    5.8593747e-7,
                    6.835937e-7,
                    7.8125e-7,
                    8.7890623e-7,
                    9.765624e-7,
                    1.0742187e-6,
                    1.1718749e-6,
                    1.2695313e-6,
                    1.3671875e-6,
                    1.4648438e-6,
                    1.5625e-6,
                    1.6601562e-6,
                    1.7578125e-6,
                    1.8554687e-6,
                    1.9531249e-6,
                    2.0507812e-6,
                    2.1484375e-6,
                    2.2460938e-6,
                    2.3437499e-6,
                    2.4414062e-6,
                    2.5390625e-6,
                    2.6367186e-6,
                    2.734375e-6,
                    2.8320312e-6,
                    2.9296875e-6,
                    3.0273436e-6,
                    3.125e-6,
                    3.2226562e-6,
                    3.3203123e-6,
                    3.4179686e-6,
                    3.515625e-6,
                    3.6132813e-6,
                    3.7109373e-6,
                    3.8085936e-6,
                    3.9062497e-6,
                    4.0039063e-6,
                    4.1015624e-6,
                    4.1992184e-6,
                    4.296875e-6,
                    4.394531e-6,
                    4.4921876e-6,
                    4.5898437e-6,
                    4.6874998e-6,
                    4.7851563e-6,
                    4.8828124e-6,
                    4.9804685e-6,
                    5.078125e-6,
                    5.175781e-6,
                    5.273437e-6,
                    5.3710937e-6,
                    5.46875e-6,
                    5.566406e-6,
                    5.6640624e-6,
                    5.7617185e-6,
                    5.859375e-6,
                    5.957031e-6,
                    6.054687e-6,
                    6.1523438e-6,
                    6.25e-6,
                    6.347656e-6,
                    6.4453125e-6,
                    6.5429685e-6,
                    6.6406246e-6,
                    6.738281e-6,
                    6.8359373e-6,
                    6.933594e-6,
                    7.03125e-6,
                    7.128906e-6,
                    7.2265625e-6,
                    7.3242186e-6,
                    7.4218747e-6,
                    7.519531e-6,
                    7.6171873e-6,
                    7.714843e-6,
                    7.8124995e-6,
                    7.9101565e-6,
                    8.0078125e-6,
                    8.105469e-6,
                    8.203125e-6,
                    8.300781e-6,
                    8.398437e-6,
                    8.496094e-6,
                    8.59375e-6,
                    8.691406e-6,
                    8.789062e-6,
                    8.886718e-6,
                    8.984375e-6,
                    9.082031e-6,
                    9.179687e-6,
                    9.277343e-6,
                    9.3749995e-6,
                    9.472656e-6,
                    9.570313e-6,
                    9.667969e-6,
                    9.765625e-6,
                    9.863281e-6,
                    9.960937e-6,
                    1.0058594e-5,
                    1.015625e-5,
                    1.0253906e-5,
                    1.0351562e-5,
                    1.0449218e-5,
                    1.0546874e-5,
                    1.0644531e-5,
                    1.0742187e-5,
                    1.08398435e-5,
                    1.09375e-5,
                    1.1035156e-5,
                    1.1132812e-5,
                    1.1230469e-5,
                    1.1328125e-5,
                    1.1425781e-5,
                    1.1523437e-5,
                    1.1621093e-5,
                    1.171875e-5,
                    1.1816406e-5,
                    1.1914062e-5,
                    1.2011718e-5,
                    1.2109374e-5,
                    1.22070305e-5,
                    1.23046875e-5,
                    1.2402344e-5,
                    1.25e-5,
                    1.2597656e-5,
                    1.2695312e-5,
                    1.2792969e-5,
                    1.2890625e-5,
                    1.2988281e-5,
                    1.3085937e-5,
                    1.3183593e-5,
                    1.3281249e-5,
                    1.3378906e-5,
                    1.3476562e-5,
                    1.3574218e-5,
                    1.36718745e-5,
                    1.3769531e-5,
                    1.3867188e-5,
                    1.3964844e-5,
                    1.40625e-5,
                    1.4160156e-5,
                    1.4257812e-5,
                    1.4355468e-5,
                    1.4453125e-5,
                    1.4550781e-5,
                    1.4648437e-5,
                    1.4746093e-5,
                    1.4843749e-5,
                    1.4941405e-5,
                    1.5039062e-5,
                    1.51367185e-5,
                    1.5234375e-5,
                    1.533203e-5,
                    1.5429687e-5,
                    1.5527343e-5,
                    1.5624999e-5,
                    1.5722655e-5,
                    1.5820313e-5,
                    1.5917969e-5,
                    1.6015625e-5,
                    1.6113281e-5,
                    1.6210937e-5,
                    1.6308593e-5,
                    1.640625e-5,
                    1.6503905e-5,
                    1.6601562e-5,
                    1.6699218e-5,
                    1.6796874e-5,
                    1.6894532e-5,
                    1.6992188e-5,
                    1.7089844e-5,
                    1.71875e-5,
                    1.7285156e-5,
                    1.7382812e-5,
                    1.7480468e-5,
                    1.7578124e-5,
                    1.767578e-5,
                    1.7773436e-5,
                    1.7871092e-5,
                    1.796875e-5,
                    1.8066406e-5,
                    1.8164063e-5,
                    1.8261719e-5,
                    1.8359375e-5,
                    1.845703e-5,
                    1.8554687e-5,
                    1.8652343e-5,
                    1.8749999e-5,
                    1.8847655e-5,
                    1.8945311e-5,
                    1.904297e-5,
                    1.9140625e-5,
                    1.9238281e-5,
                    1.9335937e-5,
                    1.9433593e-5,
                    1.953125e-5,
                    1.9628906e-5,
                    1.9726562e-5,
                    1.9824218e-5,
                    1.9921874e-5,
                    2.001953e-5,
                    2.0117188e-5,
                    2.0214844e-5,
                    2.03125e-5,
                    2.0410156e-5,
                    2.0507812e-5,
                    2.0605468e-5,
                    2.0703124e-5,
                    2.080078e-5,
                    2.0898437e-5,
                    2.0996093e-5,
                    2.1093749e-5,
                    2.1191405e-5,
                    2.1289063e-5,
                    2.1386719e-5,
                    2.1484375e-5,
                    2.1582031e-5,
                    2.1679687e-5,
                    2.1777343e-5,
                    2.1875e-5,
                    2.1972655e-5,
                    2.2070311e-5,
                    2.2167967e-5,
                    2.2265624e-5,
                    2.2363281e-5,
                    2.2460938e-5,
                    2.2558594e-5,
                    2.265625e-5,
                    2.2753906e-5,
                    2.2851562e-5,
                    2.2949218e-5,
                    2.3046874e-5,
                    2.314453e-5,
                    2.3242186e-5,
                    2.3339842e-5,
                    2.34375e-5,
                    2.3535156e-5,
                    2.3632812e-5,
                    2.3730468e-5,
                    2.3828125e-5,
                    2.392578e-5,
                    2.4023437e-5,
                    2.4121093e-5,
                    2.4218749e-5,
                    2.4316405e-5,
                    2.4414061e-5,
                    2.4511719e-5,
                    2.4609375e-5,
                    2.4707031e-5,
                    2.4804687e-5,
                    2.4902343e-5,
                ],
                time
            );
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

            let handle = HandlePtr(&handle as *const _ as _);

            let link_ptr = AUTDLinkGet(cnt);

            let g = gain::uniform::AUTDGainUniform(0xFF, 0x40);
            let d = gain::AUTDGainIntoDatagram(g);

            let future = controller::AUTDControllerSend(cnt, d);
            let result = AUTDWaitResultI32(handle, future);
            assert_eq!(AUTD3_TRUE, result.result);

            let result = AUTDEmulatorTickNs(link_ptr, 10 * ULTRASOUND_PERIOD.as_nanos() as u64);
            assert_eq!(AUTD3_TRUE, result.result);
        }

        unsafe {
            let runtime = AUTDCreateRuntime();
            let handle = AUTDGetRuntimeHandle(runtime);

            let emulator = {
                let pos = [Vector3::new(0.0, 0.0, 0.0); 1];
                let rot = [Quaternion::new(1.0, 0.0, 0.0, 0.0); 1];
                AUTDEmulator(pos.as_ptr(), rot.as_ptr(), 1)
            };

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
                let res = AUTDEmulatorWaitResultEmualtorErr(handle, res);
                assert_eq!(AUTD3_TRUE, res.result);
            }

            {
                let duration_ns = ULTRASOUND_PERIOD.as_nanos() as u64;
                let len = AUTDEmulatorSoundFieldTimeLen(sound_field, duration_ns);
                let points_len = AUTDEmulatorSoundFieldPointsLen(sound_field);
                let mut time = vec![0.0f32; len as _];
                let mut v = vec![vec![0.0f32; points_len as _]; len as _];

                let vp = v.iter_mut().map(|v| v.as_mut_ptr()).collect::<Vec<_>>();
                let res = AUTDEmulatorSoundFieldNext(
                    sound_field,
                    duration_ns,
                    time.as_mut_ptr(),
                    vp.as_ptr(),
                );
                let res = AUTDEmulatorWaitResultEmualtorErr(handle, res);
                assert_eq!(AUTD3_TRUE, res.result);

                assert_eq!(
                    vec![
                        0.000225,
                        0.00022599999,
                        0.000227,
                        0.00022799999,
                        0.000229,
                        0.00022999999,
                        0.000231,
                        0.00023199999,
                        0.000233,
                        0.00023399999,
                        0.000235,
                        0.00023599999,
                        0.000237,
                        0.000238,
                        0.000239,
                        0.00024,
                        0.000241,
                        0.000242,
                        0.000243,
                        0.000244,
                        0.000245,
                        0.000246,
                        0.000247,
                        0.000248,
                        0.000249,
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

use crate::RecordPtr;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorRecordDriveCols(record: RecordPtr) -> u64 {
    record.drive_cols() as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorRecordDriveRows(record: RecordPtr) -> u64 {
    record.drive_rows() as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorRecordPhase(
    record: RecordPtr,
    time: *mut u64,
    v: *const *mut u8,
) {
    let n = record.drive_cols();
    record.phase_inplace(
        std::slice::from_raw_parts_mut(time, n),
        (0..n).map(move |i| v.add(i as _).read()),
    );
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorRecordPulseWidth(
    record: RecordPtr,
    time: *mut u64,
    v: *const *mut u8,
) {
    let n = record.drive_cols();
    record.pulse_width_inplace(
        std::slice::from_raw_parts_mut(time, n),
        (0..n).map(move |i| v.add(i as _).read()),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        tests::create_emulator, AUTDEmulatorFree, AUTDEmulatorRecordFree, AUTDEmulatorRecordFrom,
        AUTDEmulatorTickNs, AUTDEmulatorWaitResultRecord,
    };
    use autd3::prelude::{DcSysTime, ULTRASOUND_PERIOD};
    use autd3capi::*;
    use autd3capi_driver::*;
    use link::AUTDLinkGet;
    use tokio::runtime::Handle;

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

    #[test]
    fn record_phase() {
        unsafe {
            let runtime = AUTDCreateRuntime();
            let handle = AUTDGetRuntimeHandle(runtime);

            let emulator = create_emulator();

            let record = AUTDEmulatorRecordFrom(
                emulator,
                DcSysTime::ZERO,
                std::mem::transmute::<unsafe extern "C" fn(ControllerPtr), ConstPtr>(f),
            );
            let record = AUTDEmulatorWaitResultRecord(handle, record);
            assert!(!record.result.0.is_null());
            let record = record.result;

            let cols = AUTDEmulatorRecordDriveCols(record);
            let rows = AUTDEmulatorRecordDriveRows(record);
            let mut time = vec![0; cols as _];
            let mut phase = vec![vec![0; rows as _]; cols as _];
            let pphase = phase.iter_mut().map(|v| v.as_mut_ptr()).collect::<Vec<_>>();
            AUTDEmulatorRecordPhase(record, time.as_mut_ptr(), pphase.as_ptr());

            assert_eq!(
                vec![0, 25000, 50000, 75000, 100000, 125000, 150000, 175000, 200000, 225000],
                time
            );
            let expect = [1, 3, 4, 6, 8, 9, 11, 12, 14, 16];
            (0..cols as usize).for_each(|col| {
                (0..rows as usize).for_each(|row| assert_eq!(expect[col], phase[col][row]));
            });

            AUTDEmulatorRecordFree(record);
            AUTDEmulatorFree(emulator);
            AUTDDeleteRuntime(runtime);
        }
    }

    #[test]
    fn record_pulse_width() {
        unsafe {
            let runtime = AUTDCreateRuntime();
            let handle = AUTDGetRuntimeHandle(runtime);

            let emulator = create_emulator();

            let record = AUTDEmulatorRecordFrom(
                emulator,
                DcSysTime::ZERO,
                std::mem::transmute::<unsafe extern "C" fn(ControllerPtr), ConstPtr>(f),
            );
            let record = AUTDEmulatorWaitResultRecord(handle, record);
            assert!(!record.result.0.is_null());
            let record = record.result;

            let cols = AUTDEmulatorRecordDriveCols(record);
            let rows = AUTDEmulatorRecordDriveRows(record);
            let mut time = vec![0; cols as _];
            let mut phase = vec![vec![0; rows as _]; cols as _];
            let pphase = phase.iter_mut().map(|v| v.as_mut_ptr()).collect::<Vec<_>>();
            AUTDEmulatorRecordPulseWidth(record, time.as_mut_ptr(), pphase.as_ptr());

            assert_eq!(
                vec![0, 25000, 50000, 75000, 100000, 125000, 150000, 175000, 200000, 225000],
                time
            );
            let expect = [8, 16, 25, 34, 42, 52, 63, 76, 91, 128];
            (0..cols as usize).for_each(|col| {
                (0..rows as usize).for_each(|row| assert_eq!(expect[col], phase[col][row]));
            });

            AUTDEmulatorRecordFree(record);
            AUTDEmulatorFree(emulator);
            AUTDDeleteRuntime(runtime);
        }
    }
}

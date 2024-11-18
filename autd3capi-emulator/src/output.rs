use crate::RecordPtr;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmulatorRecordOutputCols(record: RecordPtr) -> u64 {
    record.output_cols() as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorRecordOutputVoltage(record: RecordPtr, v: *const *mut f32) {
    record.output_voltage_inplace((0..record.output_cols()).map(move |i| v.add(i as _).read()));
}

#[no_mangle]
pub unsafe extern "C" fn AUTDEmulatorRecordOutputUltrasound(record: RecordPtr, v: *const *mut f32) {
    record.output_ultrasound_inplace((0..record.output_cols()).map(move |i| v.add(i as _).read()));
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{
        drive::AUTDEmulatorRecordDriveRows, tests::create_emulator, AUTDEmulatorFree,
        AUTDEmulatorRecordFree, AUTDEmulatorRecordFrom, AUTDEmulatorTickNs,
        AUTDEmulatorWaitResultRecord,
    };

    use super::*;
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

        let result = AUTDEmulatorTickNs(link_ptr, ULTRASOUND_PERIOD.into());
        assert_eq!(AUTDStatus::AUTDTrue, result.result);
    }

    #[test]
    fn record_output_voltage() {
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

            let cols = AUTDEmulatorRecordOutputCols(record);
            let rows = AUTDEmulatorRecordDriveRows(record);
            let mut v = vec![vec![0.; rows as _]; cols as _];
            let pv = v.iter_mut().map(|v| v.as_mut_ptr()).collect::<Vec<_>>();
            AUTDEmulatorRecordOutputVoltage(record, pv.as_ptr());

            let expect = vec![
                12.0, 12.0, 12.0, 12.0, 12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0, -12.0,
                -12.0, 12.0, 12.0, 12.0,
            ];
            (0..cols as usize).for_each(|col| {
                (0..rows as usize).for_each(|row| assert_eq!(expect[col], v[col][row]));
            });

            AUTDEmulatorRecordFree(record);
            AUTDEmulatorFree(emulator);
            AUTDDeleteRuntime(runtime);
        }
    }
}

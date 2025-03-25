#![allow(clippy::missing_safety_doc)]

use autd3::core::{datagram::Segment, gain::Drive};
use autd3capi_driver::{autd3::link::Audit, *};
use driver::autd3_device::AUTD3;

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAudit() -> LinkPtr {
    Audit::new(autd3::link::AuditOption::default()).into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditIsOpen(audit: LinkPtr) -> bool {
    use autd3::core::link::Link;
    audit.cast::<Audit>().is_open()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDLinkAuditBreakDown(mut audit: LinkPtr) {
    audit.cast_mut::<Audit>().break_down()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDLinkAuditRepair(mut audit: LinkPtr) {
    audit.cast_mut::<Audit>().repair()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditCpuNumTransducers(audit: LinkPtr, idx: u16) -> u32 {
    audit.cast::<Audit>()[idx as usize].num_transducers() as _
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDLinkAuditFpgaAssertThermalSensor(mut audit: LinkPtr, idx: u16) {
    audit.cast_mut::<Audit>()[idx as usize]
        .fpga_mut()
        .assert_thermal_sensor()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDLinkAuditFpgaDeassertThermalSensor(mut audit: LinkPtr, idx: u16) {
    audit.cast_mut::<Audit>()[idx as usize]
        .fpga_mut()
        .deassert_thermal_sensor()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaIsForceFan(audit: LinkPtr, idx: u16) -> bool {
    audit.cast::<Audit>()[idx as usize].fpga().is_force_fan()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaCurrentStmSegment(audit: LinkPtr, idx: u16) -> Segment {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .current_stm_segment()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaCurrentModSegment(audit: LinkPtr, idx: u16) -> Segment {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .current_mod_segment()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaIsStmGainMode(
    audit: LinkPtr,
    segment: Segment,
    idx: u16,
) -> bool {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .is_stm_gain_mode(segment)
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditCpuSilencerStrictMode(audit: LinkPtr, idx: u16) -> bool {
    audit.cast::<Audit>()[idx as usize].silencer_strict_mode()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaSilencerUpdateRateIntensity(
    audit: LinkPtr,
    idx: u16,
) -> u16 {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .silencer_update_rate()
        .intensity
        .get()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaSilencerUpdateRatePhase(audit: LinkPtr, idx: u16) -> u16 {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .silencer_update_rate()
        .phase
        .get()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaSilencerCompletionStepsIntensity(
    audit: LinkPtr,
    idx: u16,
) -> u16 {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .silencer_completion_steps()
        .intensity
        .get()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaSilencerCompletionStepsPhase(
    audit: LinkPtr,
    idx: u16,
) -> u16 {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .silencer_completion_steps()
        .phase
        .get()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaSilencerFixedCompletionStepsMode(
    audit: LinkPtr,
    idx: u16,
) -> bool {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .silencer_fixed_completion_steps_mode()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDLinkAuditFpgaGPIOOutputTypes(audit: LinkPtr, idx: u16, ty: *mut u8) {
    unsafe {
        let src = audit.cast::<Audit>()[idx as usize].fpga().debug_types();
        std::ptr::copy_nonoverlapping(src.as_ptr(), ty, src.len())
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDLinkAuditFpgaDebugValues(audit: LinkPtr, idx: u16, value: *mut u64) {
    unsafe {
        let src = audit.cast::<Audit>()[idx as usize].fpga().debug_values();
        std::ptr::copy_nonoverlapping(src.as_ptr(), value, src.len())
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaStmFreqDivision(
    audit: LinkPtr,
    segment: Segment,
    idx: u16,
) -> u16 {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .stm_freq_division(segment)
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaStmCycle(
    audit: LinkPtr,
    segment: Segment,
    idx: u16,
) -> u16 {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .stm_cycle(segment) as _
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaSoundSpeed(
    audit: LinkPtr,
    segment: Segment,
    idx: u16,
) -> u16 {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .sound_speed(segment)
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaStmLoopBehavior(
    audit: LinkPtr,
    segment: Segment,
    idx: u16,
) -> LoopBehavior {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .stm_loop_behavior(segment)
        .into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaModulationFreqDivision(
    audit: LinkPtr,
    segment: Segment,
    idx: u16,
) -> u16 {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .modulation_freq_division(segment)
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaModulationCycle(
    audit: LinkPtr,
    segment: Segment,
    idx: u16,
) -> u16 {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .modulation_cycle(segment) as _
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDLinkAuditFpgaModulationBuffer(
    audit: LinkPtr,
    segment: Segment,
    idx: u16,
    data: *mut u8,
    size: u32,
) {
    unsafe {
        let dst = std::slice::from_raw_parts_mut(data, size as _);
        audit.cast::<Audit>()[idx as usize]
            .fpga()
            .modulation_buffer_inplace(segment, dst);
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaModulationLoopBehavior(
    audit: LinkPtr,
    segment: Segment,
    idx: u16,
) -> LoopBehavior {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .modulation_loop_behavior(segment)
        .into()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDLinkAuditFpgaDrivesAt(
    audit: LinkPtr,
    segment: Segment,
    idx: u16,
    stm_idx: u16,
    drive: *mut Drive,
) {
    unsafe {
        let dst = std::slice::from_raw_parts_mut(drive, AUTD3::NUM_TRANS_IN_UNIT);
        audit.cast::<Audit>()[idx as usize]
            .fpga()
            .drives_at_inplace(segment, stm_idx as _, dst);
    }
}

#[allow(clippy::unnecessary_operation)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDLinkAuditFpgaPulseWidthEncoderTable(
    audit: LinkPtr,
    idx: u16,
    dst: *mut u16,
) {
    use autd3::driver::firmware::fpga::PulseWidth;
    unsafe {
        let dst = std::slice::from_raw_parts_mut(
            dst as *mut PulseWidth<u16, 9>,
            autd3::driver::firmware::fpga::PWE_BUF_SIZE,
        );
        let fpga = audit.cast::<Audit>()[idx as usize].fpga();
        fpga.pulse_width_encoder_table_inplace(dst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use autd3::driver::firmware::fpga::PulseWidth;

    #[test]
    fn test_pulse_width_size() {
        assert_eq!(size_of::<u16>(), size_of::<PulseWidth<u16, 9>>());
    }
}

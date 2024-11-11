#![allow(clippy::missing_safety_doc)]

use autd3::{
    derive::{Drive, LoopBehavior, Segment},
    prelude::{SilencerTarget, ULTRASOUND_PERIOD},
};
use autd3capi_driver::{autd3::link::audit::*, driver::link::Link, *};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAudit() -> LinkBuilderPtr {
    Audit::builder().into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditIsOpen(audit: LinkPtr) -> bool {
    audit.is_open()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkAuditDown(mut audit: LinkPtr) {
    audit.cast_mut::<Audit>().down()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkAuditUp(mut audit: LinkPtr) {
    audit.cast_mut::<Audit>().up()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkAuditBreakDown(mut audit: LinkPtr) {
    audit.cast_mut::<Audit>().break_down()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkAuditRepair(mut audit: LinkPtr) {
    audit.cast_mut::<Audit>().repair()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditLastTimeout(audit: LinkPtr) -> OptionDuration {
    audit.cast::<Audit>().last_timeout().into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditLastParallelThreshold(audit: LinkPtr) -> i64 {
    audit
        .cast::<Audit>()
        .last_parallel_threshold()
        .map(|t| t as i64)
        .unwrap_or(-1)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditCpuNumTransducers(audit: LinkPtr, idx: u16) -> u32 {
    audit.cast::<Audit>()[idx as usize].num_transducers() as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkAuditFpgaAssertThermalSensor(mut audit: LinkPtr, idx: u16) {
    audit.cast_mut::<Audit>()[idx as usize]
        .fpga_mut()
        .assert_thermal_sensor()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkAuditFpgaDeassertThermalSensor(mut audit: LinkPtr, idx: u16) {
    audit.cast_mut::<Audit>()[idx as usize]
        .fpga_mut()
        .deassert_thermal_sensor()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaIsForceFan(audit: LinkPtr, idx: u16) -> bool {
    audit.cast::<Audit>()[idx as usize].fpga().is_force_fan()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaCurrentStmSegment(audit: LinkPtr, idx: u16) -> Segment {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .current_stm_segment()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaCurrentModSegment(audit: LinkPtr, idx: u16) -> Segment {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .current_mod_segment()
}

#[no_mangle]
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

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditCpuSilencerStrictMode(audit: LinkPtr, idx: u16) -> bool {
    audit.cast::<Audit>()[idx as usize].silencer_strict_mode()
}

#[no_mangle]
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

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaSilencerUpdateRatePhase(audit: LinkPtr, idx: u16) -> u16 {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .silencer_update_rate()
        .phase
        .get()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaSilencerCompletionStepsIntensity(
    audit: LinkPtr,
    idx: u16,
) -> Duration {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .silencer_completion_steps()
        .intensity
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaSilencerCompletionStepsPhase(
    audit: LinkPtr,
    idx: u16,
) -> Duration {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .silencer_completion_steps()
        .phase
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaSilencerFixedCompletionStepsMode(
    audit: LinkPtr,
    idx: u16,
) -> bool {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .silencer_fixed_completion_steps_mode()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaSilencerTarget(
    audit: LinkPtr,
    idx: u16,
) -> SilencerTarget {
    audit.cast::<Audit>()[idx as usize].fpga().silencer_target()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkAuditFpgaDebugTypes(audit: LinkPtr, idx: u16, ty: *mut u8) {
    let src = audit.cast::<Audit>()[idx as usize].fpga().debug_types();
    std::ptr::copy_nonoverlapping(src.as_ptr(), ty, src.len())
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkAuditFpgaDebugValues(audit: LinkPtr, idx: u16, value: *mut u64) {
    let src = audit.cast::<Audit>()[idx as usize].fpga().debug_values();
    std::ptr::copy_nonoverlapping(src.as_ptr(), value, src.len())
}

#[no_mangle]
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

#[no_mangle]
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

#[no_mangle]
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

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaStmLoopBehavior(
    audit: LinkPtr,
    segment: Segment,
    idx: u16,
) -> LoopBehavior {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .stm_loop_behavior(segment)
}

#[no_mangle]
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

#[no_mangle]
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

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkAuditFpgaModulationBuffer(
    audit: LinkPtr,
    segment: Segment,
    idx: u16,
    data: *mut u8,
    size: u32,
) {
    let dst = std::slice::from_raw_parts_mut(data, size as _);
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .modulation_buffer_inplace(segment, dst);
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaModulationLoopBehavior(
    audit: LinkPtr,
    segment: Segment,
    idx: u16,
) -> LoopBehavior {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .modulation_loop_behavior(segment)
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkAuditFpgaDrivesAt(
    audit: LinkPtr,
    segment: Segment,
    idx: u16,
    stm_idx: u16,
    drive: *mut Drive,
) {
    let dst = std::slice::from_raw_parts_mut(drive, autd3::prelude::AUTD3::NUM_TRANS_IN_UNIT);
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .drives_at_inplace(segment, stm_idx as _, dst);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkAuditFpgaPulseWidthEncoderTable(
    audit: LinkPtr,
    idx: u16,
    dst: *mut u8,
) {
    let dst = std::slice::from_raw_parts_mut(dst, autd3::driver::firmware::fpga::PWE_BUF_SIZE);
    let fpga = audit.cast::<Audit>()[idx as usize].fpga();
    fpga.pulse_width_encoder_table_inplace(dst);
}

#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{autd3::link::audit::*, driver::link::Link, *};
use std::time::Duration;

#[repr(C)]
pub struct LinkAuditBuilderPtr(pub ConstPtr);

impl LinkAuditBuilderPtr {
    pub fn new(builder: AuditBuilder) -> Self {
        Self(Box::into_raw(Box::new(builder)) as _)
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAudit() -> LinkAuditBuilderPtr {
    LinkAuditBuilderPtr::new(Audit::builder())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditWithTimeout(
    audit: LinkAuditBuilderPtr,
    timeout_ns: u64,
) -> LinkAuditBuilderPtr {
    LinkAuditBuilderPtr::new(
        take!(audit, AuditBuilder).with_timeout(Duration::from_nanos(timeout_ns)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditIntoBuilder(audit: LinkAuditBuilderPtr) -> LinkBuilderPtr {
    SyncLinkBuilder::new(*take!(audit, AuditBuilder))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditIsOpen(audit: LinkPtr) -> bool {
    audit.is_open()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditTimeoutNs(audit: LinkPtr) -> u64 {
    audit.timeout().as_nanos() as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditLastTimeoutNs(audit: LinkPtr) -> i64 {
    audit
        .cast::<Audit>()
        .last_timeout()
        .map(|v| v.as_nanos() as _)
        .unwrap_or(-1)
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
pub unsafe extern "C" fn AUTDLinkAuditCpuUpdate(mut audit: LinkPtr, idx: u32) {
    audit.cast_mut::<Audit>()[idx as usize].update()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditCpuIdx(audit: LinkPtr, idx: u32) -> u32 {
    audit.cast::<Audit>()[idx as usize].idx() as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditCpuNumTransducers(audit: LinkPtr, idx: u32) -> u32 {
    audit.cast::<Audit>()[idx as usize].num_transducers() as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditCpuAck(audit: LinkPtr, idx: u32) -> u8 {
    audit.cast::<Audit>()[idx as usize].ack()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditCpuRxData(audit: LinkPtr, idx: u32) -> u8 {
    audit.cast::<Audit>()[idx as usize].rx_data()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkAuditFpgaAssertThermalSensor(mut audit: LinkPtr, idx: u32) {
    audit.cast_mut::<Audit>()[idx as usize]
        .fpga_mut()
        .assert_thermal_sensor()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkAuditFpgaDeassertThermalSensor(mut audit: LinkPtr, idx: u32) {
    audit.cast_mut::<Audit>()[idx as usize]
        .fpga_mut()
        .deassert_thermal_sensor()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaIsForceFan(audit: LinkPtr, idx: u32) -> bool {
    audit.cast::<Audit>()[idx as usize].fpga().is_force_fan()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaCurrentStmSegment(audit: LinkPtr, idx: u32) -> Segment {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .current_stm_segment()
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaCurrentModSegment(audit: LinkPtr, idx: u32) -> Segment {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .current_mod_segment()
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaIsStmGainMode(
    audit: LinkPtr,
    segment: Segment,
    idx: u32,
) -> bool {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .is_stm_gain_mode(segment.into())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaSilencerUpdateRateIntensity(
    audit: LinkPtr,
    idx: u32,
) -> u16 {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .silencer_update_rate_intensity()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaSilencerUpdateRatePhase(audit: LinkPtr, idx: u32) -> u16 {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .silencer_update_rate_phase()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaSilencerCompletionStepsIntensity(
    audit: LinkPtr,
    idx: u32,
) -> u16 {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .silencer_completion_steps_intensity()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaSilencerCompletionStepsPhase(
    audit: LinkPtr,
    idx: u32,
) -> u16 {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .silencer_completion_steps_phase()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaSilencerFixedCompletionStepsMode(
    audit: LinkPtr,
    idx: u32,
) -> bool {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .silencer_fixed_completion_steps_mode()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaDebugOutputIdx(audit: LinkPtr, idx: u32) -> u8 {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .debug_output_idx()
        .unwrap_or(0xFF)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaStmFrequencyDivision(
    audit: LinkPtr,
    segment: Segment,
    idx: u32,
) -> u32 {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .stm_frequency_division(segment.into())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaStmCycle(
    audit: LinkPtr,
    segment: Segment,
    idx: u32,
) -> u32 {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .stm_cycle(segment.into()) as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaSoundSpeed(
    audit: LinkPtr,
    segment: Segment,
    idx: u32,
) -> u32 {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .sound_speed(segment.into())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaStmLoopBehavior(
    audit: LinkPtr,
    segment: Segment,
    idx: u32,
) -> LoopBehavior {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .stm_loop_behavior(segment.into())
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaModulationFrequencyDivision(
    audit: LinkPtr,
    segment: Segment,
    idx: u32,
) -> u32 {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .modulation_frequency_division(segment.into())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaModulationCycle(
    audit: LinkPtr,
    segment: Segment,
    idx: u32,
) -> u32 {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .modulation_cycle(segment.into()) as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkAuditFpgaModulation(
    audit: LinkPtr,
    segment: Segment,
    idx: u32,
    data: *mut u8,
) {
    let m = audit.cast::<Audit>()[idx as usize]
        .fpga()
        .modulation(segment.into());
    std::ptr::copy_nonoverlapping(m.as_ptr() as _, data, m.len())
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkAuditFpgaModulationLoopBehavior(
    audit: LinkPtr,
    segment: Segment,
    idx: u32,
) -> LoopBehavior {
    audit.cast::<Audit>()[idx as usize]
        .fpga()
        .modulation_loop_behavior(segment.into())
        .into()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkAuditFpgaIDrives(
    audit: LinkPtr,
    segment: Segment,
    idx: u32,
    stm_idx: u32,
    intensities: *mut u8,
    phases: *mut u8,
) {
    let dp = audit.cast::<Audit>()[idx as usize]
        .fpga()
        .drives(segment.into(), stm_idx as _);
    let d = dp.iter().map(|v| v.intensity()).collect::<Vec<_>>();
    let p = dp.iter().map(|v| v.phase()).collect::<Vec<_>>();
    std::ptr::copy_nonoverlapping(d.as_ptr() as _, intensities, d.len());
    std::ptr::copy_nonoverlapping(p.as_ptr() as _, phases, p.len());
}

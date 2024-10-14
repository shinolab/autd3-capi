#[repr(u8)]
pub enum ProcessPriority {
    Idle = 0,
    BelowNormal = 1,
    Normal = 2,
    AboveNormal = 3,
    High = 4,
    Realtime = 5,
}

#[cfg(target_os = "windows")]
impl From<ProcessPriority> for autd3_link_soem::ProcessPriority {
    fn from(value: ProcessPriority) -> Self {
        match value {
            ProcessPriority::Idle => autd3_link_soem::ProcessPriority::Idle,
            ProcessPriority::BelowNormal => autd3_link_soem::ProcessPriority::BelowNormal,
            ProcessPriority::Normal => autd3_link_soem::ProcessPriority::Normal,
            ProcessPriority::AboveNormal => autd3_link_soem::ProcessPriority::AboveNormal,
            ProcessPriority::High => autd3_link_soem::ProcessPriority::High,
            ProcessPriority::Realtime => autd3_link_soem::ProcessPriority::Realtime,
        }
    }
}

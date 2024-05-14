#[repr(u8)]
pub enum SyncMode {
    FreeRun = 0,
    DC = 1,
}

impl From<SyncMode> for autd3_driver::ethercat::SyncMode {
    fn from(mode: SyncMode) -> Self {
        match mode {
            SyncMode::FreeRun => autd3_driver::ethercat::SyncMode::FreeRun,
            SyncMode::DC => autd3_driver::ethercat::SyncMode::DC,
        }
    }
}

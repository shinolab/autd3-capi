use autd3_driver::ethercat::ECAT_DC_SYS_TIME_BASE;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct DcSysTime {
    dc_sys_time: u64,
}

impl From<DcSysTime> for autd3_driver::ethercat::DcSysTime {
    fn from(sys_time: DcSysTime) -> Self {
        Self::from_utc(
            ECAT_DC_SYS_TIME_BASE + std::time::Duration::from_nanos(sys_time.dc_sys_time),
        )
        .unwrap()
    }
}

impl From<autd3_driver::ethercat::DcSysTime> for DcSysTime {
    fn from(sys_time: autd3_driver::ethercat::DcSysTime) -> Self {
        Self {
            dc_sys_time: sys_time.sys_time(),
        }
    }
}

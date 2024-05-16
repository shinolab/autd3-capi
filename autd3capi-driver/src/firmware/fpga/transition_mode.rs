use autd3_driver::{
    ethercat::{DcSysTime, ECAT_DC_SYS_TIME_BASE},
    firmware::fpga::*,
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TransitionModeWrap {
    ty: u8,
    value: u64,
}

impl From<TransitionModeWrap> for autd3_driver::firmware::fpga::TransitionMode {
    fn from(mode: TransitionModeWrap) -> Self {
        match mode.ty {
            TRANSITION_MODE_SYNC_IDX => autd3_driver::firmware::fpga::TransitionMode::SyncIdx,
            TRANSITION_MODE_SYS_TIME => autd3_driver::firmware::fpga::TransitionMode::SysTime(
                DcSysTime::from_utc(
                    ECAT_DC_SYS_TIME_BASE + std::time::Duration::from_nanos(mode.value),
                )
                .unwrap(),
            ),
            TRANSITION_MODE_GPIO => {
                autd3_driver::firmware::fpga::TransitionMode::GPIO(match mode.value {
                    0 => GPIOIn::I0,
                    1 => GPIOIn::I1,
                    2 => GPIOIn::I2,
                    3 => GPIOIn::I3,
                    _ => unreachable!(),
                })
            }
            TRANSITION_MODE_EXT => autd3_driver::firmware::fpga::TransitionMode::Ext,
            TRANSITION_MODE_IMMEDIATE => autd3_driver::firmware::fpga::TransitionMode::Immediate,
            _ => unreachable!(),
        }
    }
}

impl From<autd3_driver::firmware::fpga::TransitionMode> for TransitionModeWrap {
    fn from(transition_mode: autd3_driver::firmware::fpga::TransitionMode) -> Self {
        match transition_mode {
            autd3::derive::TransitionMode::SyncIdx => Self {
                ty: TRANSITION_MODE_SYNC_IDX,
                value: 0,
            },
            autd3::derive::TransitionMode::SysTime(sys_time) => Self {
                ty: TRANSITION_MODE_SYS_TIME,
                value: sys_time.sys_time(),
            },
            autd3::derive::TransitionMode::GPIO(gpio) => Self {
                ty: TRANSITION_MODE_GPIO,
                value: match gpio {
                    GPIOIn::I0 => 0,
                    GPIOIn::I1 => 1,
                    GPIOIn::I2 => 2,
                    GPIOIn::I3 => 3,
                },
            },
            autd3::derive::TransitionMode::Ext => Self {
                ty: TRANSITION_MODE_EXT,
                value: 0,
            },
            autd3::derive::TransitionMode::Immediate => Self {
                ty: TRANSITION_MODE_IMMEDIATE,
                value: 0,
            },
            _ => unreachable!(),
        }
    }
}

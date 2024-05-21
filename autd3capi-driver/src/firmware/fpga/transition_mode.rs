use autd3_driver::{
    ethercat::{DcSysTime, ECAT_DC_SYS_TIME_BASE},
    firmware::fpga::*,
};

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum TransitionModeTag {
    SyncIdx = 0,
    SysTime = 1,
    Gpio = 2,
    Ext = 3,
    Immediate = 4,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TransitionModeWrap {
    tag: TransitionModeTag,
    value: u64,
}

impl From<TransitionModeWrap> for autd3_driver::firmware::fpga::TransitionMode {
    fn from(mode: TransitionModeWrap) -> Self {
        match mode.tag {
            TransitionModeTag::SyncIdx => autd3_driver::firmware::fpga::TransitionMode::SyncIdx,
            TransitionModeTag::SysTime => autd3_driver::firmware::fpga::TransitionMode::SysTime(
                DcSysTime::from_utc(
                    ECAT_DC_SYS_TIME_BASE + std::time::Duration::from_nanos(mode.value),
                )
                .unwrap(),
            ),
            TransitionModeTag::Gpio => {
                autd3_driver::firmware::fpga::TransitionMode::GPIO(match mode.value {
                    0 => GPIOIn::I0,
                    1 => GPIOIn::I1,
                    2 => GPIOIn::I2,
                    3 => GPIOIn::I3,
                    _ => unreachable!(),
                })
            }
            TransitionModeTag::Ext => autd3_driver::firmware::fpga::TransitionMode::Ext,
            TransitionModeTag::Immediate => autd3_driver::firmware::fpga::TransitionMode::Immediate,
        }
    }
}

impl From<autd3_driver::firmware::fpga::TransitionMode> for TransitionModeWrap {
    fn from(transition_mode: autd3_driver::firmware::fpga::TransitionMode) -> Self {
        match transition_mode {
            autd3::derive::TransitionMode::SyncIdx => Self {
                tag: TransitionModeTag::SyncIdx,
                value: 0,
            },
            autd3::derive::TransitionMode::SysTime(sys_time) => Self {
                tag: TransitionModeTag::SysTime,
                value: sys_time.sys_time(),
            },
            autd3::derive::TransitionMode::GPIO(gpio) => Self {
                tag: TransitionModeTag::Gpio,
                value: match gpio {
                    GPIOIn::I0 => 0,
                    GPIOIn::I1 => 1,
                    GPIOIn::I2 => 2,
                    GPIOIn::I3 => 3,
                },
            },
            autd3::derive::TransitionMode::Ext => Self {
                tag: TransitionModeTag::Ext,
                value: 0,
            },
            autd3::derive::TransitionMode::Immediate => Self {
                tag: TransitionModeTag::Immediate,
                value: 0,
            },
            _ => unreachable!(),
        }
    }
}

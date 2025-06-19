use autd3::{core::datagram::GPIOIn, driver::ethercat::DcSysTime};

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum TransitionModeTag {
    SyncIdx = 0,
    SysTime = 1,
    Gpio = 2,
    Ext = 3,
    Immediate = 4,
    None = 0xFF,
}

#[repr(C)]
pub union TransitionModeValue {
    pub null: u64,
    pub sys_time: DcSysTime,
    pub gpio_in: GPIOIn,
}

#[repr(C)]
pub struct TransitionModeWrap {
    pub tag: TransitionModeTag,
    pub value: TransitionModeValue,
}

impl From<TransitionModeWrap> for Option<autd3::core::datagram::TransitionMode> {
    fn from(mode: TransitionModeWrap) -> Self {
        match mode.tag {
            TransitionModeTag::SyncIdx => Some(autd3::core::datagram::TransitionMode::SyncIdx),
            TransitionModeTag::SysTime => {
                Some(autd3::core::datagram::TransitionMode::SysTime(unsafe {
                    mode.value.sys_time
                }))
            }
            TransitionModeTag::Gpio => Some(autd3::core::datagram::TransitionMode::GPIO(unsafe {
                mode.value.gpio_in
            })),
            TransitionModeTag::Ext => Some(autd3::core::datagram::TransitionMode::Ext),
            TransitionModeTag::Immediate => Some(autd3::core::datagram::TransitionMode::Immediate),
            TransitionModeTag::None => None,
        }
    }
}

impl From<autd3::core::datagram::TransitionMode> for TransitionModeWrap {
    fn from(transition_mode: autd3::core::datagram::TransitionMode) -> Self {
        match transition_mode {
            autd3::core::datagram::TransitionMode::SyncIdx => Self {
                tag: TransitionModeTag::SyncIdx,
                value: TransitionModeValue { null: 0 },
            },
            autd3::core::datagram::TransitionMode::SysTime(sys_time) => Self {
                tag: TransitionModeTag::SysTime,
                value: TransitionModeValue { sys_time },
            },
            autd3::core::datagram::TransitionMode::GPIO(gpio) => Self {
                tag: TransitionModeTag::Gpio,
                value: TransitionModeValue { gpio_in: gpio },
            },
            autd3::core::datagram::TransitionMode::Ext => Self {
                tag: TransitionModeTag::Ext,
                value: TransitionModeValue { null: 0 },
            },
            autd3::core::datagram::TransitionMode::Immediate => Self {
                tag: TransitionModeTag::Immediate,
                value: TransitionModeValue { null: 0 },
            },
        }
    }
}

use autd3::{
    core::{
        datagram::{
            GPIOIn,
            internal::{HasFiniteLoop, HasSegment},
            transition_mode::{self, TransitionMode, TransitionModeParams},
        },
        sampling_config::SamplingConfig,
    },
    driver::{
        datagram::{ControlPoints, FociSTM, GainSTM},
        ethercat::DcSysTime,
    },
    gain::BoxedGain,
    modulation::BoxedModulation,
};

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum TransitionModeTag {
    Immediate = 0,
    Ext = 1,
    SyncIdx = 2,
    SysTime = 3,
    Gpio = 4,
    Later = 0xFF,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union TransitionModeValue {
    pub null: u64,
    pub sys_time: DcSysTime,
    pub gpio_in: GPIOIn,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct TransitionModeWrap {
    pub tag: TransitionModeTag,
    pub value: TransitionModeValue,
}

impl std::fmt::Debug for TransitionModeWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.tag {
            TransitionModeTag::Immediate => write!(f, "Immediate"),
            TransitionModeTag::Ext => write!(f, "Ext"),
            TransitionModeTag::SyncIdx => write!(f, "SyncIdx"),
            TransitionModeTag::SysTime => {
                write!(f, "SysTime({:?})", unsafe { self.value.sys_time })
            }
            TransitionModeTag::Gpio => write!(f, "GPIO({:?})", unsafe { self.value.gpio_in }),
            TransitionModeTag::Later => write!(f, "Later"),
        }
    }
}

impl TransitionMode for TransitionModeWrap {
    fn params(self) -> TransitionModeParams {
        match self.tag {
            TransitionModeTag::SyncIdx => transition_mode::SyncIdx.params(),
            TransitionModeTag::SysTime => {
                transition_mode::SysTime(unsafe { self.value.sys_time }).params()
            }
            TransitionModeTag::Gpio => {
                transition_mode::GPIO(unsafe { self.value.gpio_in }).params()
            }
            TransitionModeTag::Ext => transition_mode::Ext.params(),
            TransitionModeTag::Immediate => transition_mode::Immediate.params(),
            TransitionModeTag::Later => transition_mode::Later.params(),
        }
    }
}

impl HasSegment<TransitionModeWrap> for BoxedGain<'static> {}
impl HasSegment<TransitionModeWrap> for BoxedModulation {}
impl HasSegment<TransitionModeWrap> for GainSTM<Vec<BoxedGain<'static>>, SamplingConfig> {}
impl<const N: usize> HasSegment<TransitionModeWrap>
    for FociSTM<N, Vec<ControlPoints<N>>, SamplingConfig>
{
}
impl HasFiniteLoop<TransitionModeWrap> for BoxedModulation {}
impl HasFiniteLoop<TransitionModeWrap> for GainSTM<Vec<BoxedGain<'static>>, SamplingConfig> {}
impl<const N: usize> HasFiniteLoop<TransitionModeWrap>
    for FociSTM<N, Vec<ControlPoints<N>>, SamplingConfig>
{
}

// impl From<autd3::core::datagram::TransitionMode> for TransitionModeWrap {
//     fn from(transition_mode: autd3::core::datagram::TransitionMode) -> Self {
//         match transition_mode {
//             autd3::core::datagram::TransitionMode::SyncIdx => Self {
//                 tag: TransitionModeTag::SyncIdx,
//                 value: TransitionModeValue { null: 0 },
//             },
//             autd3::core::datagram::TransitionMode::SysTime(sys_time) => Self {
//                 tag: TransitionModeTag::SysTime,
//                 value: TransitionModeValue { sys_time },
//             },
//             autd3::core::datagram::TransitionMode::GPIO(gpio) => Self {
//                 tag: TransitionModeTag::Gpio,
//                 value: TransitionModeValue { gpio_in: gpio },
//             },
//             autd3::core::datagram::TransitionMode::Ext => Self {
//                 tag: TransitionModeTag::Ext,
//                 value: TransitionModeValue { null: 0 },
//             },
//             autd3::core::datagram::TransitionMode::Immediate => Self {
//                 tag: TransitionModeTag::Immediate,
//                 value: TransitionModeValue { null: 0 },
//             },
//         }
//     }
// }

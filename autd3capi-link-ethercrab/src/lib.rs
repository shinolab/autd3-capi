#![allow(clippy::missing_safety_doc)]

mod thread_builder;

use std::ffi::{CStr, CString, c_char};

use autd3capi_driver::*;

use autd3_link_ethercrab::{
    core_affinity::CoreId,
    thread_priority::{ThreadBuilder, ThreadPriority},
    *,
};

use crate::thread_builder::ThreadPriorityPtr;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDLinkEtherCrabTracingInit() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AUTDLinkEtherCrabTracingInitWithFile(path: *const c_char) -> ResultStatus {
    let path = validate_cstr!(path, AUTDStatus, ResultStatus);
    std::fs::File::options()
        .append(true)
        .create(true)
        .open(path)
        .map(|f| {
            tracing_subscriber::fmt()
                .with_writer(f)
                .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
                .with_ansi(false)
                .init();
            AUTDStatus::AUTDTrue
        })
        .into()
}

#[repr(C)]
pub struct EtherCrabOption {
    pub ifname: *const c_char,
    pub buf_size: u32,
    pub timeouts_state_transition: Duration,
    pub timeouts_pdu: Duration,
    pub timeouts_eeprom: Duration,
    pub timeouts_wait_loop_delay: Duration,
    pub timeouts_mailbox_echo: Duration,
    pub timeouts_mailbox_response: Duration,
    pub main_device_config_dc_static_sync_iterations: u32,
    pub main_device_config_retry_behaviour: u64,
    pub dc_configuration_start_delay: Duration,
    pub dc_configuration_sync0_period: Duration,
    pub dc_configuration_sync0_shift: Duration,
    pub state_check_period: Duration,
    pub sync_tolerance: Duration,
    pub sync_timeout: Duration,
    pub tx_rx_thread_builder: ThreadPriorityPtr,
    pub tx_rx_thread_affinity: i32,
    pub main_thread_builder: ThreadPriorityPtr,
    pub main_thread_affinity: i32,
}

impl TryFrom<EtherCrabOption> for EtherCrabOptionFull {
    type Error = std::str::Utf8Error;

    fn try_from(value: EtherCrabOption) -> Result<Self, Self::Error> {
        unsafe {
            let ifname = if value.ifname.is_null() {
                None
            } else {
                std::ffi::CStr::from_ptr(value.ifname)
                    .to_str()
                    .map(String::from)
                    .map(Some)?
            };
            let default = EtherCrabOptionFull::default();
            Ok(EtherCrabOptionFull {
                ifname,
                buf_size: value.buf_size as _,
                timeouts: Timeouts {
                    state_transition: value.timeouts_state_transition.into(),
                    pdu: value.timeouts_pdu.into(),
                    eeprom: value.timeouts_eeprom.into(),
                    wait_loop_delay: value.timeouts_wait_loop_delay.into(),
                    mailbox_echo: value.timeouts_mailbox_echo.into(),
                    mailbox_response: value.timeouts_mailbox_response.into(),
                },
                main_device_config: MainDeviceConfig {
                    dc_static_sync_iterations: value.main_device_config_dc_static_sync_iterations,
                    retry_behaviour: match value.main_device_config_retry_behaviour {
                        0 => RetryBehaviour::None,
                        u64::MAX => RetryBehaviour::Forever,
                        v => RetryBehaviour::Count(v as _),
                    },
                },
                dc_configuration: DcConfiguration {
                    start_delay: value.dc_configuration_start_delay.into(),
                    sync0_period: value.dc_configuration_sync0_period.into(),
                    sync0_shift: value.dc_configuration_sync0_shift.into(),
                },
                state_check_period: value.state_check_period.into(),
                sync_tolerance: value.sync_tolerance.into(),
                sync_timeout: value.sync_timeout.into(),
                tx_rx_thread_builder: if value.tx_rx_thread_builder.0.is_null() {
                    default.tx_rx_thread_builder
                } else {
                    ThreadBuilder::default()
                        .priority(*take!(value.tx_rx_thread_builder, ThreadPriority))
                },
                tx_rx_thread_affinity: if value.tx_rx_thread_affinity < 0 {
                    None
                } else {
                    Some(CoreId {
                        id: value.tx_rx_thread_affinity as _,
                    })
                },
                main_thread_builder: if value.main_thread_builder.0.is_null() {
                    default.main_thread_builder
                } else {
                    ThreadBuilder::default()
                        .priority(*take!(value.main_thread_builder, ThreadPriority))
                },
                main_thread_affinity: if value.main_thread_affinity < 0 {
                    None
                } else {
                    Some(CoreId {
                        id: value.main_thread_affinity as _,
                    })
                },
            })
        }
    }
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkEtherCrab(
    err_handler: ConstPtr,
    err_context: ConstPtr,
    option: EtherCrabOption,
) -> ResultLink {
    unsafe {
        let out_func = move |slave: usize, status: Status| {
            let (out_f, context) = {
                (
                    std::mem::transmute::<ConstPtr, unsafe extern "C" fn(ConstPtr, u32, Status)>(
                        err_handler,
                    ),
                    err_context,
                )
            };
            out_f(context, slave as _, status);
        };
        option
            .try_into()
            .map(|option: EtherCrabOptionFull| EtherCrab::new(out_func, option))
            .into()
    }
}

#[unsafe(no_mangle)]
#[must_use]
#[allow(unused_variables)]
pub unsafe extern "C" fn AUTDLinkEtherCrabIsDefault(option: EtherCrabOption) -> bool {
    option
        .try_into()
        .is_ok_and(|option: EtherCrabOptionFull| option == EtherCrabOptionFull::default())
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDLinkEtherCrabStatusGetMsg(src: Status, dst: *mut c_char) -> u32 {
    unsafe {
        let msg = format!("{src}");
        if dst.is_null() {
            return msg.len() as u32 + 1;
        }
        let c_string = CString::new(msg).unwrap();
        let c_str: &CStr = c_string.as_c_str();
        libc::strcpy(dst, c_str.as_ptr());
        0
    }
}

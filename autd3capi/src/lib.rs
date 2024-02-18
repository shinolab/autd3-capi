#![allow(clippy::missing_safety_doc)]

pub mod gain;
pub mod geometry;
pub mod link;
pub mod modulation;
pub mod stm;

use std::{collections::HashMap, ffi::c_char, time::Duration};

use autd3capi_def::{
    autd3::prelude::*, driver::datagram::ConfigureSilencerFixedCompletionSteps, *,
};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ControllerBuilderPtr(pub ConstPtr);

impl ControllerBuilderPtr {
    pub fn new(builder: SyncControllerBuilder) -> Self {
        Self(Box::into_raw(Box::new(builder)) as _)
    }
}

struct CallbackPtr(ConstPtr);
unsafe impl Send for CallbackPtr {}

#[no_mangle]
#[must_use]
#[allow(clippy::box_default)]
pub unsafe extern "C" fn AUTDControllerBuilder() -> ControllerBuilderPtr {
    ControllerBuilderPtr::new(SyncControllerBuilder::new())
}

#[no_mangle]
pub unsafe extern "C" fn AUTDControllerBuilderAddDevice(
    builder: ControllerBuilderPtr,
    x: float,
    y: float,
    z: float,
    qw: float,
    qx: float,
    qy: float,
    qz: float,
) -> ControllerBuilderPtr {
    ControllerBuilderPtr::new(take!(builder, SyncControllerBuilder).add_device(
        AUTD3::new(Vector3::new(x, y, z)).with_rotation(UnitQuaternion::from_quaternion(
            Quaternion::new(qw, qx, qy, qz),
        )),
    ))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerOpen(
    builder: ControllerBuilderPtr,
    link_builder: LinkBuilderPtr,
    timeout_ns: i64,
) -> ResultController {
    let builder = take!(builder, SyncControllerBuilder);
    let link_builder = take!(link_builder, SyncLinkBuilder);
    match timeout_ns {
        v if v < 0 => builder.open(*link_builder),
        _ => builder.open_with_timeout(*link_builder, Duration::from_nanos(timeout_ns as _)),
    }
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerClose(mut cnt: ControllerPtr) -> ResultI32 {
    cnt.close().into()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDControllerDelete(mut cnt: ControllerPtr) -> ResultI32 {
    cnt.close()
        .map(|r| {
            let _ = take!(cnt, SyncController);
            r
        })
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerFPGAState(
    mut cnt: ControllerPtr,
    out: *mut i32,
) -> ResultI32 {
    cnt.fpga_state()
        .map(|states| {
            states.iter().enumerate().for_each(|(i, state)| {
                out.add(i).write(state.map_or(-1, |s| s.state() as i32));
            });
            true
        })
        .into()
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FirmwareInfoListPtr(pub ConstPtr);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResultFirmwareInfoList {
    pub result: FirmwareInfoListPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl std::ops::Deref for FirmwareInfoListPtr {
    type Target = Vec<FirmwareInfo>;

    fn deref(&self) -> &Self::Target {
        unsafe { (self.0 as *const Self::Target).as_ref().unwrap() }
    }
}

impl From<Result<Vec<FirmwareInfo>, AUTDError>> for ResultFirmwareInfoList {
    fn from(r: Result<Vec<FirmwareInfo>, AUTDError>) -> Self {
        match r {
            Ok(v) => Self {
                result: FirmwareInfoListPtr(Box::into_raw(Box::new(v)) as _),
                err_len: 0,
                err: std::ptr::null_mut(),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: FirmwareInfoListPtr(std::ptr::null()),
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: Box::into_raw(Box::new(err)) as _,
                }
            }
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerFirmwareInfoListPointer(
    mut cnt: ControllerPtr,
) -> ResultFirmwareInfoList {
    cnt.firmware_infos().into()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDControllerFirmwareInfoGet(
    p_info_list: FirmwareInfoListPtr,
    idx: u32,
    info: *mut c_char,
) {
    let info_str = std::ffi::CString::new(p_info_list[idx as usize].to_string()).unwrap();
    libc::strcpy(info, info_str.as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn AUTDControllerFirmwareInfoListPointerDelete(
    p_info_list: FirmwareInfoListPtr,
) {
    let _ = take!(p_info_list, Vec<FirmwareInfo>);
}

#[no_mangle]
pub unsafe extern "C" fn AUTDFirmwareLatest(latest: *mut c_char) {
    let info_str = std::ffi::CString::new(FirmwareInfo::latest_version()).unwrap();
    libc::strcpy(latest, info_str.as_ptr());
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSynchronize() -> DatagramPtr {
    Synchronize::new().into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramClear() -> DatagramPtr {
    Clear::new().into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramConfigureDebugOutputIdx(
    f: ConstPtr,
    context: ConstPtr,
    geometry: GeometryPtr,
) -> DatagramPtr {
    let f = std::mem::transmute::<
        _,
        unsafe extern "C" fn(ConstPtr, geometry: GeometryPtr, u32) -> u8,
    >(f);
    DynamicConfigureDebugOutputIdx::new(
        geometry
            .devices()
            .map(move |dev| (dev.idx(), f(context, geometry, dev.idx() as u32)))
            .collect(),
    )
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramConfigureForceFan(
    f: ConstPtr,
    context: ConstPtr,
    geometry: GeometryPtr,
) -> DatagramPtr {
    let f = std::mem::transmute::<
        _,
        unsafe extern "C" fn(ConstPtr, geometry: GeometryPtr, u32) -> bool,
    >(f);
    DynamicConfigureForceFan::new(
        geometry
            .devices()
            .map(move |dev| (dev.idx(), f(context, geometry, dev.idx() as u32)))
            .collect(),
    )
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramConfigureReadsFPGAState(
    f: ConstPtr,
    context: ConstPtr,
    geometry: GeometryPtr,
) -> DatagramPtr {
    let f = std::mem::transmute::<
        _,
        unsafe extern "C" fn(ConstPtr, geometry: GeometryPtr, u32) -> bool,
    >(f);
    DynamicConfigureReadsFPGAState::new(
        geometry
            .devices()
            .map(move |dev| (dev.idx(), f(context, geometry, dev.idx() as u32)))
            .collect(),
    )
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFixedUpdateRate(
    value_intensity: u16,
    value_phase: u16,
) -> ResultDatagram {
    ConfigureSilencer::fixed_update_rate(value_intensity, value_phase).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFixedCompletionSteps(
    value_intensity: u16,
    value_phase: u16,
    strict_mode: bool,
) -> ResultDatagram {
    ConfigureSilencer::fixed_completion_steps(value_intensity, value_phase)
        .map(|s| s.with_strict_mode(strict_mode))
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramSilencerFixedCompletionStepsIsDefault(
    silencer: DatagramPtr,
) -> bool {
    let silencer = take!(silencer, ConfigureSilencerFixedCompletionSteps);
    silencer.strict_mode() == ConfigureSilencerFixedCompletionSteps::default().strict_mode()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerSend(
    mut cnt: ControllerPtr,
    d1: DatagramPtr,
    d2: DatagramPtr,
    timeout_ns: i64,
) -> ResultI32 {
    let timeout = if timeout_ns < 0 {
        None
    } else {
        Some(Duration::from_nanos(timeout_ns as _))
    };
    match (d1.is_null(), d2.is_null()) {
        (false, false) => cnt.send(DynamicDatagramPack2 {
            d1: d1.into(),
            d2: d2.into(),
            timeout,
        }),
        (false, true) => cnt.send(DynamicDatagramPack {
            d: d1.into(),
            timeout,
        }),
        (true, false) => cnt.send(DynamicDatagramPack {
            d: d2.into(),
            timeout,
        }),
        (true, true) => Err(AUTDInternalError::NotSupported("No datagram".to_owned()).into()),
    }
    .into()
}

type K = i32;
type V = (
    Box<dyn driver::operation::Operation>,
    Box<dyn driver::operation::Operation>,
    Option<Duration>,
);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResultGroupKVMap {
    pub result: GroupKVMapPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl From<Result<Box<HashMap<K, V>>, AUTDInternalError>> for ResultGroupKVMap {
    fn from(value: Result<Box<HashMap<K, V>>, AUTDInternalError>) -> Self {
        match value {
            Ok(map) => Self {
                result: GroupKVMapPtr(Box::into_raw(map) as _),
                err_len: 0,
                err: std::ptr::null_mut(),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: GroupKVMapPtr(std::ptr::null()),
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: Box::into_raw(Box::new(err)) as _,
                }
            }
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerGroupCreateKVMap() -> GroupKVMapPtr {
    GroupKVMapPtr(Box::into_raw(Box::<HashMap<K, V>>::default()) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerGroupKVMapSet(
    map: GroupKVMapPtr,
    key: i32,
    d1: DatagramPtr,
    d2: DatagramPtr,
    timeout_ns: i64,
) -> ResultGroupKVMap {
    let mut map = take!(map, HashMap<K, V>);
    let timeout = if timeout_ns < 0 {
        None
    } else {
        Some(Duration::from_nanos(timeout_ns as _))
    };
    match (d1.is_null(), d2.is_null()) {
        (false, false) => DynamicDatagramPack2 {
            d1: d1.into(),
            d2: d2.into(),
            timeout,
        }
        .operation(),
        (false, true) => DynamicDatagramPack {
            d: d1.into(),
            timeout,
        }
        .operation(),
        (true, false) => DynamicDatagramPack {
            d: d2.into(),
            timeout,
        }
        .operation(),
        (true, true) => Err(AUTDInternalError::NotSupported("No datagram".to_owned())),
    }
    .map(|op| {
        map.insert(key, (op.0, op.1, timeout));
        map
    })
    .into()
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GroupKVMapPtr(pub ConstPtr);

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerGroup(
    mut cnt: ControllerPtr,
    map: *const i32,
    kv_map: GroupKVMapPtr,
) -> ResultI32 {
    take!(kv_map, HashMap<K, V>)
        .into_iter()
        .try_fold(
            cnt.group(|dev| {
                let k = map.add(dev.idx()).read();
                if k < 0 {
                    None
                } else {
                    Some(k)
                }
            }),
            |acc, (k, (op1, op2, timeout))| acc.set_boxed_op(k, op1, op2, timeout),
        )
        .and_then(|group| group.send())
        .into()
}

#[cfg(test)]
mod tests {
    use super::{
        geometry::AUTDGeometry,
        link::{audit::*, AUTDLinkGet},
        *,
    };

    pub unsafe fn create_controller() -> ControllerPtr {
        let builder = AUTDControllerBuilder();
        let builder = AUTDControllerBuilderAddDevice(builder, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let builder = AUTDControllerBuilderAddDevice(builder, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let link = AUTDLinkAuditIntoBuilder(AUTDLinkAudit());
        let result = AUTDControllerOpen(builder, link, -1);
        assert_ne!(result.result.0, std::ptr::null());
        result.result
    }

    unsafe extern "C" fn debug_0(_ptr: ConstPtr, _geometry: GeometryPtr, _idx: u32) -> u8 {
        0
    }

    unsafe extern "C" fn debug_1(_ptr: ConstPtr, _geometry: GeometryPtr, idx: u32) -> u8 {
        if idx == 0 {
            10
        } else {
            0xFF
        }
    }

    #[test]
    fn test_debug_output_idx() {
        unsafe {
            let cnt = create_controller();
            let geometry = AUTDGeometry(cnt);

            let audit = AUTDLinkGet(cnt);
            for i in 0..2 {
                assert_eq!(AUTDLinkAuditFpgaDebugOutputIdx(audit, i), 0xFF);
            }

            let d1 = AUTDDatagramConfigureDebugOutputIdx(debug_0 as _, std::ptr::null(), geometry);
            let res = AUTDControllerSend(cnt, d1, DatagramPtr(std::ptr::null()), 200 * 1000 * 1000);
            assert_eq!(res.result, AUTD3_TRUE);
            for i in 0..2 {
                assert_eq!(AUTDLinkAuditFpgaDebugOutputIdx(audit, i), 0);
            }

            let d1 = AUTDDatagramConfigureDebugOutputIdx(debug_1 as _, std::ptr::null(), geometry);
            let res = AUTDControllerSend(cnt, d1, DatagramPtr(std::ptr::null()), 200 * 1000 * 1000);
            assert_eq!(res.result, AUTD3_TRUE);
            assert_eq!(AUTDLinkAuditFpgaDebugOutputIdx(audit, 0), 10);
            assert_eq!(AUTDLinkAuditFpgaDebugOutputIdx(audit, 1), 0xFF);
        }
    }
}

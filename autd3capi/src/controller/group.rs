use std::{collections::HashMap, fmt::Debug, hash::Hash, time::Duration};

use autd3capi_driver::{
    autd3::{
        controller::GroupGuard,
        derive::{Datagram, Device, Operation},
    },
    driver::error::AUTDInternalError,
    take, tokio, ConstPtr, DatagramPtr, DynamicDatagramPack, DynamicDatagramPack2, ResultI32,
    SyncLink,
};

use super::{ControllerPtr, SyncController};

#[allow(clippy::type_complexity)]
pub struct SyncGroupGuard<'a, K: Hash + Eq + Clone + Debug, F: Fn(&Device) -> Option<K>> {
    handle: tokio::runtime::Handle,
    inner: GroupGuard<'a, K, SyncLink, F>,
}

impl<'a, K: Hash + Eq + Clone + Debug, F: Fn(&Device) -> Option<K>> SyncGroupGuard<'a, K, F> {
    pub fn set_boxed_op(
        self,
        k: K,
        op1: Box<dyn Operation>,
        op2: Box<dyn Operation>,
        timeout: Option<Duration>,
    ) -> Self {
        Self {
            handle: self.handle,
            inner: self.inner.set_boxed_op(k, op1, op2, timeout),
        }
    }

    pub fn send(self) -> Result<bool, AUTDInternalError> {
        self.handle.block_on(self.inner.send())
    }
}

impl SyncController {
    pub fn group<K: Hash + Eq + Clone + Debug, F: Fn(&Device) -> Option<K>>(
        &mut self,
        f: F,
    ) -> SyncGroupGuard<K, F> {
        SyncGroupGuard {
            handle: self.inner.link.handle.clone(),
            inner: self.inner.group(f),
        }
    }
}

type K = i32;
type V = (Box<dyn Operation>, Box<dyn Operation>, Option<Duration>);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GroupKVMapPtr(pub ConstPtr);

impl std::ops::Deref for GroupKVMapPtr {
    type Target = HashMap<K, V>;
    fn deref(&self) -> &Self::Target {
        unsafe { (self.0 as *const HashMap<K, V>).as_ref().unwrap() }
    }
}

impl std::ops::DerefMut for GroupKVMapPtr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { (self.0 as *mut HashMap<K, V>).as_mut().unwrap() }
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
    mut map: GroupKVMapPtr,
    key: i32,
    d1: DatagramPtr,
    d2: DatagramPtr,
    timeout_ns: i64,
) {
    let timeout = if timeout_ns < 0 {
        None
    } else {
        Some(Duration::from_nanos(timeout_ns as _))
    };
    let op = match (d1.is_null(), d2.is_null()) {
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
        _ => unreachable!(),
    };
    map.insert(key, (op.0, op.1, timeout));
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerGroup(
    mut cnt: ControllerPtr,
    map: *const i32,
    kv_map: GroupKVMapPtr,
) -> ResultI32 {
    take!(kv_map, HashMap<K, V>)
        .into_iter()
        .fold(
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
        .send()
        .into()
}

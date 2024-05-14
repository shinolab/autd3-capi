use std::collections::HashMap;

use autd3capi_driver::{autd3::prelude::Group, driver::autd3_device::AUTD3, *};

type M = HashMap<usize, Vec<i32>>;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GroupGainMapPtr(pub ConstPtr);

#[no_mangle]
#[must_use]
#[allow(clippy::uninit_vec)]
pub unsafe extern "C" fn AUTDGainGroupCreateMap(
    device_indices_ptr: *const u32,
    num_devices: u32,
) -> GroupGainMapPtr {
    GroupGainMapPtr(Box::into_raw(Box::new(
        (0..num_devices as usize)
            .map(|i| {
                let mut v = Vec::with_capacity(AUTD3::NUM_TRANS_IN_UNIT);
                v.set_len(AUTD3::NUM_TRANS_IN_UNIT);
                (device_indices_ptr.add(i).read() as _, v)
            })
            .collect::<M>(),
    )) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainGroupMapSet(
    map: GroupGainMapPtr,
    dev_idx: u32,
    map_data: *const i32,
) -> GroupGainMapPtr {
    let dev_idx = dev_idx as usize;
    let map = {
        let mut map = take!(map, M);
        std::ptr::copy_nonoverlapping(
            map_data,
            map.get_mut(&dev_idx).unwrap().as_mut_ptr(),
            map[&dev_idx].len(),
        );
        map
    };
    GroupGainMapPtr(Box::into_raw(map) as _)
}

#[no_mangle]
#[must_use]
#[allow(clippy::uninit_vec)]
pub unsafe extern "C" fn AUTDGainGroup(
    map: GroupGainMapPtr,
    keys_ptr: *const i32,
    values_ptr: *const GainPtr,
    kv_len: u32,
) -> GainPtr {
    let map = take!(map, M);
    vec_from_raw!(keys_ptr, i32, kv_len)
        .iter()
        .zip(vec_from_raw!(values_ptr, GainPtr, kv_len).iter())
        .fold(
            Group::new(move |dev, tr| {
                let key = map[&dev.idx()][tr.idx()];
                if key < 0 {
                    None
                } else {
                    Some(key)
                }
            }),
            |acc, (&k, v)| acc.set(k, *take!(v, Box<G>)),
        )
        .into()
}

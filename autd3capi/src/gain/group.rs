use std::{collections::HashMap, sync::Arc};

use autd3::gain::Group;
use autd3capi_driver::{driver::autd3_device::AUTD3, *};
use driver::{
    datagram::BoxedGain,
    geometry::{Device, Transducer},
};

type M = HashMap<usize, Vec<i32>>;

#[repr(C)]
pub struct GroupGainMapPtr(pub *const libc::c_void);

#[unsafe(no_mangle)]
#[must_use]
#[allow(clippy::uninit_vec)]
pub unsafe extern "C" fn AUTDGainGroupCreateMap(
    device_indices_ptr: *const u16,
    num_devices: u16,
) -> GroupGainMapPtr {
    unsafe {
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
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGainGroupMapSet(
    map: GroupGainMapPtr,
    dev_idx: u16,
    map_data: *const i32,
) -> GroupGainMapPtr {
    let dev_idx = dev_idx as usize;
    let map = unsafe {
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

#[unsafe(no_mangle)]
#[must_use]
#[allow(clippy::uninit_vec)]
pub unsafe extern "C" fn AUTDGainGroup(
    map: GroupGainMapPtr,
    keys_ptr: *const i32,
    values_ptr: *const GainPtr,
    kv_len: u32,
) -> GainPtr {
    let key_map = unsafe {
        let map: HashMap<_, _> = take!(map, M)
            .into_iter()
            .map(|(k, v)| (k, Arc::new(v)))
            .collect();
        move |dev: &Device| {
            let map = map[&dev.idx()].clone();
            move |tr: &Transducer| {
                let key = map[tr.idx()];
                if key < 0 { None } else { Some(key) }
            }
        }
    };
    let gain_map = (0..kv_len as usize)
        .map(|i| unsafe {
            (
                keys_ptr.add(i).read(),
                *take!(values_ptr.add(i).read(), BoxedGain),
            )
        })
        .collect();
    Group { key_map, gain_map }.into()
}

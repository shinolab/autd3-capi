use autd3capi_driver::{
    autd3::derive::Device, ConstPtr, ContextPtr, DatagramPtr, DynamicDatagramPack, GeometryPtr,
    ResultI32,
};

use super::ControllerPtr;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerGroup(
    mut cnt: ControllerPtr,
    f: ConstPtr,
    context: ContextPtr,
    geometry: GeometryPtr,
    keys: *const i32,
    d: *const DatagramPtr,
    n: u16,
) -> ResultI32 {
    let runtime = cnt.runtime.handle().clone();
    let f = std::mem::transmute::<_, unsafe extern "C" fn(ContextPtr, GeometryPtr, u16) -> i32>(f);
    (0..n)
        .try_fold(
            cnt.inner.group(Box::new(move |dev: &Device| {
                match f(context, geometry, dev.idx() as _) {
                    k if k >= 0 => Some(k),
                    _ => None,
                }
            }) as Box<_>),
            |acc, i| {
                let k = keys.add(i as _).read();
                let d = d.add(i as _).read();
                acc.set(k, DynamicDatagramPack { d: d.into() })
            },
        )
        .map(|g| runtime.block_on(g.send()))
        .into()
}

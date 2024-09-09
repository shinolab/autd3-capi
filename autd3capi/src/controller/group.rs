use autd3capi_driver::{
    async_ffi::LocalFfiFuture, autd3::driver::geometry::Device, ConstPtr, DatagramPtr,
    DynamicDatagramPack, GeometryPtr, ResultI32,
};

use super::ControllerPtr;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDControllerGroup(
    mut cnt: ControllerPtr,
    f: ConstPtr,
    context: ConstPtr,
    geometry: GeometryPtr,
    keys: *const i32,
    d: *const DatagramPtr,
    n: u16,
) -> LocalFfiFuture<ResultI32> {
    let f =
        std::mem::transmute::<ConstPtr, unsafe extern "C" fn(ConstPtr, GeometryPtr, u16) -> i32>(f);
    LocalFfiFuture::new(async move {
        let r: ResultI32 = match (0..n).try_fold(
            cnt.group(Box::new(
                move |dev: &Device| match f(context, geometry, dev.idx() as _) {
                    k if k >= 0 => Some(k),
                    _ => None,
                },
            ) as Box<_>),
            |acc, i| {
                let k = keys.add(i as _).read();
                let d = d.add(i as _).read();
                acc.set(k, DynamicDatagramPack { d: d.into() })
            },
        ) {
            Ok(g) => g.send().await.into(),
            Err(e) => e.into(),
        };
        r
    })
}

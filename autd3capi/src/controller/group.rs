use autd3capi_driver::{
    async_ffi::LocalFfiFuture,
    autd3::{driver::geometry::Device, error::AUTDError},
    ConstPtr, DatagramPtr, DynamicDatagram, GeometryPtr, ResultStatus,
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
) -> LocalFfiFuture<ResultStatus> {
    let f =
        std::mem::transmute::<ConstPtr, unsafe extern "C" fn(ConstPtr, GeometryPtr, u16) -> i32>(f);
    LocalFfiFuture::new(async move {
        match (0..n).try_fold(
            cnt.group(Box::new(
                move |dev: &Device| match f(context, geometry, dev.idx() as _) {
                    k if k >= 0 => Some(k),
                    _ => None,
                },
            ) as Box<_>),
            |acc, i| {
                let k = keys.add(i as _).read();
                let d = d.add(i as _).read();
                acc.set(k, *Box::<DynamicDatagram>::from(d))
            },
        ) {
            Ok(g) => g.send().await,
            Err(e) => Err(AUTDError::Internal(e)),
        }
        .into()
    })
}

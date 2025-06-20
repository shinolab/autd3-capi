use autd3capi_driver::{
    ConstPtr, DatagramPtr, GeometryPtr,
    driver::{firmware::driver::BoxedDatagram, geometry::Device},
};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDDatagramGroup(
    f: ConstPtr,
    context: ConstPtr,
    geometry: GeometryPtr,
    keys: *const i32,
    d: *const DatagramPtr,
    n: u16,
) -> DatagramPtr {
    unsafe {
        let map = (0..n)
            .map(|i| {
                let k = keys.add(i as _).read();
                let d = d.add(i as _).read();
                (k, *Box::<BoxedDatagram>::from(d))
            })
            .collect();
        let f = std::mem::transmute::<
            ConstPtr,
            unsafe extern "C" fn(ConstPtr, GeometryPtr, u16) -> i32,
        >(f);
        autd3capi_driver::autd3::prelude::Group {
            key_map: move |dev: &Device| match f(context, geometry, dev.idx() as _) {
                k if k >= 0 => Some(k),
                _ => None,
            },
            datagram_map: map,
        }
    }
    .into()
}

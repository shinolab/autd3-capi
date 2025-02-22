use autd3capi_driver::{
    ConstPtr, DatagramPtr, GeometryPtr, ResultStatus, SenderPtr,
    autd3::{
        controller::{Sender, Sleep},
        core::link::Link,
        driver::geometry::Device,
    },
    driver::datagram::BoxedDatagram,
};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDControllerGroup(
    sender: SenderPtr,
    f: ConstPtr,
    context: ConstPtr,
    geometry: GeometryPtr,
    keys: *const i32,
    d: *const DatagramPtr,
    n: u16,
) -> ResultStatus {
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
        (sender.0 as *mut Sender<'static, Box<dyn Link>, Box<dyn Sleep>>)
            .as_mut()
            .unwrap()
            .group_send(
                move |dev: &Device| match f(context, geometry, dev.idx() as _) {
                    k if k >= 0 => Some(k),
                    _ => None,
                },
                map,
            )
            .into()
    }
}

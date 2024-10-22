use autd3_driver::datagram::{BoxedGain, IntoBoxedGain};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct GainPtr(pub *const libc::c_void);

impl<T: IntoBoxedGain + 'static> From<T> for GainPtr {
    fn from(g: T) -> Self {
        let g: Box<BoxedGain> = Box::new(g.into_boxed());
        Self(Box::into_raw(g) as _)
    }
}

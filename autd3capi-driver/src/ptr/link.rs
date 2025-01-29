use autd3_core::link::Link;

use crate::{impl_result, ConstPtr};

#[repr(C)]
pub struct LinkPtr(pub *const libc::c_void);

impl<L: Link + 'static> From<L> for LinkPtr {
    fn from(v: L) -> Self {
        let v: Box<dyn Link> = Box::new(v);
        Self(Box::into_raw(Box::new(v)) as _)
    }
}

impl LinkPtr {
    pub const NULL: Self = Self(std::ptr::null());

    pub fn cast<T: Link>(&self) -> &T {
        unsafe {
            (self.0 as *const Box<dyn Link> as *const Box<T>)
                .as_ref()
                .unwrap()
        }
    }

    pub fn cast_mut<T: Link>(&mut self) -> &mut T {
        unsafe {
            (self.0 as *mut Box<dyn Link> as *mut Box<T>)
                .as_mut()
                .unwrap()
        }
    }
}

#[repr(C)]
pub struct ResultLink {
    pub result: LinkPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl_result!(ResultLink, LinkPtr);

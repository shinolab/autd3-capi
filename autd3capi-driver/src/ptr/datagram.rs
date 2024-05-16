use crate::{take, ConstPtr, DynamicDatagram};

#[repr(C)]
pub struct DatagramPtr(pub ConstPtr);

impl DatagramPtr {
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    pub const NULL: Self = Self(std::ptr::null());
}

impl From<DatagramPtr> for Box<Box<dyn DynamicDatagram>> {
    fn from(value: DatagramPtr) -> Self {
        unsafe { take!(value, Box<dyn DynamicDatagram>) }
    }
}

impl<T: DynamicDatagram> From<T> for DatagramPtr {
    fn from(d: T) -> Self {
        let d: Box<Box<dyn DynamicDatagram>> = Box::new(Box::new(d));
        Self(Box::into_raw(d) as _)
    }
}

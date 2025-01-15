use autd3_core::link::{Link, LinkBuilder, LinkError};
use autd3_driver::geometry::Geometry;

use crate::impl_ptr;

use crate::{impl_result, ConstPtr};

#[repr(C)]
pub struct LinkBuilderPtr(pub *const libc::c_void);

impl_ptr!(LinkBuilderPtr);

#[repr(C)]
pub struct LinkPtr(pub *const libc::c_void);

impl_ptr!(LinkPtr, Box<dyn Link>);

impl LinkPtr {
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

pub struct DynamicLinkBuilder {
    #[allow(clippy::type_complexity)]
    pub gen: Box<dyn FnOnce(&Geometry) -> Result<Box<dyn Link>, LinkError>>,
}

unsafe impl Send for DynamicLinkBuilder {}
unsafe impl Sync for DynamicLinkBuilder {}

impl DynamicLinkBuilder {
    pub fn new<B: LinkBuilder + 'static>(builder: B) -> Self
    where
        B::L: Link,
    {
        Self {
            gen: Box::new(move |geometry| {
                let link = builder.open(geometry)?;
                Ok(Box::new(link))
            }),
        }
    }
}

impl<B: LinkBuilder + 'static> From<B> for LinkBuilderPtr
where
    B::L: Link,
{
    fn from(value: B) -> Self {
        LinkBuilderPtr(Box::into_raw(Box::new(DynamicLinkBuilder::new(value))) as _)
    }
}

impl LinkBuilder for DynamicLinkBuilder {
    type L = Box<dyn Link>;

    fn open(self, geometry: &Geometry) -> Result<Self::L, LinkError> {
        (self.gen)(geometry)
    }
}

#[repr(C)]
pub struct ResultLinkBuilder {
    pub result: LinkBuilderPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl_result!(ResultLinkBuilder, LinkBuilderPtr);

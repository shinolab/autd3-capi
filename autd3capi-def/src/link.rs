use std::time::Duration;

use autd3_driver::{
    cpu::{RxMessage, TxDatagram},
    error::AUTDInternalError,
    geometry::Geometry,
    link::{Link, LinkBuilder},
};

use crate::{impl_ptr, ConstPtr};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct LinkBuilderPtr(pub ConstPtr);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct LinkPtr(pub ConstPtr);

impl_ptr!(LinkPtr, SyncLink);

impl LinkPtr {
    pub fn cast<L: Link>(&self) -> &L {
        unsafe {
            (&self.inner as *const Box<dyn Link> as *const Box<L>)
                .as_ref()
                .unwrap()
        }
    }

    pub fn cast_mut<L: Link>(&mut self) -> &mut L {
        unsafe {
            (&mut self.inner as *mut Box<dyn Link> as *mut Box<L>)
                .as_mut()
                .unwrap()
        }
    }
}

pub struct SyncLink {
    pub(crate) handle: tokio::runtime::Handle,
    pub inner: Box<dyn Link>,
}

impl SyncLink {
    pub fn runtime(&self) -> tokio::runtime::Handle {
        self.handle.clone()
    }
}

pub struct SyncLinkBuilder {
    pub(crate) runtime: Option<tokio::runtime::Runtime>,
    #[allow(clippy::type_complexity)]
    gen: Box<dyn FnOnce(&Geometry) -> Result<SyncLink, AUTDInternalError>>,
}

unsafe impl Send for SyncLinkBuilder {}
unsafe impl Sync for SyncLinkBuilder {}

impl SyncLinkBuilder {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<B: LinkBuilder + 'static>(builder: B) -> LinkBuilderPtr
    where
        B::L: Link,
    {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        let handle = runtime.handle().clone();
        LinkBuilderPtr(Box::into_raw(Box::new(Self {
            runtime: Some(runtime),
            gen: Box::new(move |geometry| {
                match tokio::task::block_in_place(|| handle.block_on(builder.open(geometry))) {
                    Ok(v) => Ok(SyncLink {
                        handle,
                        inner: Box::new(v),
                    }),
                    Err(e) => Err(e),
                }
            }),
        })) as _)
    }
}

#[autd3_driver::async_trait]
impl LinkBuilder for SyncLinkBuilder {
    type L = SyncLink;

    async fn open(self, geometry: &Geometry) -> Result<Self::L, AUTDInternalError> {
        (self.gen)(geometry)
    }
}

#[autd3_driver::async_trait]
impl Link for SyncLink {
    async fn close(&mut self) -> Result<(), AUTDInternalError> {
        self.inner.close().await
    }

    async fn send(&mut self, tx: &TxDatagram) -> Result<bool, AUTDInternalError> {
        self.inner.send(tx).await
    }

    async fn receive(&mut self, rx: &mut [RxMessage]) -> Result<bool, AUTDInternalError> {
        self.inner.receive(rx).await
    }

    fn is_open(&self) -> bool {
        self.inner.is_open()
    }

    fn timeout(&self) -> Duration {
        self.inner.timeout()
    }

    fn trace(&mut self, tx: &TxDatagram, rx: &mut [RxMessage], timeout: Option<Duration>) {
        self.inner.trace(tx, rx, timeout)
    }
}

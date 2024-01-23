use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use autd3_driver::{
    cpu::{RxMessage, TxDatagram},
    error::AUTDInternalError,
    geometry::Geometry,
    link::{Link, LinkBuilder},
};

use crate::ConstPtr;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct LinkBuilderPtr(pub ConstPtr);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct LinkPtr(pub ConstPtr);

impl LinkPtr {
    pub fn cast<L: Link>(&self) -> &Box<L> {
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

impl Deref for LinkPtr {
    type Target = SyncLink;

    fn deref(&self) -> &Self::Target {
        unsafe { (self.0 as *const SyncLink).as_ref().unwrap() }
    }
}

impl DerefMut for LinkPtr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { (self.0 as *mut SyncLink).as_mut().unwrap() }
    }
}

pub struct SyncLink {
    runtime: tokio::runtime::Runtime,
    pub inner: Box<dyn Link>,
}

impl SyncLink {
    pub fn runtime(&self) -> tokio::runtime::Handle {
        self.runtime.handle().clone()
    }
}

pub struct SyncLinkBuilder {
    runtime: tokio::runtime::Handle,
    #[allow(clippy::type_complexity)]
    gen: Box<dyn FnOnce(&Geometry) -> Result<SyncLink, AUTDInternalError>>,
}

unsafe impl Send for SyncLinkBuilder {}
unsafe impl Sync for SyncLinkBuilder {}

impl SyncLinkBuilder {
    pub fn new<B: LinkBuilder + 'static>(builder: B) -> LinkBuilderPtr
    where
        B::L: Link,
    {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        LinkBuilderPtr(Box::into_raw(Box::new(Self {
            runtime: runtime.handle().clone(),
            gen: Box::new(move |geometry| {
                tokio::task::block_in_place(|| runtime.block_on(builder.open(geometry))).map(|l| {
                    SyncLink {
                        runtime,
                        inner: Box::new(l),
                    }
                })
            }),
        })) as _)
    }

    pub fn runtime(&self) -> tokio::runtime::Handle {
        self.runtime.clone()
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
}

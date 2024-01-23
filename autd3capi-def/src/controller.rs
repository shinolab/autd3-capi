use std::{hash::Hash, time::Duration};

use autd3::{
    controller::{ControllerBuilder, GroupGuard},
    error::AUTDError,
    Controller,
};
use autd3_driver::{
    derive::Datagram,
    error::AUTDInternalError,
    firmware_version::FirmwareInfo,
    fpga::FPGAState,
    geometry::{Device, IntoDevice},
};

use crate::{
    link::{SyncLink, SyncLinkBuilder},
    ConstPtr,
};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ControllerPtr(pub ConstPtr);

impl std::ops::Deref for ControllerPtr {
    type Target = SyncController;
    fn deref(&self) -> &Self::Target {
        unsafe { (self.0 as *const SyncController).as_ref().unwrap() }
    }
}

impl std::ops::DerefMut for ControllerPtr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { (self.0 as *mut SyncController).as_mut().unwrap() }
    }
}

pub struct SyncController {
    pub inner: Controller<SyncLink>,
}

impl SyncController {
    pub fn send<S: Datagram>(&mut self, s: S) -> Result<bool, AUTDError> {
        self.inner.link.runtime().block_on(self.inner.send(s))
    }

    pub fn close(&mut self) -> Result<bool, AUTDError> {
        self.inner.link.runtime().block_on(self.inner.close())
    }

    pub fn firmware_infos(&mut self) -> Result<Vec<FirmwareInfo>, AUTDError> {
        self.inner
            .link
            .runtime()
            .block_on(self.inner.firmware_infos())
    }

    pub fn fpga_state(&mut self) -> Result<Vec<Option<FPGAState>>, AUTDError> {
        self.inner.link.runtime().block_on(self.inner.fpga_state())
    }
}

pub struct SyncControllerBuilder {
    inner: ControllerBuilder,
}

impl SyncControllerBuilder {
    pub const fn new() -> Self {
        Self {
            inner: Controller::builder(),
        }
    }

    pub fn add_device<D: IntoDevice>(self, dev: D) -> Self {
        Self {
            inner: self.inner.add_device(dev),
        }
    }

    pub fn open_with(self, link_builder: SyncLinkBuilder) -> Result<SyncController, AUTDError> {
        Ok(SyncController {
            inner: link_builder
                .runtime()
                .block_on(self.inner.open_with(link_builder))?,
        })
    }
}

#[allow(clippy::type_complexity)]
pub struct SyncGroupGuard<'a, K: Hash + Eq + Clone, F: Fn(&Device) -> Option<K>> {
    handle: tokio::runtime::Handle,
    inner: GroupGuard<'a, K, SyncLink, F>,
}

impl<'a, K: Hash + Eq + Clone, F: Fn(&Device) -> Option<K>> SyncGroupGuard<'a, K, F> {
    pub fn set_boxed_op(
        self,
        k: K,
        op1: Box<dyn autd3_driver::operation::Operation>,
        op2: Box<dyn autd3_driver::operation::Operation>,
        timeout: Option<Duration>,
    ) -> Result<Self, AUTDInternalError> {
        Ok(Self {
            handle: self.handle,
            inner: self.inner.set_boxed_op(k, op1, op2, timeout)?,
        })
    }

    pub fn send(self) -> Result<bool, AUTDInternalError> {
        self.handle.block_on(self.inner.send())
    }
}

impl SyncController {
    pub fn group<K: Hash + Eq + Clone, F: Fn(&Device) -> Option<K>>(
        &mut self,
        f: F,
    ) -> SyncGroupGuard<K, F> {
        SyncGroupGuard {
            handle: self.inner.link.runtime().clone(),
            inner: self.inner.group(f),
        }
    }
}

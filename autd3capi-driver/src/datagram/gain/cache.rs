use std::{collections::HashMap, sync::Mutex};

use crate::autd3::{derive::*, prelude::Drive};

use autd3_driver::datagram::BoxedGain;
use derive_more::Debug;

#[derive(Gain, Debug, Clone)]
pub struct BoxedCache {
    pub gain: Arc<Mutex<Option<BoxedGain>>>,
    #[debug("{}", !self.cache.lock().unwrap().is_empty())]
    pub cache: Arc<Mutex<HashMap<usize, Arc<Vec<Drive>>>>>,
}

impl BoxedCache {
    pub fn new(gain: BoxedGain) -> Self {
        Self {
            gain: Arc::new(Mutex::new(Some(gain))),
            cache: Default::default(),
        }
    }

    pub fn init(&self, geometry: &Geometry) -> Result<(), AUTDInternalError> {
        dbg!("init");
        if let Some(gain) = self.gain.lock().unwrap().take() {
            let mut f = gain.init(geometry)?;
            geometry
                .devices()
                .filter(|dev| !self.cache.lock().unwrap().contains_key(&dev.idx()))
                .for_each(|dev| {
                    tracing::debug!("Initializing cache for device {}", dev.idx());
                    let f = f.generate(dev);
                    self.cache.lock().unwrap().insert(
                        dev.idx(),
                        Arc::new(dev.iter().map(|tr| f.calc(tr)).collect()),
                    );
                });
        }

        if self.cache.lock().unwrap().len() != geometry.devices().count()
            || geometry
                .devices()
                .any(|dev| !self.cache.lock().unwrap().contains_key(&dev.idx()))
        {
            return Err(AUTDInternalError::GainError(
                "Cache is initialized with different geometry".to_string(),
            ));
        }

        Ok(())
    }
}

pub struct Context {
    g: Arc<Vec<Drive>>,
}

impl GainContext for Context {
    fn calc(&self, tr: &Transducer) -> Drive {
        self.g[tr.idx()]
    }
}

impl GainContextGenerator for BoxedCache {
    type Context = Context;

    fn generate(&mut self, device: &Device) -> Self::Context {
        Context {
            g: self.cache.lock().unwrap()[&device.idx()].clone(),
        }
    }
}

impl Gain for BoxedCache {
    type G = Self;

    fn init_with_filter(
        self,
        geometry: &Geometry,
        _filter: Option<HashMap<usize, BitVec<u32>>>,
    ) -> Result<Self::G, AUTDInternalError> {
        dbg!('a');
        BoxedCache::init(&self, geometry)?;
        dbg!('b');
        Ok(self)
    }
}

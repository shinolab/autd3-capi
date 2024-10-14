use std::{collections::HashMap, sync::Mutex};

use crate::{
    autd3::{derive::*, prelude::Drive},
    G,
};

use derive_more::{Debug, Deref};

#[derive(Gain, Deref, Debug, Clone)]
pub struct BoxedCache {
    #[deref]
    pub gain: Arc<G>,
    #[debug("{}", !self.cache.lock().unwrap().is_empty())]
    cache: Arc<Mutex<HashMap<usize, Arc<Vec<Drive>>>>>,
}

impl BoxedCache {
    pub fn new(gain: Box<G>) -> Self {
        Self {
            gain: Arc::from(gain),
            cache: Default::default(),
        }
    }
}

impl Gain for BoxedCache {
    fn calc(&self, geometry: &Geometry) -> Result<GainCalcFn, AUTDInternalError> {
        if self.cache.lock().unwrap().len() != geometry.devices().count()
            || geometry
                .devices()
                .any(|dev| !self.cache.lock().unwrap().contains_key(&dev.idx()))
        {
            let mut f = self.gain.calc(geometry)?;
            geometry
                .devices()
                .filter(|dev| !self.cache.lock().unwrap().contains_key(&dev.idx()))
                .for_each(|dev| {
                    tracing::debug!("Initialize cache for device {}", dev.idx());
                    self.cache
                        .lock()
                        .unwrap()
                        .insert(dev.idx(), Arc::new(dev.iter().map(f(dev)).collect()));
                });
        }

        let cache = self.cache.lock().unwrap();
        Ok(Box::new(move |dev| {
            let drives = cache[&dev.idx()].clone();
            Box::new(move |tr| drives[tr.idx()])
        }))
    }
}

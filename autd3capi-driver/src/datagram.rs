use std::{num::NonZeroU16, time::Duration};

use autd3::derive::{AUTDInternalError, Geometry, Segment};
use autd3_driver::{
    datagram::Datagram,
    firmware::operation::{Operation, OperationGenerator},
    geometry::Device,
};

use derive_more::Debug;

use crate::{G, M};

pub struct DynamicOperationGenerator<O1: Operation + 'static, O2: Operation + 'static> {
    pub g: Box<dyn OperationGenerator<O1 = O1, O2 = O2>>,
}

impl<O1: Operation + 'static, O2: Operation + 'static> OperationGenerator
    for DynamicOperationGenerator<O1, O2>
{
    type O1 = Box<dyn Operation>;
    type O2 = Box<dyn Operation>;

    fn generate(&self, device: &Device) -> (Self::O1, Self::O2) {
        let (o1, o2) = self.g.generate(device);
        (Box::new(o1), Box::new(o2))
    }
}

pub trait DatagramDefault {
    fn default() -> Self;
}

pub trait DynamicDatagram: std::fmt::Debug {
    #[allow(clippy::type_complexity)]
    fn operation_generator(
        &mut self,
        geometry: &Geometry,
    ) -> Result<
        Box<dyn OperationGenerator<O1 = Box<dyn Operation>, O2 = Box<dyn Operation>>>,
        AUTDInternalError,
    >;
    fn timeout(&self) -> Option<Duration>;
    fn parallel_threshold(&self) -> Option<usize>;
}

impl<G: OperationGenerator + 'static, D: Datagram<G = G> + DatagramDefault> DynamicDatagram for D {
    fn operation_generator(
        &mut self,
        geometry: &Geometry,
    ) -> Result<
        Box<dyn OperationGenerator<O1 = Box<dyn Operation>, O2 = Box<dyn Operation>>>,
        AUTDInternalError,
    > {
        let mut tmp: D = D::default();
        std::mem::swap(self, &mut tmp);
        let g = <Self as Datagram>::operation_generator(tmp, geometry)?;

        let b = DynamicOperationGenerator { g: Box::new(g) };
        Ok(Box::new(b))
    }

    fn timeout(&self) -> Option<Duration> {
        <Self as Datagram>::timeout(self)
    }

    fn parallel_threshold(&self) -> Option<usize> {
        <Self as Datagram>::parallel_threshold(self)
    }
}

pub struct DynamicOperationGeneratorPack {
    pub g: Box<dyn OperationGenerator<O1 = Box<dyn Operation>, O2 = Box<dyn Operation>>>,
}

impl OperationGenerator for DynamicOperationGeneratorPack {
    type O1 = Box<dyn Operation>;
    type O2 = Box<dyn Operation>;

    fn generate(&self, device: &Device) -> (Self::O1, Self::O2) {
        self.g.generate(device)
    }
}

#[derive(Debug)]
pub struct DynamicDatagramPack {
    pub d: Box<Box<dyn DynamicDatagram>>,
}

impl Datagram for DynamicDatagramPack {
    type G = DynamicOperationGeneratorPack;

    fn operation_generator(mut self, geometry: &Geometry) -> Result<Self::G, AUTDInternalError> {
        Ok(DynamicOperationGeneratorPack {
            g: self.d.operation_generator(geometry)?,
        })
    }

    fn timeout(&self) -> Option<Duration> {
        self.d.timeout()
    }

    fn parallel_threshold(&self) -> Option<usize> {
        self.d.parallel_threshold()
    }
}

unsafe impl Send for DynamicDatagramPack {}
unsafe impl Sync for DynamicDatagramPack {}

impl DatagramDefault for autd3_driver::datagram::Synchronize {
    fn default() -> Self {
        Self::new()
    }
}

impl DatagramDefault for autd3_driver::datagram::Silencer<autd3::prelude::FixedUpdateRate> {
    fn default() -> Self {
        autd3_driver::datagram::Silencer::new(autd3::prelude::FixedUpdateRate {
            intensity: NonZeroU16::MIN,
            phase: NonZeroU16::MIN,
        })
    }
}

impl DatagramDefault for autd3_driver::datagram::Silencer<autd3::prelude::FixedCompletionTime> {
    fn default() -> Self {
        autd3_driver::datagram::Silencer::new(autd3::prelude::FixedCompletionTime {
            intensity: Duration::ZERO,
            phase: Duration::ZERO,
        })
    }
}

impl DatagramDefault for autd3_driver::datagram::SwapSegment {
    fn default() -> Self {
        Self::Gain(Segment::S0)
    }
}

impl DatagramDefault for autd3_driver::datagram::ReadsFPGAState<Box<dyn Fn(&Device) -> bool>> {
    fn default() -> Self {
        Self::new(Box::new(|_| false))
    }
}

impl DatagramDefault
    for autd3_driver::datagram::PulseWidthEncoder<
        Box<dyn Fn(u8) -> u8 + Send + Sync>,
        Box<dyn Fn(&Device) -> Box<dyn Fn(u8) -> u8 + Send + Sync>>,
    >
{
    fn default() -> Self {
        Self::new(Box::new(|_| Box::new(|_| 0)))
    }
}

impl DatagramDefault for autd3_driver::datagram::ForceFan<Box<dyn Fn(&Device) -> bool>> {
    fn default() -> Self {
        Self::new(Box::new(|_| false))
    }
}

impl DatagramDefault
    for autd3_driver::datagram::DebugSettings<
        Box<dyn Fn(&Device, autd3::prelude::GPIOOut) -> autd3::prelude::DebugType + Send + Sync>,
    >
{
    fn default() -> Self {
        Self::new(Box::new(|_, _| autd3::prelude::DebugType::None))
    }
}

impl DatagramDefault for autd3_driver::datagram::Clear {
    fn default() -> Self {
        Self::new()
    }
}

impl DatagramDefault for autd3::prelude::GainSTM<Box<G>> {
    fn default() -> Self {
        unsafe { autd3::prelude::GainSTM::uninit() }
    }
}

impl<const N: usize> DatagramDefault for autd3::prelude::FociSTM<N> {
    fn default() -> Self {
        unsafe { autd3::prelude::FociSTM::uninit() }
    }
}

impl DatagramDefault for Box<G> {
    fn default() -> Self {
        Box::new(autd3::gain::Null {})
    }
}

impl DatagramDefault for Box<M> {
    fn default() -> Self {
        Box::new(autd3::modulation::Static::new())
    }
}

impl<D: autd3::derive::DatagramS + DatagramDefault> DatagramDefault
    for autd3_driver::datagram::DatagramWithSegment<D>
{
    fn default() -> Self {
        use autd3::prelude::IntoDatagramWithSegment;
        D::default().with_segment(autd3::derive::Segment::S0, false)
    }
}

impl<D: autd3::derive::DatagramST + DatagramDefault> DatagramDefault
    for autd3_driver::datagram::DatagramWithSegmentTransition<D>
{
    fn default() -> Self {
        use autd3::prelude::IntoDatagramWithSegmentTransition;
        D::default().with_segment(autd3::derive::Segment::S0, None)
    }
}

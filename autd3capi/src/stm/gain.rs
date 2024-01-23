#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{
    driver::datagram::{GainSTM, STMProps},
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGain(
    props: STMPropsPtr,
    gains: *const GainPtr,
    size: u32,
    mode: GainSTMMode,
) -> ResultDatagram {
    GainSTM::<Box<dyn Gain>>::from_props_mode(*Box::from_raw(props.0 as *mut STMProps), mode.into())
        .add_gains_from_iter(
            (0..size as usize).map(|i| *Box::from_raw(gains.add(i).read().0 as *mut Box<G>)),
        )
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGainAddGain(stm: DatagramPtr, gain: GainPtr) -> ResultDatagram {
    Box::from_raw(stm.0 as *mut Box<GainSTM<_>>)
        .add_gain(*Box::from_raw(gain.0 as *mut Box<G>))
        .into()
}

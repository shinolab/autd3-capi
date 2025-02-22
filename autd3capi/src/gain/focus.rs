use autd3::gain::FocusOption;
use autd3capi_driver::{autd3::gain::Focus, *};

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGainFocus(pos: Point3, option: FocusOption) -> GainPtr {
    Focus { pos, option }.into()
}

#[unsafe(no_mangle)]
#[must_use]
pub unsafe extern "C" fn AUTDGainFocusIsDefault(option: FocusOption) -> bool {
    option == Default::default()
}

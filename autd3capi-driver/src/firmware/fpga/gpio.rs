#[repr(u8)]

pub enum GPIOIn {
    I0 = 0,
    I1 = 1,
    I2 = 2,
    I3 = 3,
}

impl From<GPIOIn> for autd3_driver::firmware::fpga::GPIOIn {
    fn from(gpio: GPIOIn) -> Self {
        match gpio {
            GPIOIn::I0 => autd3_driver::firmware::fpga::GPIOIn::I0,
            GPIOIn::I1 => autd3_driver::firmware::fpga::GPIOIn::I1,
            GPIOIn::I2 => autd3_driver::firmware::fpga::GPIOIn::I2,
            GPIOIn::I3 => autd3_driver::firmware::fpga::GPIOIn::I3,
        }
    }
}

impl From<autd3_driver::firmware::fpga::GPIOIn> for GPIOIn {
    fn from(gpio: autd3_driver::firmware::fpga::GPIOIn) -> Self {
        match gpio {
            autd3_driver::firmware::fpga::GPIOIn::I0 => GPIOIn::I0,
            autd3_driver::firmware::fpga::GPIOIn::I1 => GPIOIn::I1,
            autd3_driver::firmware::fpga::GPIOIn::I2 => GPIOIn::I2,
            autd3_driver::firmware::fpga::GPIOIn::I3 => GPIOIn::I3,
        }
    }
}

#[repr(u8)]

pub enum GPIOOut {
    O0 = 0,
    O1 = 1,
    O2 = 2,
    O3 = 3,
}

impl From<GPIOOut> for autd3_driver::firmware::fpga::GPIOOut {
    fn from(gpio: GPIOOut) -> Self {
        match gpio {
            GPIOOut::O0 => autd3_driver::firmware::fpga::GPIOOut::O0,
            GPIOOut::O1 => autd3_driver::firmware::fpga::GPIOOut::O1,
            GPIOOut::O2 => autd3_driver::firmware::fpga::GPIOOut::O2,
            GPIOOut::O3 => autd3_driver::firmware::fpga::GPIOOut::O3,
        }
    }
}

impl From<autd3_driver::firmware::fpga::GPIOOut> for GPIOOut {
    fn from(gpio: autd3_driver::firmware::fpga::GPIOOut) -> Self {
        match gpio {
            autd3_driver::firmware::fpga::GPIOOut::O0 => GPIOOut::O0,
            autd3_driver::firmware::fpga::GPIOOut::O1 => GPIOOut::O1,
            autd3_driver::firmware::fpga::GPIOOut::O2 => GPIOOut::O2,
            autd3_driver::firmware::fpga::GPIOOut::O3 => GPIOOut::O3,
        }
    }
}

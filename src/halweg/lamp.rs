/// Lamp UV


use embedded_hal::digital::v2::{OutputPin,ToggleableOutputPin};
// use embedded_hal::timer::CountDown;
// use embedded_hal::timer::Periodic;
// use nb::block;
/// Lamp UV
use nb;


use super::error::Error;

/// Beeper
pub struct Lamp<PIN>
where
    PIN: OutputPin,
{
    /// pin
    pin: PIN,
    // rt:  u64,
    // st:  u64,
}

impl<PIN> Lamp<PIN>
where
    PIN: OutputPin +ToggleableOutputPin,
{
    pub fn create(pin: PIN) -> Self {
        Lamp { pin }
    }
    pub fn on(&mut self) -> nb::Result<(),Error> {
        self.pin.set_high().ok();
        Ok(())
    }
    pub fn off(&mut self) -> nb::Result<(),Error>  {
        self.pin.set_low().ok();
        Ok(())
    }
}


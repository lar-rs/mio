/// Valve I/O
///
///
use embedded_hal::digital::v2::OutputPin;
// use embedded_hal::timer::CountDown;
// use embedded_hal::timer::Periodic;
// use nb::block;
use super::error::Error;
/// Beeper
/// /// Valve I/O

/// Single digital push-pull output pin
pub trait VY {
    /// Drives the pin low
    ///
    fn open(&mut self) -> nb::Result<(), Error>;

    /// Drives the pin high
    ///
    fn close(&mut self) -> nb::Result<(), Error>;

}

pub struct Valve<PIN>
where
    PIN: OutputPin,
{
    /// pin on/off
    pin: PIN,
}

impl<PIN> Valve<PIN>
where
    PIN: OutputPin,
{
    pub fn create(pin: PIN) -> Self {
        Valve { pin }
    }
   
}

impl<PIN> VY for Valve<PIN> 
where
    PIN: OutputPin,
{
    fn open(&mut self) -> nb::Result<(),Error> {
        self.pin.set_high().ok();
        Ok(())
    }
    fn close(&mut self) -> nb::Result<(),Error>  {
        self.pin.set_low().ok();
        Ok(())
    }
}
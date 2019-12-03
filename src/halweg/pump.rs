/// Lamp UV

use embedded_hal::digital::v2::OutputPin;
// use embedded_hal::timer::CountDown;
// use embedded_hal::timer::Periodic;
// use nb::block;

use super::error::Error;



pub trait Pump {
    fn start( &mut self) -> nb::Result<(), Error>;
    fn stop( &mut self)  -> nb::Result<(), Error>;
}


/// Beeper
pub struct GearPump<PIN>
where
    PIN: OutputPin,
{
    /// pin
    pin: PIN,
    // rt:  u64,
    // st:  u64,
}

impl<PIN> GearPump<PIN>
where
    PIN: OutputPin,
{
    pub fn create(pin: PIN) -> Self {
        GearPump { pin }
    }

}


impl<PIN> Pump for GearPump<PIN> 
where
    PIN: OutputPin,
{
    fn start(&mut self) -> nb::Result<(),Error> {
        self.pin.set_high().ok();
        Ok(())
    }
    fn stop(&mut self) -> nb::Result<(),Error>  {
        self.pin.set_low().ok();
        Ok(())
    }

}

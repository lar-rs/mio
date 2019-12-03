/// Furnace

use embedded_hal::digital::v2::{OutputPin,InputPin};
// use embedded_hal::timer::CountDown;
// use embedded_hal::timer::Periodic;
// use nb::block;
use super::error::Error;
/// Beeper
pub struct Furnace<PIN,ISN>
where
    PIN: OutputPin,
    ISN: InputPin,
{
    /// pin
    on: PIN,
    open: PIN,
    close: PIN,
    isopen: ISN,
    isclose: ISN,
    optemp: ISN,
    // rt:  u64,
    // st:  u64,
}

impl<PIN,ISN> Furnace<PIN,ISN>
where
    PIN: OutputPin,
    ISN: InputPin,
{
    pub fn create(on: PIN,open:PIN,close:PIN,isopen:ISN,isclose:ISN,optemp:ISN) -> Self {
        Furnace { state,open,close,isopen,isclose,optemp}
    }
    pub fn open(&mut self)  -> nb::Result<(), Self::Error> {
       Ok(())
    }
    pub fn close(&mut self) -> nb::Result<(), Self::Error> {
        Ok(()))
    }
    pub fn is_op(&mut self) -> nb::Result<bool, Self::Error> {
        let op = self.optemp.is_high().ok();
        Ok(op)
    }
    pub fn on(&mut self) -> nb::Result<(),Error> {
        self.on.set_high().ok();
        Ok(())
    }
    pub fn off(&mut self) -> nb::Result<(),Error>  {
        self.on.set_low().ok();
        Ok(())
    }
}


///
/// Analog
///
///
///

use nb;

/// SPI Transaction mode
pub trait Adc12{
    type Value;

    fn set_value(&mut self,v:Self::Value);
    fn get_value(&mut self)->Self::Value;
}

/// Read write voltage
pub trait Voltage{
    type Error;
    fn get_value(&mut self)->nb::Result<f64,Self::Error>;
    fn set_value(&mut self,value: f64)->nb::Result<(),Self::Error>;
}


/// Read write voltage
pub trait Current{
    type Error;
    fn get_value(&mut self)->nb::Result<f64,Self::Error>;
    fn set_value(&mut self,value: f64)->nb::Result<(),Self::Error>;
}



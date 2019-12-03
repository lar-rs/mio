/// Sensor HAL
/// NDir1,NDir2 Sauerstoff
///
///

use serde::{Deserialize, Serialize};
use core::ops::Range;

pub trait Sensor {
    type Error;
    fn get_fsr(&mut self) -> nb::Result<f64, Self::Error>;
}



pub trait NDir {
    type Error;
    fn get_ppm(&mut self) -> nb::Result<f64,Self::Error>;
}

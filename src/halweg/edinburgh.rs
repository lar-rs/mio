/// Sensor HAL
/// NDir1,NDir2 Sauerstoff
///
///

use serde::{Deserialize, Serialize};

use super::sensor::Sensor;

#[derive(Clone, Serialize,Debug, PartialEq)]
pub enum Level {
    Low,
    In,
    Hight
}

#[derive(Clone, Serialize,Debug, PartialEq)]
pub enum Model {
    Edinburg500,
    Aide50_150_200,
}

#[derive(Clone, Serialize,Debug, PartialEq)]
pub enum Error {
    Bautrate,
    DataString
}

#[derive(Clone, Serialize,Debug, PartialEq)]
pub struct NDir{
    value: f32,
    scale: f32,
    interval:u64,
    range: u8,

}

impl Edinburgh {
    fn get_fsr(&mut self) -> nb::Result<f32,Error> {
        Ok(self.value*self.scale)
    }
    fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }
    fn scale(&self) -> f32 {
        self.scale
    }
    fn interval (&self) -> u64 {
        self.interval
    }
    fn set_interval(&mut self, interval: u64)  {
        self.interval = interval;
    }
}

impl Sensor for NDir {
    type Error = Error;
    fn signal(&mut self) -> nb::Result<f32,Error> {
        self.get_fsr()
    }
}

pub type NDir1 = NDir;
pub type NDir2 = NDir;

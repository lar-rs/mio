/// Sensor 
/// NDir1,NDir2 Sauerstoff usw
///
///

// use async_std::prelude::*;
use async_std::io;
use async_std::fs;
use async_std::path::{PathBuf,Path};
use serde::{Serialize,Deserialize};
use super::{driver,Driver,HID,MioError};
// use async_std::stream;
// use std::time::{Duration,Instant};
pub const SIGNAL: &'static str = "signal";
pub const VALUE: &'static str = "value";
pub const INTERVAL: &'static str = "interval";
pub const SCALE: &'static str = "scale";


use std::time::SystemTime;

// use core::ops::Range;

/// Integration
///
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Integration {
    pub justification: u32,
    pub start_treshold: f64,
    pub stop_treshold: f64,
    pub min_start: u32,
    pub min_stop: u32,
    pub max_stop: u32,
}


impl Default for Integration {
    fn default() -> Self {
        Self {
            justification: 15,
            start_treshold: 0.002,
            stop_treshold: 0.003,
            min_start: 10,
            min_stop: 60,
            max_stop: 210,
        }
    }
}


#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Data {
    pub value: f32,
    pub state: u8,
    pub msec:  u64,
}

impl Data{
    pub fn new(value: f32,state:u8,msec:u64)-> Data {
        Data{
            value: value,
            state: state,
            msec:  msec,
        }
    }
}

pub struct Signal {
    path: PathBuf,
    start: SystemTime,
}

impl Signal{
    pub fn new(path:PathBuf)-> Signal {
        let start = SystemTime::now();
        Signal { path,start }
    }
}
impl From<Driver> for Sensor {
    fn from(drv:Driver) -> Sensor {
        Sensor{
            path: drv.path
        }
    }
}

pub struct Sensor {
    path:     PathBuf,
}

impl Sensor {
    pub async fn select(path: &Path) -> Result<Sensor,MioError> {
        let path    = path.to_path_buf();
        Ok(Sensor{path})
    }
    pub async fn signal(&mut self) -> Result<Signal,MioError>{
        Ok(Signal::new(self.path.join(SIGNAL)))
    }
    pub async fn value (&mut self) -> Result<f32,MioError> {
        let value = fs::read_to_string(self.path.join(VALUE)).await?.parse::<f32>()?;
        Ok(value)
    }
    pub async fn scale (&self) -> Result<f32,MioError>{
        let value = fs::read_to_string(self.path.join(SCALE)).await?.parse::<f32>()?;
        Ok(value)
    }
    pub async fn interval (&self) -> Result<u64,MioError>{
        let interval = fs::read_to_string(self.path.join(INTERVAL)).await?.parse::<u64>()?;
        Ok(interval)
    }
    pub async fn set_scale (&mut self,scale:f32) -> Result<(),MioError>{
        fs::write(self.path.join(SCALE),format!("{}",scale).as_bytes()).await?;
        Ok(())
    }
    pub async fn set_interval(&mut self, millis: u64) -> Result<(),MioError>{
        fs::write(self.path.join(INTERVAL),format!("{}",millis).as_bytes()).await?;
        Ok(())
    }
}

pub async fn ndir(path:&Path) -> io::Result<Sensor> {
    driver::create(path,HID::NDir).await?;
    fs::write(path.join(VALUE), b"0.0").await?;
    fs::write(path.join(INTERVAL), b"500").await?;
    fs::write(path.join(SCALE), b"1.0").await?;
    fs::write(path.join(SIGNAL), b"").await?;
    let sensor = Sensor::select(path).await?;
    Ok(sensor)
}

/// Sensor 
/// NDir1,NDir2 Sauerstoff usw
///
///

// use async_std::prelude::*;
use async_std::io;
use async_std::fs;
use async_std::path::{PathBuf,Path};
use serde::{Serialize,Deserialize};
// use async_std::stream;
// use std::time::{Duration,Instant};
pub const SIGNAL: &'static str = "signal";
pub const LABEL: &'static str = "label";
pub const UNIT: &'static str = "unit";
pub const VALUE: &'static str = "value";
pub const INTERVAL: &'static str = "interval";
pub const SCALE: &'static str = "scale";
pub const MODEL: &'static str = "model";


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

pub struct Sensor {
    path:     PathBuf,
    pub unit:     String,
    pub label:    String,
    pub value:    f32,
    pub model:    String,
}

impl Sensor {
    pub async fn select(path: &Path) -> io::Result<Sensor> {
        let model = fs::read_to_string(path.join(MODEL)).await?;
        let unit    = fs::read_to_string(path.join(UNIT)).await?;
        let label    = fs::read_to_string(path.join(LABEL)).await?;
        let value = fs::read_to_string(path.join(VALUE)).await?.parse::<f32>().unwrap_or(0.0);
        let path  = path.to_path_buf();
        Ok(Sensor{path,label,unit,model,value})
    }
    pub async fn signal(&mut self) -> io::Result<Signal>{
        Ok(Signal::new(self.path.join(SIGNAL)))
    }
    pub async fn value (&mut self) -> io::Result<f32> {
        let value = fs::read_to_string(self.path.join(VALUE)).await?.parse::<f32>().unwrap_or(0.0);
        Ok(0.0)
    }
    pub async fn scale (&self) -> io::Result<f32>{
        let value = fs::read_to_string(self.path.join(SCALE)).await?.parse::<f32>().unwrap_or(1.0);
        Ok(value)
    }
    pub async fn interval (&self) -> io::Result<u64>{
        let interval = fs::read_to_string(self.path.join(INTERVAL)).await?.parse::<u64>().unwrap_or(500);
        Ok(interval)
    }
    pub async fn set_scale (&mut self,scale:f32) -> io::Result<()>{
        fs::write(self.path.join(SCALE),format!("{}",scale).as_bytes()).await?;
        Ok(())
    }
    pub async fn set_interval(&mut self, millis: u64) -> io::Result<()>{
        fs::write(self.path.join(INTERVAL),format!("{}",millis).as_bytes()).await?;
        Ok(())
    }
}

/// Sensor 
/// NDir1,NDir2 Sauerstoff usw
///
///

// use std::prelude::*;
use std::fmt;
// use std::io;
use std::fs;
use std::path::{PathBuf,Path};
use serde::{Serialize,Deserialize};
use super::*;
// use std::stream;
// use std::time::{Duration,Instant};
pub const SIGNAL: &'static str = "signal";
pub const VALUE: &'static str = "value";
pub const INTERVAL: &'static str = "interval";
pub const SCALE: &'static str = "scale";


// use std::time::SystemTime;

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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Down,
    Up,
    Start,
    Brocket,
}



impl From<u8> for State {
    fn from(value: u8) -> Self {
        match value {
            0 => State::Down,
            1 => State::Up,
            2 => State::Start,
            _ => State::Brocket,
        }
    }
}

impl From<State> for u8 {
    fn from(state:State) -> u8 {
        state.into()
    }
}

impl From<&str> for State {
    fn from(value: &str) -> Self {
        match value {
            "down"    =>  State::Down,
            "up"      =>  State::Up,
            "start"   =>  State::Start,
            _         =>  State::Brocket
        }
    }
}

impl From<String> for State {
    fn from(value: String) -> Self {
        State::from(value.as_str())
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            State::Down    =>  return write!(f,"down"),
            State::Up      =>  return write!(f,"up"),
            State::Start   =>  return write!(f,"start"),
            State::Brocket =>  return write!(f,"brocket"),
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

// pub struct Signal {
//     path: PathBuf,
//     start: SystemTime,
// }

// impl Signal{
//     pub fn new(path:PathBuf)-> Signal {
//         let start = SystemTime::now();
//         Signal { path,start }
//     }
// }


use std::convert::TryFrom;
impl TryFrom<Interface> for Sensor {
    type Error = Error;
    fn try_from(iface: Interface) -> Result<Self> {
        iface.set_itype(IType::NDir)?;
        Ok(Self{
            path:iface.path,
        })
    }
}
pub struct Sensor {
    path:     PathBuf,
}

impl Sensor {
    pub fn open(path: &Path) -> Result<Sensor> {
        let path    = path.to_path_buf();
        Ok(Sensor{path})
    }
    // pub fn signal(&mut self) -> Result<Signal>{
        // Ok(Signal::new(self.path.join(SIGNAL)))
    // }
    pub fn value (&mut self) -> Result<f32> {
        let value = fs::read_to_string(self.path.join(VALUE))?.parse::<f32>()?;
        Ok(value)
    }
    pub fn scale (&self) -> Result<f32>{
        let value = fs::read_to_string(self.path.join(SCALE))?.parse::<f32>()?;
        Ok(value)
    }
    pub fn interval (&self) -> Result<u64>{
        let interval = fs::read_to_string(self.path.join(INTERVAL))?.parse::<u64>()?;
        Ok(interval)
    }
    pub fn set_scale (&mut self,scale:f32) -> Result<()>{
        fs::write(self.path.join(SCALE),format!("{}",scale).as_bytes())?;
        Ok(())
    }
    pub fn set_interval(&mut self, millis: u64) -> Result<()>{
        fs::write(self.path.join(INTERVAL),format!("{}",millis).as_bytes())?;
        Ok(())
    }
    pub fn read_data() -> Result<Vec<Data>> {
        let mut datas : Vec<Data> = Vec::new();
         
        Ok(datas)
    }
}

// pub fn ndir(path:&Path) -> io::Result<Sensor> {
//     device::create(path,IType::NDir)?;
//     fs::write(path.join(VALUE), b"0.0")?;
//     fs::write(path.join(INTERVAL), b"500")?;
//     fs::write(path.join(SCALE), b"1.0")?;
//     fs::write(path.join(SIGNAL), b"")?;
//     let sensor = Sensor::select(path)?;
//     Ok(sensor)
// }

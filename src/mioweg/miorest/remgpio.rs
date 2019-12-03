use super::*;
use log::info;
// use std::thread::sleep;
use std::time::{Duration,SystemTime};

use async_std::prelude::*;
use async_std::io;
use async_std::fs;
use async_std::task;
// use linux_embedded_hal::Gpio;

use std::fmt;

pub fn gpiodir()-> PathBuf {
    PathBuf::from("/sys/class/gpio")
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Gpio {
    pin: u64,
    path: PathBuf,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    In,
    Out,
    High,
    Low,
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            0 => Direction::In,
            1 => Direction::Out,
            2 => Direction::High,
            3 => Direction::Low,
            _ => Direction::In,
        }
    }
}

impl From<String> for Direction {
    fn from(value: String) -> Self {
        match value.trim() {
            "in"   =>  Direction::In,
            "out"  =>  Direction::Out,
            "high" =>  Direction::High,
            "low"  =>  Direction::Low,
            _      =>  Direction::In
        }
    }
}
impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "in"   =>  Direction::In,
            "out"  =>  Direction::Out,
            "high" =>  Direction::High,
            "low"  =>  Direction::Low,
             _     =>  Direction::In
        }
    }
}
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::In =>  return write!(f,"in"),
            Direction::Out => return write!(f,"out"),
            Direction::High =>return write!(f,"high"),
            Direction::Low => return write!(f,"low"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edge {
    NoInterrupt,
    RisingEdge,
    FallingEdge,
    BothEdges,
}

impl From<u8> for Edge {
    fn from(value: u8) -> Self {
        match value {
            0 => Edge::NoInterrupt,
            1 => Edge::RisingEdge,
            2 => Edge::FallingEdge,
            3 => Edge::BothEdges,
            _ => Edge::NoInterrupt,
        }
    }
}

impl From<String> for Edge {
    fn from(value: String) -> Self {
        match value.trim() {
            "none"    =>  Edge::NoInterrupt,
            "rising"  =>  Edge::RisingEdge,
            "falling" =>  Edge::FallingEdge,
            "both"    =>  Edge::BothEdges,
            _         =>  Edge::NoInterrupt
        }
    }
}
impl From<&str> for Edge {
    fn from(value: &str) -> Self {
        match value {
            "none"   =>   Edge::NoInterrupt,
            "rising"  =>  Edge::RisingEdge,
            "falling" =>     Edge::FallingEdge,
            "both"  =>     Edge::BothEdges,
             _     =>     Edge::NoInterrupt
        }
    }
}
impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Edge::NoInterrupt =>  return write!(f,"none"),
            Edge::RisingEdge  =>  return write!(f,"rising"),
            Edge::FallingEdge =>  return write!(f,"falling"),
            Edge::BothEdges   =>  return write!(f,"both"),
        }
    }
}


impl Gpio {
    pub fn directory(&self) -> PathBuf {
        gpiodir().join(&format!("gpio{}", self.pin))
    }
    pub fn value_path(&self,) -> PathBuf {
        self.directory().join("value")
    }
    pub fn direction_path(&self) -> PathBuf {
        self.directory().join("direction")
    }
    pub fn edge_path(&self) -> PathBuf {
        self.directory().join("edge")
    }
    pub fn pin_number(&self) -> u64 {
        self.pin
    }
    pub fn new(pin:u64) -> Gpio {
        let path = gpiodir().join(&format!("gpio{}", pin));
        Gpio {path:path,pin:pin}
    }
}

impl From<PathBuf> for Gpio {
    fn from(path:PathBuf) -> Gpio {
        let id = path.file_name()
            .and_then(|filename| filename.to_str())
            .and_then(|filename_str| filename_str.trim_start_matches("gpio").parse::<u64>().ok()).unwrap();
        Gpio{
            path:path,
            pin:id,
        }
    }

}

impl From<u64> for Gpio {
    fn from(pin:u64) -> Gpio {
        Gpio::new(pin)
    }
}

pub async fn set_high(pin:&Gpio) -> io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .open(pin.value_path())
        .await?;
    file.write_all(b"1").await?;
    file.sync_all().await?;
    Ok(())
}
pub async fn set_low(pin:&Gpio) -> io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .open(pin.value_path())
        .await?;
    file.write_all(b"0").await?;
    file.sync_all().await?;
    Ok(())
}
pub async fn set_value(pin:&Gpio,value:bool) -> io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .open(pin.value_path())
        .await?;
    file.write_all(match value {
        true => b"1",
        false => b"0"}).await?;
    file.sync_all().await?;
    Ok(())
}


/// Set this GPIO as either an input or an output
/// The basic values allowed here are Direction::In and Direction::Out which set the Gpio as either an input or output respectively. In addition to those, two additional settings of Direction::High and Direction::Low. These both set the Gpio as an output but do so with an initial value of high or low respectively. This allows for glitch-free operation.
/// Note that this entry may not exist if the kernel does not support changing the direction of a pin in userspace. If this is the case, you will get an error.
pub async fn get_value(pin:&Gpio) -> io::Result<bool> {
    let path = pin.value_path();
    match fs::read_to_string(&path).await?.as_str() {
        "1" => Ok(true),
        _ => Ok(false),
    }
}


/// check pin is exported
pub async fn is_exported(pin:&Gpio)-> io::Result<bool> {
    Ok(fs::metadata(&pin.directory()).await?.is_dir())
}

pub async fn unexport(pin:&Gpio) -> io::Result<()> {
    let mut unexport = fs::OpenOptions::new()
        .write(true)
        .open(gpiodir().join("unexport"))
        .await?;
    let num = format!("{}",pin.pin_number());
    unexport.write_all(num.as_bytes()).await?;
    unexport.sync_all().await?;
    Ok(())

}

/// export pin
pub async fn export(gpio:Gpio) -> io::Result<()> {

    if fs::metadata(&gpio.directory()).await.is_err() {
        info!("export GPIO:{:?}",gpio);
        let mut export = fs::OpenOptions::new()
            .write(true)
            .open(gpiodir().join("export"))
            .await?;
        let pin = format!("{}",gpio.pin_number());
        export.write_all(pin.as_bytes()).await?;
        export.sync_all().await?;
   }
   Ok(())
}

///read pin direction
pub async fn direction(pin: &Gpio)-> io::Result<String> {
    let path = pin.direction_path();
    Ok(fs::read_to_string(&path).await?)

}
///read pin edge
pub async fn edge(pin: &Gpio)-> io::Result<String> {
    let path = pin.edge_path();
    Ok(fs::read_to_string(&path).await?)
}
/// change direction
pub async fn set_direction(pin:&Gpio,dir:Direction) -> io::Result<()> {
    let path = pin.direction_path();
    let dir = format!("{}",dir);
    info!("export GPIO:{} set direction {}", pin,dir);
    let mut file = fs::OpenOptions::new() .write(true) .open(&path) .await?;
    file.write_all(dir.as_bytes()).await?;
    file.sync_all().await?;
    Ok(())
}

/// change edge
pub async fn set_edge(pin:&Gpio,edge:Edge) -> io::Result<()> {
    let path = pin.edge_path();
    let edge = format!("{}",edge);
    info!("GPIO:{} set edge {}", pin,edge);
    let mut file = fs::OpenOptions::new() .write(true) .open(&path) .await?;
    file.write_all(edge.as_bytes()).await?;
    file.sync_all().await?;
    Ok(())
}

impl fmt::Display for Gpio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GPIO{} {:?}", self.pin,self.directory())
    }
}


// Closes the gpio and write its pin number into /sys/class/gpio/unexport
// impl Drop for Gpio {
//     fn drop(&mut self) {
//         if let Ok(mut unexport) = OpenOptions::new()
//             .write(true)
//             .open(format!("{}unexport", GPIO_PATH))
//         {
//             match unexport.write_all(format!("{}", self.pin).as_bytes()) {
//                 Err(why) => panic!("couldn't close gpio {}: {}", self.pin, why),
//                 Ok(_) => {}
//             }
//         } else {
//             panic!("file error: {}")
//         }
//     }
// }
//

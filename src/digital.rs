/// Digital Input Output interface 
/// 
// use async_std::prelude::*;
use async_std::io;
use async_std::fs;
use std::fmt;
// use async_std::task;
// use sysfs_gpio::{Direction,Pin};
use async_std::path::{ Path, PathBuf};
use async_trait::async_trait;
use super::{MioError,driver,HID};

pub const VALUE: &'static str = "value";

#[async_trait]
pub trait Input{
    async fn is_low(&self,path:&Path) -> io::Result<bool> {
        match fs::read_to_string(path.join(VALUE)).await?.as_str() {
            "1" => Ok(false),
            _ => Ok(true),
        }
    }
    async fn is_high(&self,path:&Path) -> io::Result<bool>{
        match fs::read_to_string(path.join(VALUE)).await?.as_str() {
            "1" => Ok(true),
            _ => Ok(false),
        }
    }
}
#[async_trait]
pub trait Output{
    async fn set_low(&self,path:&Path) -> io::Result<()> {
        fs::write(path.join(VALUE), b"0").await?;
        Ok(())
    }
    async fn set_high(&self,path:&Path) -> io::Result<()> {
        fs::write(path.join(VALUE), b"1").await?;
        Ok(())
    }
    async fn is_low(&self,path:&Path) -> io::Result<bool> {
        match fs::read_to_string(path.join(VALUE)).await?.as_str() {
            "1" => Ok(false),
            _ => Ok(true),
        }
    }
    async fn is_high(&self,path:&Path) -> io::Result<bool>{
        match fs::read_to_string(path.join(VALUE)).await?.as_str() {
            "1" => Ok(true),
            _ => Ok(false),
        }
    }
}

/// Digital simulation
pub struct Digital {
    path:  PathBuf,
}

#[async_trait]
impl Output for Digital{}

impl Digital{
    pub async fn select(path:&Path) -> io::Result<Digital> {
        Ok(
            Digital{
                path: path.to_path_buf(),
            }
        )
    }
    pub async fn toggle(&mut self) -> io::Result<bool> {
        if self.is_high(&self.path).await? {
            self.set_low(&self.path).await?;
            Ok(false)
        }else {
            self.set_high(&self.path).await?;
            Ok(true)
        }
    }
}

pub async fn get_value(path:&Path) -> Result<bool,MioError> {
    match fs::read_to_string(path.join(VALUE)).await?.as_str() {
        "1" => Ok(true),
        _ => Ok(false),
    }
}
pub async fn set_value(path:&Path,val:bool) -> Result<(),MioError> {
    fs::write(path.join(VALUE),match val {
        true  => b"1",
        false => b"0",
    }).await?;
    Ok(())
}
pub async fn set_high(path:&Path) -> Result<(),MioError> {
    set_value(path,true).await
}
pub async fn set_low(path:&Path) -> Result<(),MioError> {
    set_value(path,false).await
}
pub async fn is_low(path:&Path) -> Result<bool,MioError> {
    let val = get_value(path).await?;
    Ok(!val)
}
pub async fn is_high(path:&Path) -> Result<bool,MioError>{
    let val = get_value(path).await?;
    Ok(!val)
}

// pub async fn link_output_gpio(dest:&Path,pin:u64) -> Result<(),MioError> {
//     // let path = gpiosys().await?;
//     let path = format!("/sys/class/gpio/gpio{}",pin);
//     let pin = Pin::new(pin);
//     pin.export()?;
//     pin.set_direction(Direction::Low).unwrap();
//     pin.set_value(0).unwrap();
//     fs::hard_link(dest.join("value"), path.join("value")).await?;
//     Ok(())
// }


pub async fn simulation(path:&Path) -> Result<Digital,MioError> {
    driver::create(path,HID::DigitalOut).await?;
    fs::write(path.join("value"), b"0").await?;
    let dig = Digital::select(path).await?;
    Ok(dig)
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
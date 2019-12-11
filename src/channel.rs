/// Methode
/// 
/// 
use std::fs;
use std::fmt;
// use sysfs_gpio::{Direction,Pin};
use std::path::{PathBuf};
// use std::fmt;

// use async_trait::async_trait;
use super::*;

use std::convert::TryFrom;

impl TryFrom<Interface> for Channel {
    type Error = Error;
    fn try_from(iface: Interface) -> Result<Self> {
        iface.set_itype(IType::Method)?;
        iface.set_iclass(IClass::Method)?;
        Ok(Self{
            path:iface.path,
        })
    }
}



/// Interface 
/// 
pub struct Channel{
    pub path: PathBuf,

}


impl Channel {
    pub fn get_sensor(&self) -> Result<Sensor> {
        let path = self.path.join("sensor");
        if !path.is_dir() {
            Err(Error::Interface{msg:format!("channel ensor not found")})
        }else{
            Sensor::open(path.as_path())
        }
    }
    pub fn value(&self) -> Result<f32> {
        let value = fs::read_to_string(self.path.join("value"))?.parse::<f32>()?;
        Ok(value)
    }
}


impl fmt::Display for Channel{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"CH#{}",self.path.as_path().display())
    }
}

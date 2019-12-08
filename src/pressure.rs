use std::fmt;
// pub use uom::si::f32::{Ratio, Presure};
use std::prelude::*;
use std::stream;

use std::convert::TryFrom;
impl TryFrom<Interface> for Pressure {
    type Error = Error;
    fn try_from(iface: Interface) -> Result<Self> {
        iface.set_itype(IType::Pressure)?;
        Ok(Self{
            path:iface.path,
        })
    }
}


/// Hardware presure sensor.
pub struct Pressure {
    path: PathBuf,

}

impl Pressure{

    /// Returns current presure reported by sensor.
    pub fn current_pressure(&self) -> Result<u32> {
        let value = fs::read_to_string(self.path.join("pressure"))?.parse::<u32>()?;
        Ok(value)
    }
    pub fn warning_pressure(&self) -> Result<u32> {
        let warn = fs::read_to_string(self.path.join("warning_pressure"))?.parse::<u32>()?;
         Ok(warn)
    }
    pub fn critical_pressure(&self) -> Result<u32> {
        let critical = fs::read_to_string(self.path.join("critical_pressure"))?.parse::<u32>()?;
         Ok(critical)
    }

}


// pub fn check(sensor:Sensor) -> io::Result<bool> {

// }

pub fn pressure(path: &Path) -> io::Result<Presure> {
    
    let sensor = sensor(path.as_path())?;
    Ok(sensor)
}


pub fn create(path:&Path ) -> io::Result<Pressure> {

}

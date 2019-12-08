
/// Stirrer
///
///
///
// use std::prelude::*;

use super::*;
use std::fs;
// use std::stream::Stream;
use std::path::{PathBuf};


impl From<&Interface> for Stirrer {
    #[inline]
    fn from(device:&Interface) -> Stirrer {
        Stirrer{
            path: device.path.to_path_buf()
        }
    }
}

impl From<&Stirrer> for Interface {
    #[inline]
    fn from(stirrer:&Stirrer) -> Interface {
        Interface{
            path:stirrer.path.to_path_buf()
        }
    }
}

use std::convert::TryFrom;
impl TryFrom<Interface> for Stirrer {
    type Error = Error;
    fn try_from(iface: Interface) -> Result<Self> {
        iface.set_itype(IType::NDir)?;
        Ok(Self{
            path:iface.path,
        })
    }
}

pub struct Stirrer {
    pub path : PathBuf,
}

/// Injection interface..
impl Stirrer {
    /// current in mAh
    pub fn current(&self ) -> Result<u32> {
        let current = fs::read_to_string(self.path.join("current"))?.parse::<u32>()?;
        Ok(current)
    }
    pub fn delay( &mut self) -> Result<u32> {
        let delay = fs::read_to_string(self.path.join("delay"))?.parse::<u32>()?;
        Ok(delay)
    }
    pub fn state(&self) -> Result<bool> {
        match fs::read_to_string(self.path.join("state"))?.as_str() {
            "1" => Ok(true),
            _ => Ok(false),
        }
    }
    pub fn start(&mut self) -> Result<()> {
        fs::write(self.path.join("state"),b"1")?;
        Ok(())
    }
    pub fn stop(&mut self) -> Result<()> {
        fs::write(self.path.join("state"),b"0")?;
        Ok(())
    }
    pub fn set_delay(&mut self,delay:u32) -> Result<()> {
        fs::write(self.path.join("delay"),format!("{}",delay).as_bytes())?;
        Ok(())
    }
    pub fn set_current( &mut self , current: u32 ) -> Result<()> {
        fs::write(self.path.join("current"),format!("{}",current).as_bytes())?;
        Ok(())
    }

}



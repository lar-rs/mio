
/// Autosampler
///
///
///
// use std::prelude::*;
use std::io;

use super::{device,error,MioError};
use std::fs;
// use std::stream::Stream;
use std::path::{PathBuf};
use std::stream;
use std::time::Duration;
use std::prelude::*;

use std::convert::TryFrom;
/// interface transfer

impl TryFrom<Interface> for Injection {
    type Error = Error;
    fn try_from(iface: Interface) -> Result<Self> {
        iface.set_itype(IType::Axis)?;
        Ok(Self{
            path:iface.path,
        })
    }
}
pub struct Injection{
    path     : PathBuf,
}

/// Injection interface..
impl Injection {
    // type Velocity;
    pub fn rinsing(&self ) -> Result<u32> {
        let position = fs::read_to_string(self.path.join("position"))?.parse::<u32>()?;
        Ok(position)
    }
    pub fn max( &mut self) -> Result<u32> {
        let max = fs::read_to_string(self.path.join("max"))?.parse::<u32>()?;
        Ok(max)
    }
    pub fn velocity(&mut self) -> Result<u64> {
        let velocity = fs::read_to_string(self.path.join("velocity"))?.parse::<u64>()?;
        Ok(velocity)
    }
    pub fn is_sensor(&self) -> Result<bool> {
        match fs::read_to_string(self.path.join("sensor"))?.as_str() {
            "1" => Ok(true),
            _ => Ok(false),
        }
    }
    pub fn move_to(&mut self ,moveto:u32) -> Result<()> {
        fs::write(self.path.join("move"),format!("{}",moveto).as_bytes())?;
        let pos  =  self.position()?;
        let velocity = self.velocity()?;
        let  diff =  if pos > moveto {
            pos - moveto
        }else {
            moveto - pos
        };
        let mut interval  = stream::interval(Duration::from_millis(velocity)).take(diff as usize);
        let now = std::time::SystemTime::now();
        while let Some(_) = interval.next() {
            if moveto == self.position()? {
                break;
            }
        } 
        if moveto != self.position()? {
            let msg = format!("axis:{} move from {} to {} error - timeout in {} millis",device::label(self.path.as_path())?,pos,moveto,
            now.elapsed().unwrap_or(Duration::from_millis(999999u64)).as_millis());
            log::warn!("{}",msg.as_str());
            return Err(error::driver_timeout(msg))
        }
        Ok(())
    }
    pub fn goto_sensor(&mut self) -> Result<()> {
        fs::write(self.path.join("command"),b"1")?;
        let pos  =  self.position()?;
        let velocity = self.velocity()?;
        let mut interval  = stream::interval(Duration::from_millis(velocity)).take(pos as usize);
        let now = std::time::SystemTime::now();
        while let Some(_) = interval.next() {
           if self.is_sensor()? {
                break;
            }
        } 
        if  self.is_sensor()? {
            let msg = format!("axis:{} command go to sensor fail - timeout in {} millis",device::label(self.path.as_path())?,
            now.elapsed().unwrap_or(Duration::from_millis(999999u64)).as_millis());
            log::warn!("{}",msg.as_str());
            return Err(error::driver_timeout(msg))
        }
        Ok(())
    }
  
    pub fn set_max(&mut self,max:u32) -> Result<()> {
        fs::write(self.path.join("max"),format!("{}",max).as_bytes())?;
        Ok(())
    }
    pub fn set_velocity( &mut self , velocity: u64 ) -> Result<()> {
        fs::write(self.path.join("velocity"),format!("{}",velocity).as_bytes())?;
        Ok(())
    }
  
}



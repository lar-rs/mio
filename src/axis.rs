/// Autosampler
///
///
///
// use async_std::prelude::*;
use async_std::io;

use super::{driver,error,HID,Driver,MioError};
use async_std::fs;
// use async_std::stream::Stream;
use async_std::path::{PathBuf};
use async_std::stream;
use std::time::Duration;
use async_std::prelude::*;

pub const VELOCITY: &'static str = "velocity";
pub const POSITION: &'static str = "position";
pub const HOLD: &'static str = "hold";
pub const MAX: &'static str = "max";
pub const ENDSCHALTER: &'static str = "endschalter";
pub const PTP: &'static str = "ptp";
pub const COMMAND: &'static str = "command";
pub const GOTO: &'static str = "goto";



pub struct Axis{
    path     : PathBuf,
}

/// Axis interface..
impl Axis {
    // type Velocity;
    pub async fn position(&self ) -> Result<u32,MioError> {
        let position = fs::read_to_string(self.path.join(POSITION)).await?.parse::<u32>()?;
        Ok(position)
    }
    pub async fn max( &mut self) -> Result<u32,MioError> {
        let max = fs::read_to_string(self.path.join(MAX)).await?.parse::<u32>()?;
        Ok(max)
    }
    pub async fn velocity(&mut self) -> Result<u64,MioError> {
        let velocity = fs::read_to_string(self.path.join(VELOCITY)).await?.parse::<u64>()?;
        Ok(velocity)
    }
    pub async fn is_sensor(&self) -> Result<bool,MioError> {
        match fs::read_to_string(self.path.join(ENDSCHALTER)).await?.as_str() {
            "1" => Ok(true),
            _ => Ok(false),
        }
    }
    pub async fn goto(&mut self ,goto:u32) -> Result<(),MioError> {
        fs::write(self.path.join(GOTO),format!("{}",goto).as_bytes()).await?;
        let pos  =  self.position().await?;
        let velocity = self.velocity().await?;
        let  diff =  if pos > goto {
            pos - goto
        }else {
            goto - pos
        };
        let mut interval  = stream::interval(Duration::from_millis(velocity)).take(diff as usize);
        let now = std::time::SystemTime::now();
        while let Some(_) = interval.next().await {
            if goto == self.position().await? {
                break;
            }
        } 
        if goto != self.position().await? {
            let msg = format!("axis:{} move from {} to {} error - timeout in {} millis",driver::label(self.path.as_path()).await?,pos,goto,
            now.elapsed().unwrap_or(Duration::from_millis(999999u64)).as_millis());
            log::warn!("{}",msg.as_str());
            return Err(error::driver_timeout(msg))
        }
        Ok(())
    }
    pub async fn goto_sensor(&mut self) -> Result<(),MioError> {
        fs::write(self.path.join(COMMAND),b"1").await?;
        let pos  =  self.position().await?;
        let velocity = self.velocity().await?;
        let mut interval  = stream::interval(Duration::from_millis(velocity)).take(pos as usize);
        let now = std::time::SystemTime::now();
        while let Some(_) = interval.next().await {
           if self.is_sensor().await? {
                break;
            }
        } 
        if  self.is_sensor().await? {
            let msg = format!("axis:{} command go to sensor fail - timeout in {} millis",driver::label(self.path.as_path()).await?,
            now.elapsed().unwrap_or(Duration::from_millis(999999u64)).as_millis());
            log::warn!("{}",msg.as_str());
            return Err(error::driver_timeout(msg))
        }
        Ok(())
    }
  
    pub async fn set_max(&mut self,max:u32) -> Result<(),MioError> {
        fs::write(self.path.join(MAX),format!("{}",max).as_bytes()).await?;
        Ok(())
    }
    pub async fn set_velocity( &mut self , velocity: u64 ) -> Result<(),MioError> {
        fs::write(self.path.join(VELOCITY),format!("{}",velocity).as_bytes()).await?;
        Ok(())
    }
  
}



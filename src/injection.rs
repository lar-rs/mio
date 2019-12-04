
/// Autosampler
///
///
///
// use async_std::prelude::*;
use async_std::io;

use super::{driver,error,MioError};
use async_std::fs;
// use async_std::stream::Stream;
use async_std::path::{PathBuf};
use async_std::stream;
use std::time::Duration;
use async_std::prelude::*;

pub struct Injection{
    path     : PathBuf,
}

/// Injection interface..
impl Injection {
    // type Velocity;
    pub async fn rinsing(&self ) -> Result<u32,MioError> {
        let position = fs::read_to_string(self.path.join("position")).await?.parse::<u32>()?;
        Ok(position)
    }
    pub async fn max( &mut self) -> Result<u32,MioError> {
        let max = fs::read_to_string(self.path.join("max")).await?.parse::<u32>()?;
        Ok(max)
    }
    pub async fn velocity(&mut self) -> Result<u64,MioError> {
        let velocity = fs::read_to_string(self.path.join("velocity")).await?.parse::<u64>()?;
        Ok(velocity)
    }
    pub async fn is_sensor(&self) -> Result<bool,MioError> {
        match fs::read_to_string(self.path.join("sensor")).await?.as_str() {
            "1" => Ok(true),
            _ => Ok(false),
        }
    }
    pub async fn move_to(&mut self ,moveto:u32) -> Result<(),MioError> {
        fs::write(self.path.join("move"),format!("{}",moveto).as_bytes()).await?;
        let pos  =  self.position().await?;
        let velocity = self.velocity().await?;
        let  diff =  if pos > moveto {
            pos - moveto
        }else {
            moveto - pos
        };
        let mut interval  = stream::interval(Duration::from_millis(velocity)).take(diff as usize);
        let now = std::time::SystemTime::now();
        while let Some(_) = interval.next().await {
            if moveto == self.position().await? {
                break;
            }
        } 
        if moveto != self.position().await? {
            let msg = format!("axis:{} move from {} to {} error - timeout in {} millis",driver::label(self.path.as_path()).await?,pos,moveto,
            now.elapsed().unwrap_or(Duration::from_millis(999999u64)).as_millis());
            log::warn!("{}",msg.as_str());
            return Err(error::driver_timeout(msg))
        }
        Ok(())
    }
    pub async fn goto_sensor(&mut self) -> Result<(),MioError> {
        fs::write(self.path.join("command"),b"1").await?;
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
        fs::write(self.path.join("max"),format!("{}",max).as_bytes()).await?;
        Ok(())
    }
    pub async fn set_velocity( &mut self , velocity: u64 ) -> Result<(),MioError> {
        fs::write(self.path.join("velocity"),format!("{}",velocity).as_bytes()).await?;
        Ok(())
    }
  
}



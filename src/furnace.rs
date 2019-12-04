/// Monitor gear pump normally used for solution sampling.
///
///

// use async_std::prelude::*;
// use async_std::io;
use async_std::fs;
use async_std::prelude::*;
use async_std::stream;
use std::time::Duration;
use async_std::path::PathBuf;
// use async_trait::async_trait;
use super::{error,MioError};



pub struct Furnace {
    path: PathBuf,
}

impl Furnace{
    pub async fn is_open(&self)   -> Result<bool,MioError> {
        match fs::read_to_string(self.path.join("isopen")).await?.as_str() {
            "1" => Ok(true),
            _ => Ok(false),
        }
    }
    pub async fn is_close(&self)   -> Result<bool,MioError> {
        match fs::read_to_string(self.path.join("isclose")).await?.as_str() {
            "1" => Ok(true),
            _ => Ok(false),
        }
    }
    pub async fn is_ready(&mut self) -> Result<bool,MioError> {
        match fs::read_to_string(self.path.join("ready")).await?.as_str() {
            "1" => Ok(true),
            _ => Ok(false),
        }
     }
    pub async fn open(&mut self)  -> Result<(),MioError> {
        fs::write(self.path.join("open"), b"1").await?;
        let mut interval  = stream::interval(Duration::from_millis(500)).take(10);
        let now = std::time::SystemTime::now();
        while let Some(_) = interval.next().await {
            if self.is_open().await? {
                 break;
             }
         } 
         fs::write(self.path.join("open"), b"0").await?;
         if !self.is_open().await? {
            let msg = format!("furnace open fail - timeout in {} millis",
            now.elapsed().unwrap_or(Duration::from_millis(999999u64)).as_millis());
            log::warn!("{}",msg.as_str());
            return Err(error::driver_timeout(msg))
        }
         Ok(())
    }
    pub async fn close(&mut self) -> Result<(),MioError> {
        fs::write(self.path.join("close"), b"1").await?;
        let mut interval  = stream::interval(Duration::from_millis(500)).take(10);
        let now = std::time::SystemTime::now();
        while let Some(_) = interval.next().await {
            if self.is_close().await? {
                 break;
             }
         } 
        fs::write(self.path.join("close"), b"0").await?;
        if !self.is_close().await? {
            let msg = format!("furnace open fail - timeout in {} millis",
            now.elapsed().unwrap_or(Duration::from_millis(999999u64)).as_millis());
            log::warn!("{}",msg.as_str());
            return Err(error::driver_timeout(msg))
        }
        Ok(())
    }
}

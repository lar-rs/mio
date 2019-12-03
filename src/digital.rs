// use log::info;
// use std::thread::sleep;
// use std::time::{Duration,SystemTime};

// use async_std::prelude::*;
use async_std::io;
use async_std::fs;
// use async_std::task;
// use sysfs_gpio::{Direction,Pin};
use async_std::path::{ Path, PathBuf};
use async_trait::async_trait;



/// Digital simulation
pub struct Digital {
    path:  PathBuf,
}

impl Digital{
    pub async fn select(path:&Path) -> io::Result<Digital> {
        Ok(
            Digital{
                path: path.to_path_buf(),
            }
        )
    }
    async fn write_value(&mut self,value: bool) -> io::Result<()> {
        fs::write(self.path.join("value"),match value {
            true => b"1",
            false => b"0",
        }).await?;
        Ok(())
    }
    async fn read_value(&self) -> io::Result<bool> {
        match fs::read_to_string(self.path.join("value")).await?.as_str() {
            "1" => Ok(true),
            _ => Ok(false),
        }
    }
    pub async fn set_low(&mut self) -> io::Result<()> {
        self.write_value(false).await
    }
    pub async fn set_high(&mut self) -> io::Result<()> {
        self.write_value(true).await
    }
    pub async fn is_low(&self) -> io::Result<bool> {
        self.read_value().await
    }
    pub async fn is_high(&self) -> io::Result<bool>{
        self.read_value().await
    }
    pub async fn toggle(&mut self) -> io::Result<bool> {
        let value = self.read_value().await?;
        self.write_value(!value).await?;
        Ok(!value)

    }
}

// pub async fn select(path:&Path)->io::Result<Digital> {
   
// use log::info;
// use std::thread::sleep;
// use std::time::{Duration,SystemTime};
use std::str::FromStr;

// use async_std::prelude::*;
// use async_std::io;
use async_std::fs;
// use async_std::task;
// use sysfs_gpio::{Direction,Pin};
use async_std::io::{Result};
use async_std::path::{ Path, PathBuf};





pub struct Analog {
    path: PathBuf,
}

impl Analog {

    async fn write_value(&mut self,value: f32) -> Result<()> {
        let s = value.to_string();
        fs::write(self.path.join("value"),s.as_bytes()).await?;
        Ok(())
    }
    async fn read_value(&self) -> Result<f32> {
        let signal = f32::from_str(&fs::read_to_string(self.path.join("value")).await?).unwrap();
        Ok(signal)
    }

    pub async fn get_scaled(&self) -> Result<f32>{
        self.read_value().await
    }
    // pub async fn set_low(&mut self) -> Result<()> {
        // self.write_value(false).await
    // }
}



pub async fn connect(path:&Path) -> Result<Analog> {
    Ok(Analog{ path:path.to_path_buf() })
}

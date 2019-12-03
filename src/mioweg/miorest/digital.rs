// use log::info;
// use std::thread::sleep;
// use std::time::{Duration,SystemTime};

// use async_std::prelude::*;
// use async_std::io;
use async_std::fs;
// use async_std::task;
use sysfs_gpio::{Direction,Pin};
use async_std::io::{Result};
use async_std::path::{ Path, PathBuf};





pub struct Digital {
    path: PathBuf,
}

impl Digital {

    pub fn dir(&self) -> &Path {
        &self.path
    }
    pub fn driver(&self) -> PathBuf {
        self.path.join("value")
    }
    async fn write_value(&mut self,value: bool) -> Result<()> {
        fs::write(self.path.join("value"),match value {
        true => b"1",
        false => b"0"}).await?;
        Ok(())
    }
    async fn read_value(&self) -> Result<bool> {
        let value = match fs::read_to_string(self.path.join("value")).await?.as_str() {
            "1" => true,
            _ => false,
        };
        Ok(value)
    }

    pub async fn set_low(&mut self) -> Result<()> {
        self.write_value(false).await
    }

    pub async fn set_high(&mut self) -> Result<()> {
        self.write_value(true).await
    }


    pub async fn is_low(&self) -> Result<bool> {
        let value = self.read_value().await?;
        Ok(!value)
    }

    pub async fn is_high(&self) -> Result<bool> {
        self.read_value().await
    }

    pub async fn toggle(&mut self) -> Result<bool> {
        let value = self.read_value().await?;
        self.write_value(value).await?;
        Ok(value)
    }
}



pub async fn digital(path:&Path) -> Result<Digital> {
    Ok(Digital{ path:path.to_path_buf() })
}
pub async fn simulate(path:&Path) -> Result<()>{
    Ok(())
}

pub async fn setup_gpio_output(pin:u64) -> Result<()> {
    // let path = gpiosys().await?;
    let pin = Pin::new(pin);
    pin.export().unwrap();
    pin.set_direction(Direction::Low).unwrap();
    pin.set_value(0).unwrap();
    Ok(())
    // connect(&path).await
}



pub async fn setup_gpio_input(pin :u64) -> Result<()> {
      // let path = gpiosys().await?;
    // let path =format!("/sys/class/gpio/gpio{}",pin);
    let pin = Pin::new(pin);
    pin.export().unwrap();
    pin.set_direction(Direction::Low).unwrap();
    pin.set_value(0).unwrap();
    Ok(())
    // connect(&path).await
}

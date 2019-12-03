use super::*;
use log::info;
// use std::thread::sleep;
use std::time::{Duration,SystemTime};

use async_std::prelude::*;
use async_std::io;
use async_std::fs;
use async_std::task;
use sysfs_gpio::{Direction,Pin};
use async_std::io::Result;





pub struct ADio {
    path: PathBuf,
}

impl ADio {
    pub fn dir(&self) -> &Path {
        &self.path
    }
    pub fn driver(&self) -> PathBuf {
        self.path.join("value")
    }
}


async fn write_value(dout : &mut ADio,value: u64)-> Result<()> {
    fs::write(&dout.driver(),match value {
        true => b"1",
        false => b"0"}).await?;
}

async fn read_value(dout:&ADio,value: bool ) Result<bool> {
    match fs::read_to_string(&dout.driver()).await?.as_str() {
        "1" => Ok(true),
        _ => Ok(false),
    }
}


pub async set_low(dout: &mut DOut) -> Result<()> {
    write_value(dout,false).await
}

pub async set_hight(dout: &mut DOut) -> Result<()> {
    write_value(dout,true).await
}


pub async fn is_low(dout: &mut DOut) -> Result<bool> {
    let value = read_value(dout).await?;
    !value
}

pub async fn is_high(dout: &mut DOut) -> Result<bool> {
    read_value(dout).await
}

pub async fn toggle(dout: &mut DOut) -> Result<bool> {
    let value = read_value(dout).await?;
    wrete_value(dout,value).await?;
    Ok(value)
}




pub async setup_gpio(pin:u64) -> Result<DOut> {
    // let path = gpiosys().await?;
    let path = format!("/sys/class/gpio/gpio{}",pin);
    let pin = Pin::new(pin);
    pin.export()?;
    pin.set_direction(Direction::Low).unwrap();
    pin.set_value(0).unwrap();
    let value  = false;
    Ok(())
}


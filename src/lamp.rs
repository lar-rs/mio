/// Lamp UV
// use async_std::prelude::*;
use async_std::io;
use async_std::fs;
use async_std::path::{Path,PathBuf};
// use async_trait::async_trait;
// use std::time::{Duration,Instant};
use super::digital;
use super::{driver,HID,Driver,MioError};
 /// Lamp interface
 pub struct Lamp {
    path: PathBuf,
}
impl From<Driver> for Lamp{
    fn from(drv:Driver) -> Lamp {
        Lamp{
            path: drv.path
        }
    }
}

impl Lamp {
    pub async fn select(path: &Path) -> io::Result<Lamp> {
        Ok({
            Lamp{
                path: path.to_path_buf(),
            }
        })
    }
    pub async fn set_on(&mut self)  -> io::Result<()> {
        digital::set_high(self.path.as_path()).await?;
        Ok(())
    }
    pub async fn set_off(&mut self) -> io::Result<()> {
        digital::set_low(self.path.as_path()).await?;
        Ok(())
    }
}



pub async fn create(path:&Path) -> Result<Lamp,MioError> {
    let drv = driver::create(path,HID::Lamp).await?;
    fs::write(path.join("value"), b"0").await?;
    Ok(drv.into())
}
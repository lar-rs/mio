/// Valve I/O
// use async_std::prelude::*;
// use async_std::prelude::*;
// use async_std::stream;
use async_std::io;
use async_std::fs;
use async_std::path::{PathBuf,Path};
// use std::time::{Duration};
use super::{driver,Driver,HID,MioError};


// /// Single digital push-pull output pin
// #[async_trait]
// pub trait Valve {
//     /// Valve open
//     ///
//     /// *NOTE* the actual electrical state of the pin may not actually be low, e.g. due to external
//     /// electrical sources
//     async fn open(&mut self) -> Result<()>;

//     /// Valve close
//     ///
//     /// *NOTE* the actual electrical state of the pin may not actually be high, e.g. due to external
//     /// electrical sources
//     async fn close(&mut self) -> Result<()>;
// }


 /// Lamp simulation
 pub struct Valve {
    path: PathBuf,
}
impl From<Driver> for Valve {
    fn from(driver:Driver) -> Valve {
        Valve{
            path: driver.path
        }
    }
}

impl Valve {
    pub async fn open(&mut self) -> io::Result<()> {
        fs::write(self.path.join("state"), b"1").await?;
        Ok(())
    }
    pub async fn close(&mut self) -> io::Result<()> {
        fs::write(self.path.join("state"), b"0").await?;
        Ok(())
    }
}

pub async fn create(path:&Path) -> Result<Valve,MioError> {
    let driver = driver::create(path,HID::Valve).await?;
    fs::create_dir_all(path).await?;
    fs::write(path.join("value"), b"0").await?;
    Ok(driver.into())
}

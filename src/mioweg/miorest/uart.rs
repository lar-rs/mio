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





pub struct Uart {
    path: PathBuf,
}

impl Uart {
    pub fn dir(&self) -> &Path {
        &self.path
    }
    pub fn driver(&self) -> PathBuf {
        self.path.join("value")
    }
    pub async fn write(dout : &mut Uart,value: Vec<u8>)-> Result<()> {
        Ok()
    }
    
    pub async fn read(dout:&Uart) -> Result<Vec<u8>> {
        // match fs::read_to_string(&dout.driver()).await?.as_str() {
            // "1" => Ok(true),
            // _ => Ok(false),
        // }
        Ok(Vec::new())
    }
    // pub fn reaw(mut ) -> 
}

pub async fn serial(parh:&Path) -> Result<Uart> {

}





// pub async setup_gpio(pin:u64) -> Result<DOut> {
    // let path = gpiosys().await?;
    // let path = format!("/sys/class/gpio/gpio{}",pin);
    // let pin = Pin::new(pin);
    // pin.export()?;
    // pin.set_direction(Direction::Low).unwrap();
    // pin.set_value(0).unwrap();
    // let value  = false;
    // Ok(())
// }
// 

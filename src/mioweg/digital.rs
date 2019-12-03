// use log::info;
// use std::thread::sleep;
// use std::time::{Duration,SystemTime};

// use async_std::prelude::*;
// use async_std::io;
// use async_std::fs;
// use async_std::task;
// use sysfs_gpio::{Direction,Pin};
use async_std::io::{Result};
// use async_std::path::{ Path, PathBuf};
use async_trait::async_trait;




#[async_trait]
pub trait Digital {
    async fn set_low(&mut self) -> Result<()>;
    async fn set_high(&mut self) -> Result<()>;
    async fn is_low(&self) -> Result<bool>;
    async fn is_high(&self) -> Result<bool>;
    async fn toggle(&mut self) -> Result<bool> ;
}




// #[cfg(feature = "mosk")]
pub mod mosk {
    use async_std::io::Result;
    use async_trait::async_trait;
    // use async_std::prelude::*;
    // use async_std::stream;
    // use std::time::{Duration};

    /// Digital simulation
    pub struct Digital {
        value: bool,
    }

    impl Digital {
        async fn write_value(&mut self,value: bool) -> Result<()> {
            self.value = value;
            Ok(())
        }
        async fn read_value(&self) -> Result<bool> {
            Ok(self.value)
        }
    }
    #[async_trait]
    impl super::Digital for Digital {
        
        async fn set_low(&mut self) -> Result<()> {
            self.write_value(false).await
        }
    
        async fn set_high(&mut self) -> Result<()> {
            self.write_value(true).await
        }
    
    
        async fn is_low(&self) -> Result<bool> {
            let value = self.read_value().await?;
            Ok(!value)
        }
    
        async fn is_high(&self) -> Result<bool> {
            self.read_value().await
        }
    
        async fn toggle(&mut self) -> Result<bool> {
            let value = self.read_value().await?;
            self.write_value(value).await?;
            Ok(value)
        }
    }
}

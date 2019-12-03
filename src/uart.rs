use async_std::io::{Result};
// use async_std::path::{ Path, PathBuf};
use async_trait::async_trait;




#[async_trait]
pub trait Uart {
    /// read data. 
    async fn read(&mut self) -> Result<Vec<u8>>;
    /// write data. 
    async fn write(&mut self,bytes:Vec<u8>) -> Result<()>;
    /// change bautrate
    async fn bautrate(&mut self, baut: u32) -> Result<()>;
}




// #[cfg(feature = "mosk")]
pub mod mosk {
    use async_std::io::Result;
    use async_trait::async_trait;
    // use async_std::prelude::*;
    // use async_std::stream;
    // use std::time::{Duration};

    /// Digital simulation
    pub struct Uart {
        bytes:Vec<u8>,
    }

    #[async_trait]
    impl super::Uart for Uart {
        async fn read(&mut self) -> Result<Vec<u8>> {
            Ok(self.bytes.clone())
        }
    
        async fn write(&mut self,data:Vec<u8>) -> Result<()> {
            Ok(())
        }
        async fn bautrate(&mut self, baut: u32) -> Result<()> {
            Ok(())
        }

    }
}
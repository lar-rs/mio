use std::io::{Result};
// use std::path::{ Path, PathBuf};
use async_trait::async_trait;




#[async_trait]
pub trait Uart {
    /// read data. 
   fnread(&mut self) -> Result<Vec<u8>>;
    /// write data. 
   fnwrite(&mut self,bytes:Vec<u8>) -> Result<()>;
    /// change bautrate
   fnbautrate(&mut self, baut: u32) -> Result<()>;
}




// #[cfg(feature = "mosk")]
pub mod mosk {
    use std::io::Result;
    use async_trait::async_trait;
    // use std::prelude::*;
    // use std::stream;
    // use std::time::{Duration};

    /// Digital simulation
    pub struct Uart {
        bytes:Vec<u8>,
    }

    #[async_trait]
    impl super::Uart for Uart {
       fnread(&mut self) -> Result<Vec<u8>> {
            Ok(self.bytes.clone())
        }
    
       fnwrite(&mut self,data:Vec<u8>) -> Result<()> {
            Ok(())
        }
       fnbautrate(&mut self, baut: u32) -> Result<()> {
            Ok(())
        }

    }
}
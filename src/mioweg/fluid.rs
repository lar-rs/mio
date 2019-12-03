use async_std::io::{Result};
// use async_std::path::{ Path, PathBuf};
use async_trait::async_trait;




#[async_trait]
pub trait Fluid {
    async fn is_low(&self) -> Result<bool>;
    async fn is_high(&self) -> Result<bool>;
}




// #[cfg(feature = "mosk")]
pub mod mosk {
    use async_std::io::Result;
    use async_trait::async_trait;
    // use async_std::prelude::*;
    // use async_std::stream;
    // use std::time::{Duration};

    /// Digital simulation
    pub struct Fluid {
        value: bool,
    }

    #[async_trait]
    impl super::Fluid for Fluid {
        
        async fn is_low(&self) -> Result<bool> {
            Ok(!value)
        }
    
        async fn is_high(&self) -> Result<bool> {
            Ok(self.value)
        }
    }
}

/// Monitor hardware caontrol system.
// use super::setting;
use async_std::prelude::*;
use async_std::fs;
use async_std::path::{ Path, PathBuf};
use async_std::stream;
use async_std::io::Result;

// use serde_json::from_str;
// use serde::{Deserialize, Serialize};

// use super::digital::{self, Digital};



pub struct Fluid {
    path: PathBuf,
    din: Digital,
}
impl Fluid {
    pub async fn is_high(&self) -> Result<bool> {
        self.dout.is_high().await
    }

    pub async fn is_low(&mut self ) -> Result<()> {
        self.dout.is_low().await
    }
}


pub async fn check(path:&Path) -> bool {
   path.is_dir().await && path.join("lamp").exists().await 
}

pub async fn connect(path : &Path) -> Result<Fluid> {
    let model = model(path).await?;
    let dout  = digital::connect(&path.join("din")).await?;
    Ok(Fluid{
        path:path.to_path_buf(),
        dout: dout,
        model: model,
    })
}

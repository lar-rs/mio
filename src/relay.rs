/// Relay
// use async_std::prelude::*;
use async_std::fs;
// use async_trait::async_trait;
use async_std::path::PathBuf;
use super::{digital,MioError};

pub const BOOLEXP: &'static str = "boolexp";


 /// Relay simulation
 pub struct Relay {
    path: PathBuf,
}
impl Relay {
    pub async fn is_open(&self) -> Result<bool,MioError> {
        digital::is_high(self.path.as_path()).await
    }
    pub async fn open(&mut self)  -> Result<(),MioError> {
        digital::set_high(self.path.as_path()).await
    }
    pub async fn close(&mut self) -> Result<(),MioError> {
        digital::set_low(self.path.as_path()).await
    }
    pub async fn boolexp(&self) -> Result<String,MioError> {
        let boolexp = fs::read_to_string(self.path.join(BOOLEXP)).await?;
        Ok(boolexp) 
    }
}
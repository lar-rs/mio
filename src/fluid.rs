/// Fluid sensor interface
use async_std::path::{ PathBuf};
// use async_trait::async_trait;
use super::{MioError,digital};




pub struct Fluid {
    path:PathBuf,
}


impl Fluid {
    pub async fn empty(&self) -> Result<bool,MioError> {
        digital::is_high(self.path.as_path()).await
    }

    pub async fn full(&self ) -> Result<bool,MioError> {
        digital::is_low(self.path.as_path()).await
    }
}



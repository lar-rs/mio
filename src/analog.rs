///
/// Analog IN OUT
///
///
///
// use async_std::io;
use async_std::path::{PathBuf,Path};
use super::{driver,MioError,HID};
// use async_std::prelude::*;
// use async_trait::async_trait;




pub struct ADC {
    path: PathBuf,
}


impl ADC {
    /// current in mAh
    pub async fn current(&self) -> Result<f32,MioError> {
        Ok(0.0)
    }
    /// digital retros
    pub async fn digital(&self) -> Result<u32,MioError> {
        Ok(0)
    }

}

pub async fn simulation(path:&Path) -> Result<ADC,MioError> {
    Ok(ADC{path:driver::create(path,HID::Airflow).await?.into()})
}
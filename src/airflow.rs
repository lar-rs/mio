// use std::fmt;
use serde::{Deserialize, Serialize};

use super::{driver,MioError,HID};
use async_std::fs;
// use async_std::stream::Stream;
use async_std::path::{PathBuf,Path};
// use async_std::stream;
// use std::time::Duration;
// use async_std::prelude::*;
pub const INPUT:  &'static str = "input";
pub const OUTPUT: &'static str = "output";
pub const CORELATION: &'static str = "corelation";
pub const REFERENCE: &'static str = "reference";
pub const DEVIATION: &'static str = "deviation";

/// für 0..60:   0.230197;
// static AC:[f32;7] = [0.003836617, -0.06027397,0.3727283,-1.1430475,1.83842,-1.4032,0.39159];
// für 0..60:  -3.616438;
// static A5:f32 =
// für 0..60:  22.36370;
// static A4:f32 = 
// für 0..60: -68.58285;
// static A3:f32 = ;
// für 0..60: 110.3052;
// static A2:f32 = ;
// für 0..60: -84.19201;
// static A1:f32 = ;
// für 0..60:  23.49542;
// static A0:f32 = ;


#[derive(Serialize,Default, Deserialize, Clone, Debug)]
pub struct Config {
    pub correlation:          f32,
    pub soll_value:           f32,
    pub input_deviation:      f32,
    pub output_deviation:     f32,
    pub monitorin_interval:   u64,
}



/// Hardware airflow sensor.
pub struct Airflow {
    path : PathBuf,
}

impl Airflow {

    /// Returns volume rate on input reported by sensor.
    pub async fn input(&self) -> Result<f32,MioError> {
        let value = fs::read_to_string(self.path.join(INPUT)).await?.parse::<f32>()?;
        Ok(value)
    }
    /// Returns volume rate on input reported by sensor.
    pub async fn output(&self) -> Result<f32,MioError> {
        let value = fs::read_to_string(self.path.join(OUTPUT)).await?.parse::<f32>()?;
        Ok(value)
    }
    /// corelation value
    pub async fn corelation(&self) -> Result<f32,MioError> {
        let value = fs::read_to_string(self.path.join(CORELATION)).await?.parse::<f32>()?;
        Ok(value)
    }
    /// corelation value
    pub async fn reference(&self) -> Result<f32,MioError> {
        let value = fs::read_to_string(self.path.join(REFERENCE)).await?.parse::<f32>()?;
        Ok(value)
    }
    /// maximale erlaubte abweichung in procent 
    pub async fn deviation(&self) -> Result<f32,MioError> {
        let value = fs::read_to_string(self.path.join(DEVIATION)).await?.parse::<f32>()?;
        Ok(value)
    }
}


// pub async fn airflow() -> Result<Airflow,MioError> {
    // let path = super::miofs().await?.join("airflow");
    // Ok(Airflow{path}) 
// }



pub async fn create(path:&Path) -> Result<Airflow,MioError> {
    driver::create(path,HID::Airflow).await?;
    fs::write(path.join(INPUT), b"0").await?;
    fs::write(path.join(OUTPUT), b"0").await?;
    fs::write(path.join(CORELATION), b"1.0").await?;
    fs::write(path.join(REFERENCE), b"30.0").await?;
    fs::write(path.join(DEVIATION), b"5.0").await?;
    let path = path.to_path_buf();
    Ok(Airflow{path})
}
// use std::fmt;
// pub use uom::si::f32::{Ratio,MassRate,ElectricCurrent};
use super::setting;
use super::signal;
use async_std::prelude::*;
use async_std::fs;
use async_std::path::{ Path, PathBuf};
use async_std::stream;

use serde_json::from_str;
use serde::{Deserialize, Serialize};




/// Hardware airflow sensor.
pub struct NdirSensor {
   
}

impl NdirSensor {
   
   

    /// Returns current presure reported by sensor.
    pub fn current(&self) -> f32 {
        self.current
    }
}





// pub async
// pub async fn set_addres(pa)

// pub async fn simulation(path: PathBuf) {
// }

